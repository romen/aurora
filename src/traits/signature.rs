use std::fmt::Debug;

#[derive(Debug)]
pub struct Error();

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "signature error")
    }
}

impl core::error::Error for Error {}
