use std::usize::MAX as USIZE_MAX;
use crate::error::{Result, Error};
use super::*;
use std::io;

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const RED_ZONE_SIZE: isize = 128;
#[cfg(all(target_os = "linux", not(target_arch = "x86_64")))]
const RED_ZONE_SIZE: isize = 0;


impl Registers {
    pub fn alloc_mem(&mut self, size: isize) -> Result<Word> {
        let original_stack_pointer = self.orig_sp;
        let stack_pointer = self.get(SysReg::Sp);

        // Some ABIs specify an amount of bytes after the stack
        // pointer that shall not be used by anything but the compiler
        // (for optimization purpose).
        let corrected_size = match stack_pointer == original_stack_pointer {
            false => size,
            true => size + RED_ZONE_SIZE,
        };
        let overflow = corrected_size > 0 && stack_pointer <= corrected_size as Word;
        let underflow = corrected_size < 0 &&
            stack_pointer >= (USIZE_MAX as Word) - (-corrected_size as Word);

        if overflow || underflow {
            return Err(Error::IoError(
                io::Error::new(io::ErrorKind::InvalidData, "address invalid!" )
            ));
        }

        // Remember the stack grows downward.
        let new_stack_pointer = match corrected_size > 0 {
            true => stack_pointer - (corrected_size as Word),
            false => stack_pointer + (-corrected_size as Word),
        };

        self.chreg(SysReg::Sp, new_stack_pointer);

        Ok(new_stack_pointer)
    }
}