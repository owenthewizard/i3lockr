use std::error::Error;
use std::{fmt, io};

use self::CaptureError::*;

#[derive(Debug)]
pub enum CaptureError {
    Libc(io::Error),
    LibcFunc(String, io::Error),
    Xcb(xcb::GenericError),
}

impl Error for CaptureError {}

impl fmt::Display for CaptureError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Libc(e) => write!(f, "Encountered error while calling libc function: {}", e),
            LibcFunc(s, e) => write!(f, "Encountered error while calling libc::{}(): {}", s, e),
            Xcb(e) => write!(f, "Encountered error while calling xcb function: {}", e),
        }
    }
}

impl From<xcb::GenericError> for CaptureError {
    fn from(error: xcb::GenericError) -> Self {
        Xcb(error)
    }
}

impl From<io::Error> for CaptureError {
    fn from(error: io::Error) -> Self {
        Libc(error)
    }
}
