use std::mem::{size_of, transmute};
use std::path::PathBuf;
use libc::{c_void, PATH_MAX};
use crate::error::Result;
use crate::error::Error;
use nix::unistd::Pid;
use nix::sys::ptrace;
use std::io;
use log::debug;

use super::*;


#[cfg(target_pointer_width = "32")]
#[inline]
pub fn convert_word_to_bytes(value_to_convert: Word) -> [u8; 4] {
    unsafe { transmute(value_to_convert) }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn convert_word_to_bytes(value_to_convert: Word) -> [u8; 8] {
    unsafe { transmute(value_to_convert) }
}


#[inline]
pub fn read_path(pid: Pid, src_path: *mut Word) -> Result<PathBuf> {
    let bytes = read_string(pid, src_path, PATH_MAX as usize)?;

    if bytes.len() >= PATH_MAX as usize {
        let ioerr = io::Error::new(io::ErrorKind::InvalidData, "name to long when reading sys arg path");
        return Err(Error::IoError(ioerr));
    }

    Ok(PathBuf::from(unsafe { String::from_utf8_unchecked(bytes) }))
}


pub fn read_string(pid: Pid, src_string: *mut Word, max_size: usize) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::with_capacity(max_size);


    let word_size = size_of::<Word>();
    let nb_trailing_bytes = (max_size % word_size) as isize;
    let nb_full_words = ((max_size - nb_trailing_bytes as usize) / word_size) as isize;

    // Copy one word by one word, except for the last one.
    for i in 0..nb_full_words {
        let src_addr = unsafe { src_string.offset(i) as *mut c_void };

        // ptrace returns a c_long/Word that we will interpret as an 8-letters word
        debug!("get src : {:?}", src_addr);
        let word = ptrace::read(pid, src_addr)? as Word;
        let letters = convert_word_to_bytes(word);

        for &letter in &letters {
            // Stop once an end-of-string is detected.
            if letter as char == '\0' {
                // bytes.push(letter); // we do not add the null byte to the path
                bytes.shrink_to_fit();

                return Ok(bytes);
            }
            bytes.push(letter);
        }
    }

    panic!("not implement yet!");
}

impl Registers {
    pub fn get_path(&self, r : SysReg) -> Result<PathBuf> {
        read_path(self.pid, self.get(r) as *mut Word)
    }
}