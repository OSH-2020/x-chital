use super::Kernel;

use std::process::{Command, Child};
use std::os::unix::process::CommandExt;
use std::io::{Error, ErrorKind};
use std::thread;

use nix::sys::ptrace;
use nix::unistd::Pid;

use log::{info, debug};

use super::platform::ptrace::start_ptrace;

pub struct Task {
    pid : Pid,

    // a service thread for new process
    // handle : thread::JoinHandle<()>
}

impl Task {
    pub fn create(cmd : &mut Command) -> Result<Task, Error> {
        info!("creating task for cmd {:#?}", cmd);

        unsafe {
            // pre_exec use fork, can't handle child's error.
            cmd.pre_exec(move ||{
                match ptrace::traceme() {
                    Ok(()) => Ok(()),
                    Err(_) => Err(Error::new(ErrorKind::Other, "traceme failed")),
                }
            });
        }
        let mut child = cmd.spawn()
                            .expect("command spawn failed");
        let pid = child.id() as i32;


        info!("task created with pid: {}", pid);

        start_ptrace(
            Pid::from_raw(pid)
        ).expect("unknown error");
        child.wait().expect("child wait error!");
        info!("thread joining");
    

        Ok(Task {
            pid : Pid::from_raw(0),
            // handle: handle,
        })
    }
}


impl Kernel {
    pub fn create_task(&mut self, cmd : &mut Command) -> Result<(), Error> {
        self.tasks.push(Task::create(cmd)?);
        Ok(())
    }

    pub fn run(self) {
        
        for t in self.tasks {
            // t.handle.join().expect("service error");
            info!("process {} joined", t.pid);
        }
    }
}