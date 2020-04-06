pub mod task;
pub mod fs;
pub mod syscall;

use std::path::Path;
use crate::platform::ptrace;
use crate::error::*;

pub struct Kernel {
    tasks : Vec<task::Task>,
    fs : fs::Fs,
}

impl Kernel {
    pub fn new(root : &Path) -> Result<Self> {
        Ok(Self {
            tasks : vec![],
            fs: fs::Fs::new(root)?,
        })
    }
}