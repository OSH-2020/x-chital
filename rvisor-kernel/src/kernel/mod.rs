
use linux_kernel_module::{KernelResult, Error};
use alloc::borrow::ToOwned;
use alloc::string::String;

pub const PATH_MAX : usize = 200;



pub struct Kernel {
    rootpath : String,
}

impl Kernel {
    pub fn new(host_path : String) -> KernelResult<Kernel> {
        Ok(Kernel{
            rootpath: host_path,
        })
    }
}

pub mod fs;