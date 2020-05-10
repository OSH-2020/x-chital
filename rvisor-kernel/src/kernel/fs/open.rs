
use crate::kernel::Kernel;
// use crate::kernel::fs::path::Kernel;
use crate::hook::syscall::*;

use linux_kernel_module::KernelResult;
use linux_kernel_module::Error;
use linux_kernel_module::bindings;

use alloc::borrow::ToOwned;
use alloc::string::String;

use crate::kernel;
use crate::string;



impl Kernel {
    pub fn open(&mut self, filename: * const u8, flags : i32, mode : bindings::umode_t) -> KernelResult<i32> {
        let filename = string::read_from_user(filename as u64, kernel::PATH_MAX)?;
        info!("kernel open: get filename {}", filename);
        let mut filename = self.guest_to_host(&filename)?;
        info!("kernel open: traslated {}", filename);
        
        Ok(protect_fs_run(||{
            filename.push(0 as char);
            let ret = unsafe{orig_open(filename.as_str().as_ptr(), flags, mode) as i32};
            info!("orig_open: return {}", ret);
            ret
        }))
    }

    pub fn execve(&mut self, filename: * const u8, argv : u64, envp : u64) -> KernelResult<i32> {
        let filename = string::read_from_user(filename as u64, kernel::PATH_MAX)?;
        info!("kernel open: get filename {}", filename);
        let mut filename = self.guest_to_host(&filename)?;
        info!("kernel open: traslated {}", filename);
        Ok(protect_fs_run(||{
            filename.push(0 as char);
            unsafe{orig_execve(filename.as_str().as_ptr(), argv, envp) as i32}
        }))
    }
}
