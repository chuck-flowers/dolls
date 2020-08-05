//! Types for representing an ELF object file.
//! Reference pulled from [here](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format#File_header)

mod file_header;
mod parsing;
mod program_header;
mod section_header;

pub use self::file_header::FileHeader;
use self::parsing::ParseFromWorkingContext;
pub use self::program_header::ProgramHeader;
pub use self::section_header::SectionHeader;
use crate::errors::ParseError;
use crate::parsing::Parse;
use crate::parsing::ToByte;
use crate::shared::Endianess;
use crate::shared::NativeInteger;
use std::io::Read;

pub struct ElfDescriptor {
    pub file_header: FileHeader,
    pub program_header: ProgramHeader,
    pub section_header: SectionHeader,
}

impl Parse for ElfDescriptor {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let file_header = FileHeader::parse(reader)?;
        let program_header = ProgramHeader::parse_from_working_context(
            reader,
            file_header.endianess,
            file_header.width,
            &file_header,
        )?;
        let section_header = SectionHeader::parse_from_working_context(
            reader,
            file_header.endianess,
            file_header.width,
            &file_header,
        )?;

        Ok(Self {
            file_header,
            program_header,
            section_header,
        })
    }
}

#[derive(Clone, Copy)]
pub enum ArchitectureWidth {
    ThirtyTwo,
    SixtyFour,
}

impl Parse for ArchitectureWidth {
    fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let value = u8::parse(reader)?;

        Ok(match value {
            1 => ArchitectureWidth::ThirtyTwo,
            2 => ArchitectureWidth::SixtyFour,
            other => {
                return Err(ParseError::InvalidValue {
                    value: other.to_bytes().into(),
                })
            }
        })
    }
}
