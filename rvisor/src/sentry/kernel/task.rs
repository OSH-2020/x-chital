use super::Kernel;

use std::process::{Command, Child};
use std::os::unix::process::CommandExt;

use log::info;

use crate::platform::ptrace;
use crate::error::Result;

pub struct Task {
    pid : i32,
    process : Child
}

impl Task {
    pub fn create(cmd : Command) -> Result<Task> {
        info!("creating task for cmd {:#?}", cmd);
        
        let child = unsafe {
            ptrace::create_process(cmd)?
        };
        let pid = child.id() as i32;

        info!("task created with pid: {}", pid);

        Ok(Task {
            pid : pid,
            process: child,
        })
    }
}


impl Kernel {
    pub fn create_task(&mut self, cmd : Command) -> Result<()> {
        self.tasks.push(Task::create(cmd)?);
        Ok(())
    }
    pub fn run(&mut self) -> Result<()> {
        ptrace::event_loop()?;
        Ok(())
    }
}