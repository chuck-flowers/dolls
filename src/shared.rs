use crate::errors::ParseError;
use crate::parsing::Parse;
use std::io::Read;

pub enum NativeInteger {
    ThirtyTwo(u32),
    SixtyFour(u64),
}

impl From<u32> for NativeInteger {
    fn from(src: u32) -> Self {
        NativeInteger::ThirtyTwo(src)
    }
}

impl From<u64> for NativeInteger {
    fn from(src: u64) -> Self {
        NativeInteger::SixtyFour(src)
    }
}

#[derive(Clone, Copy)]
pub enum Endianess {
    Little,
    Big,
}

impl Parse for Endianess {
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError> {
        let mut buf = [0];
        if reader.read(&mut buf)? < 1 {
            return Err(ParseError::MissingData);
        }

        Ok(match buf[0] {
            1 => Self::Little,
            2 => Self::Big,
            other => {
                return Err(ParseError::InvalidValue {
                    value: Box::new([other]),
                })
            }
        })
    }
}
