
//! rVisor kernel implementation
//! 
//! #### How to Build
//! 
//! install linux headers
//! 
//! ```sh
//! $ sudo apt install clang-9 linux-headers-$(uname -r)
//! ```
//! 
//! use clang-9 as your default compiler
//! 
//! ```sh
//! $ sudo mv $(which clang-9) /usr/bin/clang
//! ```
//! 
//! then `make ins` to build the module.
//! 
//! #### How to Test
//! 
//! Thus open syscall is rarely use, you should compile & run test/open.c to test it.
//! 
//! the run `dmesg | tail -10`,  the result will be shown.
//! 

#![no_std]

extern crate alloc;

use alloc::borrow::ToOwned;
use alloc::string::String;

use linux_kernel_module;
use linux_kernel_module::c_types::*;
use linux_kernel_module::println;
use linux_kernel_module::chrdev;
use linux_kernel_module::cstr;

/// logger for this crate
#[macro_use]
pub mod log;
mod hook;
mod iodev;


/// it contains kernel varible
/// 
/// I'll test if every static varible should inside this struct.
struct RVisorModule {
    /// chrdev_registration: 为linux增加一个主设备号，可以用mknod获得设备文件。
    chrdev_registration: chrdev::Registration,
}

/// impl kernel init function here
impl linux_kernel_module::KernelModule for RVisorModule {
    /// kernel init function
    fn init() -> linux_kernel_module::KernelResult<Self> {
        info!(": module init");
        hook::init();
        
        // 登记设备名，和设备文件struct
        let _chrdev_registration =
                chrdev::builder(cstr!("rvisor"), 0..1)?
                    .register_device::<iodev::IoDeviceFile>()
                    .build()?;
        Ok(RVisorModule{
            chrdev_registration: _chrdev_registration,
        })
    }
}

/// impl kernel clean up function here
impl Drop for RVisorModule {
    /// kernel clean up function 
    fn drop(&mut self) {
        hook::cleanup();
        info!(": module clear");
    }
}

linux_kernel_module::kernel_module!(
    RVisorModule,
    author: "dnailz@chital",
    description: "kernel version of rvisor",
    lisense : "GPL"
);
