use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use xcb::GenericError;

macro_rules! handle_reply {
    ($e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => return Err(XcbGeneric(e)),
        }
    };

    ($e:expr, $cleanup:stmt) => {
        match $e {
            Ok(r) => r,
            Err(e) => {
                $cleanup;
                return Err(XcbGeneric(e));
            }
        }
    };
}

#[derive(Debug)]
pub enum I3LockrError {
    XcbGeneric(GenericError),
    ShmOpen(IoError),
    FTruncate(IoError),
    MMap(IoError),
}

impl Error for I3LockrError {}

impl fmt::Display for I3LockrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            I3LockrError::XcbGeneric(e) => write!(f, "XCB error: {}", e),
            I3LockrError::ShmOpen(e) => write!(f, "shm_open() returned error: {}", e),
            I3LockrError::FTruncate(e) => write!(f, "ftruncate() returned error: {}", e),
            I3LockrError::MMap(e) => write!(f, "mmap() returned error: {}", e),
        }
    }
}
