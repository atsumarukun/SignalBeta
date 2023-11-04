use core::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum Error {
    UefiError(uefi::Error),
    ElfError(goblin::error::Error),
}

impl core::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Error::UefiError(e) => write!(f, "{}", e),
            Error::ElfError(e) => write!(f, "{}", e),
        }
    }
}

impl From<uefi::Error> for Error {
    fn from(error: uefi::Error) -> Self {
        Error::UefiError(error)
    }
}

impl From<goblin::error::Error> for Error {
    fn from(error: goblin::error::Error) -> Self {
        Error::ElfError(error)
    }
}
