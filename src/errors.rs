use std::io::Error as IoError;

#[derive(Debug)]
pub enum ParseError {
    IoError(IoError),
    MissingData,
    InvalidValue { value: Box<[u8]> },
}

impl From<IoError> for ParseError {
    fn from(io_error: IoError) -> Self {
        Self::IoError(io_error)
    }
}
