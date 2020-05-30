//! 这里定义了所有系统调用
#![feature(concat_idents)]

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
    // pub fn orig_open(filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long;
    // pub fn orig_openat(f : u64, filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long;
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

#[inline(always)]
fn return_result(res : KernelResult<i32>) -> i64 {
    match res {
        Ok(i) => i as i64,
        Err(e) => e.to_kernel_errno() as i64,
    }
}

#[inline(always)]
fn get_pid() -> i32 {
    return unsafe{orig_getpid() as i32}
}

#[inline(always)]
fn i32_syscall<F>(f : F) -> Option<i64>
    where F : Fn(&mut Kernel) -> KernelResult<i32>
{
    let container = Container::get_container();
    if !container.try_set_current(get_pid()) {return None;}
    info!("{}", get_pid());
    Some(return_result(
        container.runk_mut(f).unwrap()
    ))
}
macro_rules! kernel_syscall{
    ($orig:ident, $safe:ident,$name:ident, $p:expr, $($arg:ident, $type:ty),*) => { 
        extern "C" { 
            pub fn $orig($($arg : $type,)*) -> i64; 
        }
        #[inline(always)]
        pub fn $safe($($arg : $type,)*) -> i64 {
            protect_fs_run(|| unsafe{$orig($($arg),*)})
        }
        pub extern "C" fn $name($($arg : $type,)*) -> i64 {
            let ret = i32_syscall( 
                $p 
            );
            if let Some(r) = ret {r } 
            else {
                unsafe {$orig($($arg),*)}
            }
        }
    }
}

kernel_syscall!(
    orig_open, safe_open, rvisor_open,{
        |k| k.open(filename, flags, mode)
    }, filename, * const u8, flags  , c_int, mode , bindings::umode_t
);
kernel_syscall!(
    orig_openat, safe_openat, rvisor_openat, {
        |k| k.open(filename, flags, mode)
    }, f, u64, filename, * const u8, flags ,  c_int, mode , bindings::umode_t
);

kernel_syscall!(
    orig_execve, safe_execve, rvisor_execve, {
        |k| k.execve(filename, argv, envp)
    }, filename, * const u8, argv , u64, envp, u64
);

kernel_syscall!(
    orig_stat, safe_stat, rvisor_stat, {
        |k| k.stat(filename, ptr)
    }, filename, u64, ptr, u64
);

kernel_syscall!(
    orig_lstat, safe_lstat, rvisor_lstat, {
        |k| k.lstat(filename, ptr)
    }, filename, u64, ptr, u64
);

kernel_syscall!(
    orig_getcwd, safe_getcwd, rvisor_getcwd, {
        |k| k.getcwd(buf, size)
    }, buf , u64, size, u64
);

kernel_syscall!(
    orig_chdir, safe_chdir, rvisor_chdir, {
        |k| k.chdir(filename)
    }, filename, u64
);

macro_rules! normal_syscall{
    ($orig:ident, $safe:ident, $name:ident, $p:expr, $($arg:ident, $type:ty),*) => {
        extern "C" {
            pub fn $orig($($arg : $type,)*) -> i64;
        }
        #[inline(always)]
        pub fn $safe($($arg : $type,)*) -> i64 {
            protect_fs_run(|| unsafe{$orig($($arg),*)})
        }
        pub extern "C" fn $name($($arg : $type,)*) -> i64 {
            let container = Container::get_container();
            if container.contains(get_pid()) {
                $p
            } else {
                unsafe {$orig($($arg),*)}
            }
        }
    }
}

normal_syscall!(
    orig_clone, safe_clone, rvisor_clone, {
        let fpid = get_pid();
        info!("clone: called inside container");
        let container = Container::get_container();
        let i = unsafe {orig_clone(flags, newsp, ptidptr, ctidptr, tls)};
        if i > 0 {container.clone_task(i as i32, fpid);}
        i
    } , flags, u64, newsp, u64, ptidptr, u64, ctidptr, u64, tls, u64
);

normal_syscall!(
    orig_fork, safe_fork, rvisor_fork, {
        let fpid = get_pid();
        info!("fork: called inside container");
        let container = Container::get_container();
        let i = unsafe {orig_fork()};
        if i > 0 {container.clone_task(i as i32, fpid);}
        i
    },
); 

normal_syscall!(
    orig_vfork, safe_vfork, rvisor_vfork, {
        let fpid = get_pid();
        let container = Container::get_container();
        let i = unsafe {orig_vfork()};
        if i > 0 {container.clone_task(i as i32, fpid);}
        i
    },
);

// kernel_syscall!(
//  orig_mknodat, safe_mknodat, rvisor_mknodat, {
 
//  }, dfd, u64, filename, u64, mode, u64, dev, u64
//);
 
// kernel_syscall!(
//   orig_mknod, safe_mknod, rvisor_mknod, {
 
//     }, filename, u64, mode, u64, dev, u64
// );

//  kernel_syscall!(
//     orig_mkdirat, safe_mkdirat, rvisor_mkdirat, {
        
//     }, dfd, u64, filename, u64, mode, u64
// );

//  kernel_syscall!(
//     orig_mkdir, safe_mkdir, rvisor_mkdir, {

//     }, filename, u64, mode, u64
// );

//  kernel_syscall!(
//     orig_rmdir, safe_rmdirat, rvisor_rmdirat, {

//     }, filename, u64
// );

// kernel_syscall!(
//     orig_stat, safe_stat, rvisor_stat, {

//     }, filename, u64, statbuf, u64
// );

// kernel_syscall!(
//     orig_lstat, safe_lstat, rvisor_lstat, {

//     }, filename, u64, statbuf, u64
// );