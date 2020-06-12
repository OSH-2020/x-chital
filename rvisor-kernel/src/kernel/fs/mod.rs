use linux_kernel_module::{KernelResult, Error, bindings};
use crate::kernel::Kernel;
use spin::RwLock as RcuLock;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::collections::linked_list::LinkedList;
use spin::Mutex;
use alloc::rc::Rc;
pub mod path;
pub mod open;

#[derive(Debug)]
pub struct INode {
    pub ino : u64,
    pub mode : u64,
    pub uid : u64,
    pub gid : u64,
}

trait FileOperations {
    fn open(&self, k : Kernel, path : &Arc<Mutex<DEntry>>) -> KernelResult<()> {
        Err(Error::EINVAL)
    }
}

#[derive(Debug)]
pub struct DEntry {
    pub name : String,
    pub inode : INode,
    pub parent : Weak<Mutex<DEntry>>,
    pub child : LinkedList<Arc<Mutex<DEntry>>>,
    pub fops : Option<Rc<dyn FileOperations>>,
}

use core::fmt::Debug;
impl Debug for dyn FileOperations {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "fops")
    }
}

impl DEntry {
    pub fn new(name : String, parent : Weak<Mutex<DEntry>>, ino : u64, mode : u64, uid : u64, gid : u64) -> DEntry {
        DEntry {
            name: name,
            inode: INode{
                ino: ino,
                mode: mode,
                uid: uid,
                gid: gid,
            },
            parent: parent,
            child: LinkedList::new(),
            fops: None,
        }
    }
}

pub fn createChild(name : String, parent : &Arc<Mutex<DEntry>>, ino : u64, mode : u64, uid : u64, gid : u64) {
    parent.lock().child.push_back(
        Arc::new(
            Mutex::new(
                DEntry::new(name, Arc::downgrade(parent), ino, mode, uid, gid)
            )
        )
    );
}