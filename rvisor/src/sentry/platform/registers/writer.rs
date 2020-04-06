use std::path::Path;
use std::os::unix::ffi::OsStrExt;
use std::mem;
use std::io::Read;
use libc::c_void;
use nix::sys::ptrace;
use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

use crate::error::Result;
use super::reader::convert_word_to_bytes;
use super::*;

#[cfg(target_pointer_width = "32")]
#[inline]
pub fn convert_bytes_to_word(value_to_convert: [u8; 4]) -> Word {
    unsafe { mem::transmute(value_to_convert) }
}

#[cfg(target_pointer_width = "64")]
#[inline]
pub fn convert_bytes_to_word(value_to_convert: [u8; 8]) -> Word {
    unsafe { mem::transmute(value_to_convert) }
}

impl Registers {
    /// Converts `path` into bytes before calling the following function.
    pub fn set_path(
        &mut self,
        sys_arg: SysReg,
        path: &Path,
    ) -> Result<()> {
        self.set_data(
            sys_arg,
            path.as_os_str().as_bytes(),
        )
    }

    /// Copies all bytes of `data` to the tracee's memory block
    /// and makes `sys_arg` point to this new block.
    pub fn set_data(
        &mut self,
        sys_arg: SysReg,
        data: &[u8],
    ) -> Result<()> {
        // Allocate space into the tracee's memory to host the new data.
        let tracee_ptr = self.alloc_mem(data.len() as isize)?;

        // Copy the new data into the previously allocated space.
        self.write_data(tracee_ptr as *mut Word, data)?;

        // Make this argument point to the new data.
        self.chreg(sys_arg, tracee_ptr);

        Ok(())
    }

    fn write_data(&self, dest_tracee: *mut Word, data: &[u8]) -> Result<()> {
        // The byteorder crate is used to read the [u8] slice as a [Word] slice.
        let null_char_slice: &[u8] = &['\0' as u8];
        let mut buf = Cursor::new(data).chain(Cursor::new(null_char_slice));

        let size = data.len() + 1; // the +1 is for the `\0` byte that we will have manually
        let word_size = mem::size_of::<Word>();
        let nb_trailing_bytes = (size % word_size) as isize;
        let nb_full_words = ((size - nb_trailing_bytes as usize) / word_size) as isize;

        // Copy one word by one word, except for the last one.
        for i in 0..nb_full_words {
            let word = buf.read_uint::<LittleEndian>(word_size).unwrap() as Word;
            let dest_addr = unsafe { dest_tracee.offset(i) as *mut c_void };

            ptrace::write(
                self.pid,
                dest_addr,
                word as *mut c_void,
            )?;
        }

        // Copy the bytes in the last word carefully since we have to
        // overwrite only the relevant ones.
        let last_dest_addr = unsafe { dest_tracee.offset(nb_full_words) as *mut c_void };
        let existing_word =
            ptrace::read(self.pid, last_dest_addr)? as Word;
        let mut bytes = convert_word_to_bytes(existing_word);

        // The trailing bytes are merged with the existing bytes. For example:
        // bytes = [0, 0, 0, 0, 0, 0, 119, 0] // the already existing bytes at the dest addr
        // trailing bytes = [164, 247, 274] // our trailing bytes
        // fusion = [164, 247, 274, 0, 0, 0, 119, 0] // the fusion of the two
        for j in 0..nb_trailing_bytes as usize {
            bytes[j] = buf.read_u8().unwrap();
        }

        let last_word = convert_bytes_to_word(bytes);
        // We can now safely write the final word.
        ptrace::write(
            self.pid,
            last_dest_addr,
            last_word as *mut c_void,
        )?;

        Ok(())
    }
}