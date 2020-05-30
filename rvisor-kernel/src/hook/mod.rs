//! 系统调用挂钩的模块
//! 

use linux_kernel_module::bindings;
use linux_kernel_module::c_types::*;
use linux_kernel_module::println;

pub mod syscall;

extern "C" {
    /// init syscall replacer (find where the syscall is)
    fn replace_init() -> c_int; 
    /// replace the syscall (here we replace open for test)
    fn replace_syscall(sysnum : c_uint, f_ptr : *const()) -> c_int;
    /// recover the replace
    fn replace_clear() -> c_int;

    pub fn user_max() -> u64;
}
/// replace_syscall 的安全包装
fn safe_replace_syscall(sysnum : c_uint, f_ptr : *const()) {
    unsafe{
        let i = replace_syscall(sysnum, f_ptr);
        if i == -1 {
            panic!("replace_syscall failed!");
        }
    }
}

/// init的时候调用
pub fn init() {
    unsafe{
        if replace_init() == -1 {
            panic!("replace_init failed!");
        }
    }
    safe_replace_syscall(bindings::__NR_open, syscall::rvisor_open as *const());
    safe_replace_syscall(bindings::__NR_openat, syscall::rvisor_openat as *const());
    safe_replace_syscall(bindings::__NR_execve, syscall::rvisor_execve as *const());
    safe_replace_syscall(bindings::__NR_clone, syscall::rvisor_clone as *const());
    safe_replace_syscall(bindings::__NR_fork, syscall::rvisor_fork as *const());
    safe_replace_syscall(bindings::__NR_vfork, syscall::rvisor_vfork as *const());
    safe_replace_syscall(bindings::__NR_chdir, syscall::rvisor_chdir as *const());
    safe_replace_syscall(bindings::__NR_getcwd, syscall::rvisor_getcwd as *const());
    safe_replace_syscall(bindings::__NR_stat, syscall::rvisor_stat as *const());
    safe_replace_syscall(bindings::__NR_lstat, syscall::rvisor_lstat as *const());
}

/// 退出的时候调用
pub fn cleanup() {
    unsafe{
        if replace_clear() == -1 {
            panic!("replace_clear failed!");
        }
    }
}

