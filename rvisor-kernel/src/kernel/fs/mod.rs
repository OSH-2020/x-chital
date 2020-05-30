
use linux_kernel_module::bindings;

use spin::RwLock as RcuLock;
use alloc::str::String;
use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::collections::linked_list::LinkedList;
use spin::Mutex;

pub mod path;
pub mod open;


struct INode {
    ino : u64,
    dentry : Weak<Mutex<DEntry>>,
    mode : u64,
    uid : u64,
    gid : u64,
}

impl INode {
    
}

struct DEntry {
    name : String,
    inode : Arc<Mutex<INode>>,
    parent : Weak<Mutex<DEntry>>,
    child : LinkedList<Arc<Mutex<DEntry>>>,
}

impl DEntry {
    pub fn new(name : String, parent : Weak<Mutex<DEntry>>) -> DEntry {
        DEntry {
            name : name,
            inode : 
        }
    }
}