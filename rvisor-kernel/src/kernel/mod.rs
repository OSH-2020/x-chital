
use linux_kernel_module::{KernelResult, Error, bindings};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use alloc::sync::Arc;
use spin::Mutex;

use crate::kernel::fs::*;
pub const PATH_MAX : usize = 200;

pub mod task;
use task::Task;

pub struct Kernel {
    rootpath : String,
    fs_map: BTreeMap<String,  Arc<Mutex<DEntry>> >,
    tasks : BTreeMap<i32, Arc<Mutex<Task>> >,
    current : Option<Arc<Mutex<Task>>>,
}

impl Kernel {
    pub fn new(host_path : String) -> KernelResult<Kernel> {
        Ok(Kernel{
            rootpath: host_path,
            fs_map: BTreeMap::new(),
            tasks: BTreeMap::new(),
            current : None
        })
    }

    #[inline(always)]
    pub fn add_task(&mut self, pid : i32) -> KernelResult<()> {
        info!("pid {} added", pid);
        self.tasks.insert(pid,
            Arc::new(
                Mutex::new(
                    Task::new(pid)
                )
            )
        );
        Ok(())
    }

    #[inline(always)]
    pub fn clone_task(&mut self, pid : i32, fpid : i32) -> KernelResult<()> {
        info!("pid {} added", pid);
        if pid != 0 {
            self.tasks.insert(pid,
                Arc::new(
                    Mutex::new({
                        let data = self.tasks[&fpid].lock();
                        data.clone(pid)
                    })
                )
            );
        }
        Ok(())
    }
    
    #[inline(always)]
    pub fn contains(&self, pid : i32) -> bool {
        self.tasks.contains_key(&pid)
    }
    
    #[inline(always)]
    pub fn remove_task(&mut self, pid : i32) -> KernelResult<()> {
        info!("pid {} removed", pid);
        self.tasks.remove(&pid);
        Ok(())
    }
 
    #[inline(always)]
    pub fn try_set_current(&mut self, pid : i32) -> bool {
        if let Some(t) = self.tasks.get(&pid) {
            self.current = Some(t.clone());
            true
        }else {false}
    }
}

pub mod fs;