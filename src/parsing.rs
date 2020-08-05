use crate::errors::ParseError;
use crate::shared::Endianess;
use std::io::Read;

pub trait Parse: Sized {
    /// Build the type from a readable sequence of bytes.
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError>;
}

impl Parse for u8 {
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError> {
        let byte = read_bytes::<1>(reader)?;
        Ok(byte[0])
    }
}

pub trait ToByte {
    fn to_bytes(&self) -> [u8; 1];
}

impl ToByte for u8 {
    fn to_bytes(&self) -> [u8; 1] {
        [*self]
    }
}

pub trait To2Bytes {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 2];
}

impl To2Bytes for u16 {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 2] {
        match endianess {
            Endianess::Little => self.to_le_bytes(),
            Endianess::Big => self.to_be_bytes(),
        }
    }
}

pub trait To4Bytes {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 4];
}

impl To4Bytes for u32 {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 4] {
        match endianess {
            Endianess::Little => self.to_le_bytes(),
            Endianess::Big => self.to_be_bytes(),
        }
    }
}

pub trait To8Bytes {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 8];
}

impl To8Bytes for u64 {
    fn to_bytes(&self, endianess: Endianess) -> [u8; 8] {
        match endianess {
            Endianess::Little => self.to_le_bytes(),
            Endianess::Big => self.to_be_bytes(),
        }
    }
}

pub(crate) fn read_bytes<const SIZE: usize>(
    reader: &mut impl Read,
) -> Result<[u8; SIZE], ParseError> {
    let mut buffer = [0; SIZE];
    if reader.read(&mut buffer)? < SIZE {
        Err(ParseError::MissingData)
    } else {
        Ok(buffer)
    }
}
