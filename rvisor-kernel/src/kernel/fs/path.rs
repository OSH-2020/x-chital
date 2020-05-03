
use crate::kernel::Kernel;
use linux_kernel_module::KernelResult;
use linux_kernel_module::Error;
use linux_kernel_module::bindings;

use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;


impl Kernel {
    pub fn guest_to_host(&self, guest_path : &String) -> KernelResult<String> {
        if guest_path == "" {
            Err(Error::EINVAL)
        }
        else if &guest_path[0..1] == "/" {
            info!("kernel path: root path found");
            let mut stack : Vec<&str> = Vec::with_capacity(10);
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
            info!("kernel path : {:?}", stack);
            String::with_capacity(80);
            let mut ret = String::new();
            ret.push_str(self.rootpath.as_str());
            for entry in &stack {
                ret.push('/');
                ret.push_str(entry);
            }
            info!("kernel path : {}", ret);
            Ok(ret)
        } else {
            Err(Error::EFAULT)
        }
    }
}