
use crate::platform::ptrace;
use crate::platform::registers::{Registers, SysReg};
use super::Kernel;
use nix::unistd::Pid;
use log::{info, debug};

use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

mod open;


#[derive(FromPrimitive, Debug)]
enum Syscall {
    Read = 0,
    Write,
    Open,
    Close,
    Stat,
    Fstat,
    Lstat,
    Mmap = 9,
    Mprotect,
    Munmap,
    Brk,
    Ioctl = 16,
    Access = 21,
    ArchPrctl = 158,
    Readlink = 89,
    Openat = 257,
    Uname=63,
    ExitGroup = 231,
}


impl ptrace::Tracer for Kernel {
    fn enter_syscall(&mut self, pid : Pid){
        let mut regs = Registers::get_from(pid)
                .expect("getreg failed");
        let sysnum = FromPrimitive::from_u64(regs.get(SysReg::Num));
        
        debug!("get ptrace syscall regs sysnum : {:?}", sysnum);

        match sysnum{
            Some(Syscall::Openat) => {
                self.openat_enter(&mut regs);
                regs.set_to()
                            .expect("registers set_to failed");
            }
            Some(_) => (),
            None => {
                debug!("{}", regs.get(SysReg::Num));
                panic!("syscall not implemented!");
            }
        }
    }
    fn exit_syscall(&mut self, pid : Pid){

    }

}