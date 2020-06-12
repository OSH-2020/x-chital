use linux_kernel_module as lkm;
use lkm::bindings;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;

use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::collections::linked_list::LinkedList;
use spin::Mutex;

use crate::kernel::fs::DEntry;

#[derive(Debug)]
pub struct Task {
    pub pid : bindings::pid_t,
    pub cwd : Vec<String>,
    pub files : BTreeMap<i32, File>,
}

#[derive(Debug, Clone)]
pub struct File {
    pub dentry : Option<Arc<Mutex<DEntry>>>, // None for same to system
    pub pos : u64,
}

impl Task { 
    pub fn new(pid : i32) -> Task {
        Task{
            pid : pid,
            cwd : Vec::new(),
            files: BTreeMap::new(),
        }
    }
    pub fn clone(&self, pid : i32) -> Task {
        Task {
            pid: pid,
            cwd: self.cwd.clone(),
            files: self.files.clone(),
        }
    }
    pub fn open_file(&mut self,fd: i32, dentry : Option<Arc<Mutex<DEntry>>>) {
        self.files.insert(fd, File{ dentry: dentry, pos:0});
    }
}