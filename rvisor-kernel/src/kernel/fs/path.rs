
use linux_kernel_module as lkm;
use lkm::KernelResult;
use lkm::Error;
use lkm::bindings;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;

use crate::kernel::Kernel;
use crate::kernel::task::Task;
use crate::string;

impl Task {
    #[inline(always)]
    pub fn get_cwd(&self, ret : &mut String) {
        for ref p in &self.cwd {
            ret.push('/');
            ret.push_str(p.as_str());
        }
    }

    pub fn chdir(&mut self, path : &String) -> KernelResult<()> {
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

impl Kernel {
    pub fn guest_to_host(&self, guest_path : &String) -> KernelResult<String> {
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

    fn path_convert(&self, guest_path : &String, relative : bool) -> KernelResult<String> {
        let mut stack : Vec<&str> = Vec::with_capacity(20);
        if relative {
            if let Some(ref cur) = self.current{
                for ref p in &cur.cwd {
                    stack.push(& p.as_str());
                }
            }
        }
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
        // info!("kernel path : {:?}", stack);

        let mut ret = String::with_capacity(80);
        ret.push_str(self.rootpath.as_str());
        for entry in &stack {
            ret.push('/');
            ret.push_str(entry);
        }
        info!("kernel path : {}", ret);
        Ok(ret)
    }
}