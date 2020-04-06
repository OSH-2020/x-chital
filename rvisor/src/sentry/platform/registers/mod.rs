use nix::unistd::Pid;
use nix::sys::ptrace;

use crate::error::Result;

pub mod reader;
pub mod writer;
pub mod mem;

pub type Word = std::os::raw::c_ulong;

pub enum SysReg {
    Num,
    Arg1,
    Arg2,
    Arg3,
    Sp, // Stack Pointer
}

pub struct Registers {
    pid : Pid,
    regs : libc::user_regs_struct,
    // save Stack Pointer
    orig_sp : Word,
}

impl Registers {
    pub fn get_from(pid : Pid) -> Result<Self> {
        let regs = ptrace::getregs(pid)?;
        Ok(Self {
            regs : regs,
            pid : pid,
            orig_sp : regs.rsp,
        })
    }

    pub fn set_to(self) -> Result<()> {
        ptrace::setregs(self.pid, self.regs)?;
        Ok(())
    }
    pub fn get(&self, r : SysReg) -> Word {
        match r {
            SysReg::Num => self.regs.orig_rax,
            SysReg::Arg1 => self.regs.rdi,
            SysReg::Arg2 => self.regs.rsi,
            SysReg::Arg3 => self.regs.rdx,
            SysReg::Sp => self.regs.rsp,
        }
    }

    pub fn chreg(&mut self, r : SysReg, s : Word) {
        match r {
            SysReg::Num => self.regs.orig_rax = s,
            SysReg::Arg1 => self.regs.rdi = s,
            SysReg::Arg2 => self.regs.rsi = s,
            SysReg::Arg3 => self.regs.rdx = s,
            SysReg::Sp => self.regs.rsp = s,
        }
    }
}
pub use reader::*;
pub use writer::*;
pub use mem::*;
