
//! rVisor kernel implementation

#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module;
use linux_kernel_module::c_types::*;
use linux_kernel_module::println;

/// logger for this crate
pub mod log;

extern "C" {
    /// init syscall replacer (find where the syscall is)
    fn replace_init() -> c_int; 
    /// replace the syscall (here we replace open for test)
    fn replace_syscall() -> c_int;
    /// recover the replace
    fn replace_clear() -> c_int;
}

/// it contains kernel varible
/// 
/// I'll test if every static varible should inside this struct.
struct RVisorModule {}

/// rvisor open syscall (this function will be used in syscall.c)
/// 
/// `umode_t` is definde as `unsigned short`, you can see at [umode_t](https://elixir.bootlin.com/linux/v4.6/ident/umode_t)
/// it will evoke a info! now (useless)
#[no_mangle]
pub extern "C" fn rvisor_open(filename: *const u8, flags : c_int, mode : c_ushort) -> c_long {
    info!("open called");
    return 0; 
}
/// impl kernel init function here
impl linux_kernel_module::KernelModule for RVisorModule {
    /// kernel init function
    fn init() -> linux_kernel_module::KernelResult<Self> {
        info!(": module init");
        unsafe {
            replace_init();
            replace_syscall();
        }

        Ok(RVisorModule{})
    }
}

/// impl kernel clean up function here
impl Drop for RVisorModule {
    /// kernel clean up function
    fn drop(&mut self) {
        unsafe{
            replace_clear();
        }
        info!(": module clear");
    }
}

linux_kernel_module::kernel_module!(
    RVisorModule,
    author: "dnailz@chital",
    description: "kernel version of rvisor",
    license: "GPL"
);
