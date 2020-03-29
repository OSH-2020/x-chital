pub mod task;
pub use task::*;

use super::platform;

pub struct Kernel {
    tasks : Vec<task::Task>
}

impl Kernel {
    pub fn new() -> Self {
        Self { tasks : vec![]}
    }
}