
use alloc::vec::*;
use linux_kernel_module::bindings;
use linux_kernel_module::KernelResult;
use linux_kernel_module::Error;
use crate::kernel::Kernel;

use alloc::borrow::ToOwned;
use alloc::string::String;

pub struct Container {
    pub kernel : Option<Kernel>,
}

static mut container: Container = Container {
    kernel: None,
};

impl Container {
    #[inline(always)]
    pub fn get_container() -> &'static mut Container {
        // 单线程，可以安全访问static
        unsafe {&mut container}
    }
    #[inline(always)]
    pub fn init(&mut self, path : String) -> KernelResult<()> {
        self.kernel = Some(
            Kernel::new(path)?
        );
        Ok(())
    }
    #[inline(always)]
    pub fn runk<T, F : Fn(&Kernel) -> T>(&self, f : F) -> Option<T> {
        if let Some(ref k) = self.kernel {
            Some(f(k))
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn runk_mut<T, F : Fn(&mut Kernel) -> T>(&mut self, f : F) -> Option<T> {
        if let Some(ref mut k) = self.kernel {
            Some(f(k))
        } else {
            None
        }
    }
    #[inline(always)]
    pub fn add_task(&mut self, pid : bindings::pid_t) -> KernelResult<()> {
        self.runk_mut(|k| k.add_task(pid)).unwrap()
    }
    #[inline(always)]
    pub fn contains(&self, pid : bindings::pid_t) -> bool {
        self.runk(|k| k.contains(pid)).unwrap_or(false)
    }
    #[inline(always)]
    pub fn remove_task(&mut self, pid : bindings::pid_t) -> KernelResult<()> {
        self.runk_mut(|k| k.remove_task(pid)).unwrap()
    }
    #[inline(always)]
    pub fn try_set_current(&mut self, pid: i32) -> bool {
        self.runk_mut(|k| k.try_set_current(pid)).unwrap_or(false)
    }
}