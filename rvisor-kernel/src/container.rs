
use alloc::vec::*;
use linux_kernel_module::bindings;
use linux_kernel_module::KernelResult;
use linux_kernel_module::Error;
use crate::kernel::Kernel;

use alloc::borrow::ToOwned;
use alloc::string::String;

pub struct Container {
    pub tasks_tgid : Vec<bindings::pid_t>,
    pub kernel : Option<Kernel>,
}

static mut container: Container = Container {
    tasks_tgid: Vec::new(),
    kernel: None,
};

impl Container {
    pub fn get_container() -> &'static mut Container {
        // 单线程，可以安全访问static
        unsafe {&mut container}
    }

    pub fn init(&mut self, path : String) -> KernelResult<()> {
        self.kernel = Some(
            Kernel::new(path)?
        );
        Ok(())
    }

    pub fn add_task(&mut self, pid : bindings::pid_t) -> KernelResult<()> {
        info!("container: add task {}", pid);
        if self.contains(pid) {
            return Err(Error::EINVAL);
        }
        self.tasks_tgid.push(pid);
        Ok(())
    }

    pub fn contains(&self, pid : bindings::pid_t) -> bool {
        if let Some(_a) = self.tasks_tgid.iter().find(|&x| *x == pid) {
            true
        } else {
            false
        }
    }

    pub fn remove_task(&mut self, pid : bindings::pid_t) -> KernelResult<()> {
        info!("container: remove task {}", pid);
        if self.contains(pid) {
            self.tasks_tgid.retain(|&x| x != pid);
            Ok(())
        } else {
            Err(Error::EINVAL)
        }
    }
    pub fn runk_mut<T, F : Fn(&mut Kernel) -> T>(&mut self, f : F) -> Option<T> {
        if let Some(ref mut k) = self.kernel {
            Some(f(k))
        } else {
            None
        }
    }
}