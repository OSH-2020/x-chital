use std::process::{Command, Child};
use std::os::unix::process::CommandExt;

use log::{info, debug};

use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, wait};
use nix::unistd::Pid;
use nix::sys::wait::WaitStatus;

use crate::error::Result;
use super::registers::{Registers, SysReg, Word};

pub unsafe fn create_process(mut cmd : Command) -> Result<Child> {
    // ! pre_exec use fork, can't handle child's error, so make it unsafe
    cmd.pre_exec(move || {
        match ptrace::traceme() {
            Ok(()) => Ok(()),
            Err(_) => Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "traceme failed"
                )
            ),
        }
    });
    let child = cmd.spawn()
            .expect("command spawn failed");
    configure(child.id() as i32)?;
    Ok(child)
}

fn configure(pid : i32) -> Result<()> {
    let pid = Pid::from_raw(pid);
    if let WaitStatus::Stopped(_, sig) = waitpid(pid, None).unwrap() {
        assert_eq!(sig, Signal::SIGTRAP);

        ptrace_set_option(pid)?;
        ptrace::syscall(pid, None)?;
    } else {
        panic!("child not stop");
    }
    Ok(())
}

fn ptrace_set_option(pid : Pid) -> Result<()> {
    use ptrace::Options;
    ptrace::setoptions(pid,
        Options::PTRACE_O_TRACESYSGOOD |
        Options::PTRACE_O_TRACEFORK |
        Options::PTRACE_O_TRACEVFORK |
        Options::PTRACE_O_TRACECLONE |
        Options::PTRACE_O_TRACEEXIT
    )?;
    Ok(())
}

pub trait Tracer {
    fn enter_syscall(&mut self, pid : Pid);
    fn exit_syscall(&mut self, pid : Pid);

    fn event_loop(&mut self) -> Result<()> {
        info!("ptrace event loop start");
        let mut entering = true;

        loop {
            let status = wait().expect("wait failed!");
            debug!("{:?}", status);
            match status {
                WaitStatus::PtraceSyscall(pid) if entering => {
                    self.enter_syscall(pid);
                    ptrace::syscall(pid, None)
                                        .expect("ptrace::syscall failed");
                    entering = !entering;
                }
                WaitStatus::PtraceSyscall(pid) if !entering => {
                    self.exit_syscall(pid);
                    ptrace::syscall(pid, None)
                                        .expect("ptrace::syscall failed");
                    entering = !entering;
                }
                WaitStatus::Stopped(pid, sig)  => {
                    info!("{} stopped with {}", pid, sig);
                    ptrace_set_option(pid).unwrap();
                    ptrace::syscall(pid, None)
                        .expect("ptrace::syscall failed");
                }
                WaitStatus::PtraceEvent(pid, Signal::SIGTRAP, i) => {
                    info!("{} SIGTRAP with {}", pid, i);
                    ptrace_set_option(pid).unwrap();
                    ptrace::syscall(pid, None)
                        .expect("ptrace::syscall failed");
                }
                WaitStatus::Exited(pid, i) => {
                    info!("process {} exit with {}", pid, i);
                    break;
                }
                _ => {
                    panic!("unimplement event!");
                }
            }
        };

        info!("ptrace end");
        Ok(())
    }
}