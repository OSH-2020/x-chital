use linux_kernel_module as lkm;
use lkm::bindings;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

pub struct Task {
    pub pid : bindings::pid_t,
    pub cwd : Vec<String>,
}

impl Task {
    pub fn new(pid : i32) -> Task {
        Task{
            pid : pid,
            cwd : Vec::new(),
        }
    }
}