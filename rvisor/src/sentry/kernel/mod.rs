pub mod task;
pub mod fs;

pub struct Kernel {
    tasks : Vec<task::Task>
}

impl Kernel {
    pub fn new() -> Self {
        Self { tasks : vec![]}
    }
}