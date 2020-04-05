
use std::process::{Command};
use log::info;

// start 'runrs boot'
pub fn run_commmand(tty: bool, command: &str) {
    info!("executing run command");
    
    // /proc/self/exe is like 127.0.0.1, a symbol for the same execuble file.
    let mut parent = Command::new("/proc/self/exe");
    
    parent.arg("boot");
    parent.arg(command);
    
    if tty {
        parent.arg("-tty");
    }
    
    let mut parent = parent.spawn()
        .expect("failed to create init command process");
    
    // wait for end of the process
    let ecode = parent.wait()
    .expect("failed to wait on child");

    info!("program exited with {}", ecode);
}