use std::path::{Path, PathBuf};
use crate::error::{Error, Result};
use log::debug;
use std::env;
use std::io;

pub struct Fs {
    // Working directory
    cwd: PathBuf,
    // Guest root (the binding associated to `/`)
    root: PathBuf,
}

impl Fs {
    pub fn new(root : &Path) -> Result<Fs> {
        // add some path
        Ok(Fs {
            cwd: Path::new("/").to_path_buf(),
            root: root.canonicalize()?,
        })
    }
    // Translates a path from guest to host.
    pub fn translate_path(&self, user_path: &Path) -> Result<PathBuf> {
        let mut guest_path = PathBuf::new();

        //TODO: dir_fd != AT_FDCWD 的情况
        if user_path.is_relative() {
            // It is relative to the current working directory.
            guest_path.push(&self.cwd);
        } else {
            guest_path.push(PathBuf::from("/"))
        }

        debug!(
            "translate_path({:?}, {:?})",
            guest_path, user_path
        );


        guest_path.push(user_path);

        let guest_path = guest_path.strip_prefix("/")
                                                .expect("guest_path turn to relative");

        let host_path = self.root.join(guest_path);

        debug!("translate_path -> {:?}", host_path);
        let host_path = host_path.canonicalize()?; // ! 如果文件不存在，此处会出现io::Error(NotFound)异常

        if !host_path.starts_with(&self.root) {
            let ioerr = io::Error::new(io::ErrorKind::NotFound, "path not found");
            Err(Error::IoError(ioerr))
        }else {
            Ok(host_path)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fs() {
        let guest_root = env::current_dir().unwrap();
        let fs = Fs::new(Path::new(".")).unwrap();
        assert_eq!(
            fs.translate_path(Path::new("/")).unwrap(),
            guest_root
        );
        assert_eq!(
            fs.translate_path(Path::new("./src")).unwrap(),
            guest_root.join("src")
        );
        assert_eq!(
            fs.translate_path(Path::new("/src")).unwrap(),
            guest_root.join("src")
        );
        assert!(
            matches!(
                fs.translate_path(Path::new("/..")),
                Err(_)
            )
        );
    }
}