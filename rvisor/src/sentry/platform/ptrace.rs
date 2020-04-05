use std::process::{Command, Child};
use std::os::unix::process::CommandExt;

use log::{info, debug};

use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::{waitpid, wait};
use nix::unistd::Pid;
use nix::sys::wait::WaitStatus;

use crate::error::Result;

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

        ptrace::setoptions(pid, ptrace::Options::PTRACE_O_TRACESYSGOOD)?;
        ptrace::syscall(pid, None)?;
    } else {
        panic!("child not stop");
    }
    Ok(())
}

pub trait Tracer {

    fn event_loop(&self) -> Result<()> {
        info!("ptrace event loop start");

        loop {
            let status = wait().expect("wait failed!");
            debug!("waitpid returned : {:?}", status);   
            match status {
                WaitStatus::PtraceSyscall(pid) => {
                    debug!("get ptrace syscall");

                    let regs = ptrace::getregs(pid)
                                        .expect("getreg failed");
                    println!("{}", regs.orig_rax);
                    ptrace::syscall(pid, None)
                                        .expect("ptrace::syscall failed");
                }
                _ => break
            }
        };

        info!("ptrace end");
        Ok(())
    }
}