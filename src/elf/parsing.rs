use crate::elf::ArchitectureWidth;
use crate::errors::ParseError;
use crate::parsing::read_bytes;
use crate::parsing::Parse;
use crate::shared::Endianess;
use crate::shared::NativeInteger;
use std::io::Read;

pub(crate) trait ParseFromEndianess: Sized {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError>;
}

impl ParseFromEndianess for u16 {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let buffer = read_bytes::<2>(reader)?;
        Ok(match endianess {
            Endianess::Little => u16::from_le_bytes(buffer),
            Endianess::Big => u16::from_be_bytes(buffer),
        })
    }
}

impl ParseFromEndianess for u32 {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let buffer = read_bytes::<4>(reader)?;
        Ok(match endianess {
            Endianess::Little => u32::from_le_bytes(buffer),
            Endianess::Big => u32::from_be_bytes(buffer),
        })
    }
}

impl ParseFromEndianess for u64 {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<u64, ParseError> {
        let buffer = read_bytes(reader)?;
        Ok(match endianess {
            Endianess::Little => u64::from_le_bytes(buffer),
            Endianess::Big => u64::from_be_bytes(buffer),
        })
    }
}

impl<T> ParseFromEndianess for T
where
    T: Parse,
{
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        T::parse(reader)
    }
}

pub(crate) trait ParseFromContext: Sized {
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError>;
}

impl ParseFromContext for NativeInteger {
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError> {
        match arch_width {
            ArchitectureWidth::ThirtyTwo => {
                u32::parse_from_endianess(reader, endianess).map(|res| res.into())
            }
            ArchitectureWidth::SixtyFour => {
                u64::parse_from_endianess(reader, endianess).map(|res| res.into())
            }
        }
    }
}

impl<T> ParseFromContext for T
where
    T: ParseFromEndianess,
{
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError> {
        T::parse_from_endianess(reader, endianess)
    }
}

pub(crate) trait ParseFromWorkingContext<T>: Sized {
    fn parse_from_working_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
        working_context: &T,
    ) -> Result<Self, ParseError>;
}

impl<T, U> ParseFromWorkingContext<T> for U
where
    U: ParseFromContext,
{
    fn parse_from_working_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
        working_context: &T,
    ) -> Result<Self, ParseError> {
        U::parse_from_context(reader, endianess, arch_width)
    }
}
