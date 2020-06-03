
use linux_kernel_module::bindings;

use spin::RwLock as RcuLock;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::sync::Weak;
use alloc::collections::linked_list::LinkedList;
use spin::Mutex;

pub mod path;
pub mod open;


struct INode {
    ino : u64,
    mode : u64,
    uid : u64,
    gid : u64,
    ismounted : u64,
    mapped_to : String, // map to file at orig system
}

impl INode {}

struct DEntry {
    name : String,
    inode : Arc<Mutex<INode>>,
    parent : Weak<Mutex<DEntry>>,
    child : LinkedList<Arc<Mutex<DEntry>>>,
}

impl DEntry {
    pub fn new(name : String, parent : Weak<Mutex<DEntry>>, ino : u64, mode : u64, uid : u64, gid : u64) -> DEntry {
        DEntry {
            name: name,
            inode: Arc::new(
                Mutex::new(INode{
                    ino: ino,
                    mode: mode,
                    uid: uid,
                    gid: gid,
                    ismounted: false,
                })
            ),
            parent: parent,
            child: LinkedList::new(),
        }
    }

    pub fn createChild(name : String, parent : Weak<Mutex<DEntry>>, ino : u64, mode : u64, uid : u64, gid : u64) {
        
        self.child.push_back(
            Arc::new(
                Mutex::new(
                    new(name, parent, ino, mode, uid, gid)
                )
            )
        )
    }

}