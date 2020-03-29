
use std::process::{Command, Stdio};
use super::sentry::kernel::Kernel;

use log::info;

pub fn boot_commmand(tty: bool, command: &str) {

    info!("executing boot command");
    
    let mut parent = Command::new(command);
    
    if tty {
        info!("using tty for stdin/stdout/stderr");
        parent.stdin(Stdio::inherit());
        parent.stdout(Stdio::inherit());
        parent.stderr(Stdio::inherit());
    }
    
    let mut k = Kernel::new();
    if let Err(e) = k.create_task(&mut parent) {
        println!("create_task err : {}", e);
    }
    info!("kernel created");

    k.run();
}