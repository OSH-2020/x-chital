use crate::platform::registers::{Registers, SysReg};

use super::super::Kernel;
use log::{info, debug};
impl Kernel {
    pub fn openat_enter(&mut self, regs : &mut Registers) {
        let path = regs.get_path(SysReg::Arg2).unwrap();
        
        match self.fs.translate_path(path.as_path()) {
            Ok(path) =>{
                debug!("openat {}", path.display());
                regs.set_path(SysReg::Arg2, path.as_path());
            }
            Err(_) => regs.chreg(SysReg::Arg2, 0),
        }

    }
}