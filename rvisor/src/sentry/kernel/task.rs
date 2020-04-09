use super::Kernel;

use std::process::{Command, Child};
use std::os::unix::process::CommandExt;
use log::info;

use crate::platform::ptrace;
use crate::platform::ptrace::Tracer;
use crate::error::Result;

pub struct Task {
    pid : i32,
}

fn setenv(cmd : & mut Command) {
    cmd.env_clear().env("PATH", "/usr/bin").env("PWD", "/");
    info!("environment variables $PATH and $PWD have been setted for cmd {:#?}.", cmd);
}



impl Task {
    pub fn create(cmd : Command) -> Result<Task> {
        info!("creating task for cmd {:#?}", cmd);
        setenv(&mut cmd);
        let child = unsafe {
            ptrace::create_process(cmd)?
        };
        let pid = child.id() as i32;
        
        info!("task created with pid: {}", pid);

        Ok(Task {
            pid : pid,
        })
    }
}


impl Kernel {
    pub fn create_task(&mut self, cmd : Command) -> Result<()> {
        self.tasks.push(Task::create(cmd)?);
        Ok(())
    }
    pub fn run(&mut self) -> Result<()> {
        self.event_loop()?;
        Ok(())
    }
}

