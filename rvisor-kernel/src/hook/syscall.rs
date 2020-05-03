//! 这里定义了所有系统调用

use linux_kernel_module::bindings;
use linux_kernel_module::KernelResult;
use linux_kernel_module::Error;
use linux_kernel_module::c_types::*;
use crate::container::Container;
use crate::string;
use crate::kernel;
use crate::kernel::Kernel;
// use crate::kernel::fs::open::Kernel;

extern "C" {
    pub fn orig_open(filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long;
    pub fn orig_openat(f : u64, filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long;
    pub fn orig_getpid() -> c_long;

    fn protect_fs() -> bindings::mm_segment_t;
    fn release_fs(oldfs : bindings::mm_segment_t);
}

pub fn protect_fs_run<T, F: Fn()->T> (func : F) -> T{
    let oldfs = unsafe{protect_fs()};
    let ret = func();
    unsafe{ release_fs(oldfs);}
    ret
}

fn return_result(res : KernelResult<i32>) -> i64 {
    match res {
        Ok(i) => i as i64,
        Err(e) => e.to_kernel_errno() as i64,
    }
}

fn get_pid() -> i32 {
    return unsafe{orig_getpid() as i32}
}

/// 重写的open系统调用。
pub extern "C" fn rvisor_open(filename: * const u8, flags : c_int, mode : bindings::umode_t) -> i64 {
    let container = Container::get_container();
    info!("{}" , get_pid());
    info!("{:?}" , container.tasks_tgid);
    if container.contains(get_pid() as i32) {
        return_result(
            container.runk_mut(|k|{
                k.open(filename, flags, mode)
            }).unwrap()
        )
    } else {
        unsafe {
            orig_open(filename, flags, mode)
        }
    }
}

pub extern "C" fn rvisor_openat(f : u64, filename: * const u8, flags : c_int, mode : bindings::umode_t) -> i64 {
    let container = Container::get_container();
    if container.contains(get_pid()) {
        return_result(
            container.runk_mut(|k|{
                k.open(filename, flags, mode)
            }).unwrap()
        )
    } else {
        unsafe {
            orig_openat(f, filename, flags, mode)
        }
    }
}