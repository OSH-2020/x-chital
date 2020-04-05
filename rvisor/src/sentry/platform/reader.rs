use std::mem::{size_of, transmute};
use std::ptr::null_mut;
use std::path::PathBuf;
use libc::{c_void, PATH_MAX};
use crate::error::Result;
use crate::error::Error;
use nix::unistd::Pid;
use nix::sys::ptrace::ptrace;
use nix::sys::ptrace::ptrace::PTRACE_PEEKDATA;


#[cfg(target_pointer_width = "32")]
#[inline]
pub fn convert_word_to_bytes(value_to_convert: c_ulong) -> [u8; 4] {
    unsafe { transmute(value_to_convert) }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn convert_word_to_bytes(value_to_convert: c_ulong) -> [u8; 8] {
    unsafe { transmute(value_to_convert) }
}


#[inline]
fn read_path(pid: Pid, src_path: *mut c_ulong) -> Result<PathBuf> {
    let bytes = read_string(pid, src_path, PATH_MAX as usize)?;

    if bytes.len() >= PATH_MAX as usize {
        return Err(Error::name_too_long("when reading sys arg path"));
    }

    Ok(PathBuf::from(unsafe { String::from_utf8_unchecked(bytes) }))
}


fn read_string(pid: Pid, src_string: *mut c_ulong, max_size: usize) -> Result<Vec<u8>> {
    let mut bytes: Vec<u8> = Vec::with_capacity(max_size);


    let word_size = size_of::<c_ulong>();
    let nb_trailing_bytes = (max_size % word_size) as isize;
    let nb_full_words = ((max_size - nb_trailing_bytes as usize) / word_size) as isize;

    // Copy one word by one word, except for the last one.
    for i in 0..nb_full_words {
        let src_addr = unsafe { src_string.offset(i) as *mut c_void };

        // ptrace returns a c_long/c_ulong that we will interpret as an 8-letters word
        let word = ptrace::read(pid, src_addr)? as c_ulong;
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