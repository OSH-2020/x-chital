//! 
//! This is a modified copy of github repo: [linux-kernel-module-rust](https://github.com/fishinabarrel/linux-kernel-module-rust).
//! 
//! We have to modify it to run our rvisor-kernel.
//! 
//! ### How linux-kernel-module-rust build the kernel module?
//! 
//! #### `build.rs`
//!  
//! `build.rs` is rust build script, it will be run in the target/ folder.
//! 
//! It use crate `cc` to build c files. like this:
//! 
//! ```
//!     let mut builder = cc::Build::new();
//!     builder.compiler(env::var("CLANG").unwrap_or("clang".to_string()));
//!     builder.target(&target);
//!     builder.warnings(false);
//!     builder.file("src/helpers.c");
//!     for arg in shlex::split(std::str::from_utf8(&output.stdout).unwrap()).unwrap() {
//!         builder.flag(&arg);
//!     }
//!     builder.compile("helpers");
//! ```
//! 
//! #### kernel-cflags-finder
//! 
//! kernel-cflags-finder search for kernel build cflags.
//! 
//! As we don't know kernel build reference /lib/modules/$(uname -r)/build exactly (different linux kernel have different build script) , we need to search them.
//! 
//! then we can add the cflag to rust `cc::Build` as we saw before.
//! 
//! If you meet some problem in kernel-cflags-finder, you should test whether you could build a kernel module with your `clang`.
//! 
//! #### Makefile & Kbuild
//! 
//! We build kernel module by Makefile, Makefile will use /lib/modules/$(uname -r)/build (we known as $(KDIR)). Then Kbuild file will be triggerd.
//! 
//! Kbuild should contains the infomation of where the compiled library (writen in rust) is, and how to convert them to object file.


#![no_std]
#![feature(allocator_api, alloc_error_handler, const_fn, const_raw_ptr_deref)]

extern crate alloc;

use core::panic::PanicInfo;

mod allocator;
pub mod bindings;
pub mod c_types;
pub mod chrdev;
mod error;
pub mod file_operations;
pub mod filesystem;
pub mod printk;
pub mod random;
pub mod sysctl;
mod types;
pub mod user_ptr;

pub use crate::error::{Error, KernelResult};
pub use crate::types::{CStr, Mode};

/// Declares the entrypoint for a kernel module. The first argument should be a type which
/// implements the [`KernelModule`] trait. Also accepts various forms of kernel metadata.
///
/// Example:
/// ```rust,no_run
/// use linux_kernel_module;
/// struct MyKernelModule;
/// impl linux_kernel_module::KernelModule for MyKernelModule {
///     fn init() -> linux_kernel_module::KernelResult<Self> {
///         Ok(MyKernelModule)
///     }
/// }
///
/// linux_kernel_module::kernel_module!(
///     MyKernelModule,
///     author: "Fish in a Barrel Contributors",
///     description: "My very own kernel module!",
///     license: "GPL"
/// );
#[macro_export]
macro_rules! kernel_module {
    ($module:ty, $($name:ident : $value:expr),*) => {
        static mut __MOD: Option<$module> = None;
        #[no_mangle]
        pub extern "C" fn init_module() -> $crate::c_types::c_int {
            match <$module as $crate::KernelModule>::init() {
                Ok(m) => {
                    unsafe {
                        __MOD = Some(m);
                    }
                    return 0;
                }
                Err(e) => {
                    return e.to_kernel_errno();
                }
            }
        }

        #[no_mangle]
        pub extern "C" fn cleanup_module() {
            unsafe {
                // Invokes drop() on __MOD, which should be used for cleanup.
                __MOD = None;
            }
        }

        $(
            $crate::kernel_module!(@attribute $name, $value);
        )*
    };

    (@attribute $name:ident, $value:expr) => {
        #[link_section = ".modinfo"]
        #[allow(non_upper_case_globals)]
        // TODO: Generate a name the same way the kernel's `__MODULE_INFO` does.
        // TODO: This needs to be a `[u8; _]`, since the kernel defines this as a  `const char []`.
        // See https://github.com/rust-lang/rfcs/pull/2545
        pub static $name: &'static [u8] = concat!(stringify!($name), "=", $value, '\0').as_bytes();
    };
}

/// KernelModule is the top level entrypoint to implementing a kernel module. Your kernel module
/// should implement the `init` method on it, which maps to the `module_init` macro in Linux C API.
/// You can use this method to do whatever setup or registration your module should do. For any
/// teardown or cleanup operations, your type may implement [`Drop`].
///
/// [`Drop`]: https://doc.rust-lang.org/stable/core/ops/trait.Drop.html
pub trait KernelModule: Sized + Sync {
    fn init() -> KernelResult<Self>;
}

extern "C" {
    fn bug_helper() -> !;
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{:?}", _info);
    unsafe {
        bug_helper();
    }
}

#[global_allocator]
static ALLOCATOR: allocator::KernelAllocator = allocator::KernelAllocator;
