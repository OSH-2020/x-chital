
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

        filename.push(0 as char);
        Ok(protect_fs_run(||{
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

        filename.push(0 as char);
        Ok(protect_fs_run(||{
            unsafe{orig_execve(filename.as_str().as_ptr(), argv, envp) as i32}
        }))
    }

    pub fn stat(&mut self, filename: u64, ptr : u64) -> KernelResult<i32> {
        let filename = string::read_from_user(filename as u64, kernel::PATH_MAX)?;
        let mut filename = self.guest_to_host(&filename)?;
        filename.push(0 as char);
        Ok(protect_fs_run(||{
            unsafe{orig_stat(filename.as_str().as_ptr() as u64, ptr) as i32}
        }))
    }

    pub fn lstat(&mut self, filename: u64, ptr : u64) -> KernelResult<i32> {
        let filename = string::read_from_user(filename as u64, kernel::PATH_MAX)?;
        let mut filename = self.guest_to_host(&filename)?;
        filename.push(0 as char);
        Ok(protect_fs_run(||{
            unsafe{orig_lstat(filename.as_str().as_ptr() as u64, ptr) as i32}
        }))
    }

    pub fn getcwd(&self, user_ptr : u64, max_length : u64) -> KernelResult<i32> {
        let mut path = String::with_capacity(80);
        if let Some(ref cur) = self.current {
            info!("entered if let {:?}", cur);
            let data = cur.lock();
            data.get_cwd(&mut path);
        }
        info!("getcwd: path {}", path);
        string::write_to_user(user_ptr, max_length as usize, path)?;
        Ok(0)
    }

    pub fn chdir(&mut self, filename : u64) -> KernelResult<i32> {
        let filename = string::read_from_user(filename, kernel::PATH_MAX)?;
        let mut abs_filename = self.guest_to_host(&filename)?;
        info!("kernel chdir: get filename {}", filename);
        if let Some(ref mut cur) = self.current {
            let mut data = cur.lock();
            info!("chdir: cwd = {:?}", &data.cwd);
            data.chdir(&filename);
            info!("chdir: cwd = {:?}", &data.cwd);
        }
        abs_filename.push(0 as char);
        Ok(protect_fs_run(||{
            unsafe{orig_chdir(abs_filename.as_str().as_ptr() as u64) as i32}
        }))
    }
}
