use std::process::{Command, Child};
use std::os::unix::process::CommandExt;

use::log::info;

use nix::sys::ptrace;
use nix::sys::signal::Signal;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use nix::sys::wait::WaitStatus;

pub fn start_ptrace(pid : Pid) -> Result<(), nix::Error> {
    info!("ptrace start");

    if let WaitStatus::Stopped(_, sig) = waitpid(pid, None).unwrap() {
        assert_eq!(sig, Signal::SIGTRAP);
        if let Err(e) = ptrace::setoptions(pid, ptrace::Options::PTRACE_O_TRACESYSGOOD) {
            info!("{:?}", e);
            panic!("Asdf");
        }
        ptrace::syscall(pid, None).unwrap();
    } else {
        panic!("child not stop");
    }

    while let Ok(status) = waitpid(pid, None){
        info!("waitpid returned : {:?}", status);   
        match status {
            WaitStatus::PtraceSyscall(_) => {
                info!("get ptrace syscall");

                let regs = ptrace::getregs(pid)
                                    .expect("getreg failed");
                println!("{}", regs.orig_rax);
                ptrace::syscall(pid, None)?;
            }
            _ => ()
        }
    };

    info!("ptrace end");
    Ok(())
}

