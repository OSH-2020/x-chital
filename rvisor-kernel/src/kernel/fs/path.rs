
use linux_kernel_module as lkm;
use lkm::KernelResult;
use lkm::Error;
use lkm::bindings;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

use crate::kernel::Kernel;
use crate::kernel::task::Task;
use crate::kernel::fs::*;
use crate::string;

impl Task {
    #[inline(always)]
    pub fn get_cwd(&self, ret : &mut String) {
        if self.cwd.len() == 0 {
            ret.push('/');
        } else {
            for ref p in &self.cwd {
                ret.push('/');
                ret.push_str(p.as_str());
            }
        }
    }

    pub fn chdir(&mut self, path : &String) -> KernelResult<()> {
        if path == "" {
            return Err(Error::EINVAL)
        }
        if &path[0..1] == "/" {
            self.cwd.clear();
        }
        for entry in path.split("/") {
            match entry {
                ".." => {
                    if self.cwd.pop() == None {
                        return Err(Error::EINVAL);
                    }
                }
                "." => (),
                "" => (),
                _ => {self.cwd.push(String::from(entry));}
            }
        }
        Ok(())
    }
}

pub enum Path {
    Host(String),
    Mapped(Arc<Mutex<DEntry>>),
}

impl Kernel {
    pub fn guest_to_host(&self, guest_path : &String) -> KernelResult<Path> {
        if guest_path == "" {
            Err(Error::EINVAL)
        }
        else if &guest_path[0..1] == "/" {
            info!("kernel path: root path found");
            Ok(self.path_convert(guest_path, false)?)
        } else {
            Ok(self.path_convert(guest_path, true)?)
        }
    }

    pub fn relative_to_absolute(&self, guest_path : &String) -> KernelResult<Path>{
        if let Some(ref cur) = self.current {
            let mut stack : Vec<&str> = Vec::with_capacity(20);
            let data = cur.lock();
            for ref p in &data.cwd {
                stack.push(& p.as_str());
            }
            self.guest_to_host_with_stack(&stack)
        } else {
            Err(Error::EINVAL)
        }
    }

    fn path_convert(&self, guest_path : &String, relative : bool) -> KernelResult<Path> {
        let mut stack : Vec<&str> = Vec::with_capacity(20);
        if relative{
            if let Some(ref cur) = self.current {
                let data = cur.lock();
                for ref p in &data.cwd {
                    stack.push(& p.as_str());
                }
                self.build_stack(guest_path, &mut stack)?;
                self.guest_to_host_with_stack(&stack)
            } else {
                Err(Error::EINVAL)
            }
        } else {
            self.build_stack(guest_path, &mut stack)?;
            let mut dentry = self.fs_map.get(stack[0]);
            if let Some(dentry) = dentry {
                let mut tmp = dentry.clone();
                let mut iter = stack.iter();
                iter.next();

                let mut flag = true;
                for name in iter {
                    if let Some(ref next) = tmp.clone().lock().child.iter().find(|&x| { &&x.lock().name == name }) {
                        tmp = (**next).clone();
                    } else {
                        flag = false;
                        break;
                    }
                }
                if flag {
                    return Ok(Path::Mapped(tmp.clone()));
                }
            }
            self.guest_to_host_with_stack(&stack)
        }
    }
    fn build_stack<'a>(&self, guest_path : &'a String, stack : &mut Vec<&'a str>) -> KernelResult<()> {
        for entry in guest_path.split("/") {
            match entry {
                ".." => {
                    if stack.pop() == None {
                        return Err(Error::EINVAL);
                    }
                }
                "." => (),
                "" => (),
                _ => {stack.push(entry);}
            }
        }
        Ok(())
    }
    fn guest_to_host_with_stack(&self, stack : &Vec<&str>) -> KernelResult<Path> {
        let mut ret = String::with_capacity(80);
        ret.push_str(self.rootpath.as_str());
        for entry in stack {
            ret.push('/');
            ret.push_str(entry);
        }
        info!("kernel path : {}", ret);
        Ok(Path::Host(ret))
    }
}