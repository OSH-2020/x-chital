
// is error can switch between IoError and NixError

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    NixError(nix::Error)
}

impl std::error::Error for Error{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self {
            Error::IoError(ref e) => Some(e),
            Error::NixError(ref e) => Some(e),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Error::IoError(ref e) => e.fmt(f),
            Error::NixError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}

impl From<nix::Error> for Error {
    fn from(e: nix::Error) -> Self {
        Error::NixError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
