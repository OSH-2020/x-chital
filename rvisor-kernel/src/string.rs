use linux_kernel_module::KernelResult;
use linux_kernel_module::bindings;
use linux_kernel_module::user_ptr::UserSlicePtr;
use linux_kernel_module::Error;
use crate::container::Container;

use alloc::borrow::ToOwned;
use alloc::string::String;

extern "C" {
    fn strncpy_from_user2(a: *mut u8, b: *const u8, max : u64) -> i32;
}

pub fn read_from_user(user_ptr : u64, max_length : usize) -> KernelResult<String> {
    info!("read_from_user");
    let mut ret = String::from("                                                                                                                             ");
    unsafe {
        let slice = ret.as_mut_str();
        let i = strncpy_from_user2(slice.as_mut_ptr(), user_ptr as *const u8, max_length as u64);
        ret.truncate(i as usize);
        if i < 0 {
            info!("read_from_user: error {}", i);
            return Err(Error::from_kernel_errno(i));
        }
    }
    info!("read_from_user: {}", ret);
    Ok(ret)
}

pub fn write_to_user(user_ptr : u64, max_length : usize, mut src : String) -> KernelResult<()> {
    src.push(0 as char);
    let uptr = UserSlicePtr::new_ptr(user_ptr, max_length)?;
    uptr.write_all(src.as_bytes())?;
    Ok(())
}

