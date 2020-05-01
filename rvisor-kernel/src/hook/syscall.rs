use linux_kernel_module::bindings;
use linux_kernel_module::c_types::*;


extern "C" {
    fn orig_open(filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long;
    fn orig_getpid() -> c_long;
}

/// 重写的open系统调用。
pub extern "C" fn rvisor_open(filename: * const u8, flags : c_int, mode : bindings::umode_t) -> c_long {
    unsafe{
        let i = orig_getpid();
        info!(": opencalled userpid={}", i);
    }
    return -1;
}
