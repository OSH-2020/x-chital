
use linux_kernel_module::{KernelResult, Error, bindings};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::btree_map::BTreeMap;
use alloc::rc::Rc;

pub const PATH_MAX : usize = 200;

pub mod task;
use task::Task;

pub struct Kernel {
    rootpath : String,
    tasks : BTreeMap<i32, Rc<Task>>,
    current : Option<Rc<Task>>,
}

impl Kernel {
    pub fn new(host_path : String) -> KernelResult<Kernel> {
        Ok(Kernel{
            rootpath: host_path,
            tasks: BTreeMap::new(),
            current : None
        })
    }

    #[inline(always)]
    pub fn add_task(&mut self, pid : i32) -> KernelResult<()> {
        self.tasks.insert(pid, Rc::new(
            Task::new(pid)
        ));
        Ok(())
    }

    #[inline(always)]
    pub fn contains(&self, pid : i32) -> bool {
        self.tasks.contains_key(&pid)
    }

    #[inline(always)]
    pub fn remove_task(&mut self, pid : i32) -> KernelResult<()> {
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