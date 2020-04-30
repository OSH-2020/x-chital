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
}

fn safe_replace_syscall(sysnum : c_uint, f_ptr : *const()) {
    unsafe{
        let i = replace_syscall(sysnum, f_ptr);
        if i == -1 {
            panic!("replace_syscall failed!");
        }
    }
}

pub fn init() {
    unsafe{
        if replace_init() == -1 {
            panic!("replace_init failed!");
        }
    }
    safe_replace_syscall(bindings::__NR_open, syscall::rvisor_open as *const());
}

pub fn cleanup() {
    unsafe{
        if replace_clear() == -1 {
            panic!("replace_clear failed!");
        }
    }
}