use crate::elf::file_header::FileHeader;
use crate::elf::parsing::ParseFromContext;
use crate::elf::parsing::ParseFromEndianess;
use crate::elf::parsing::ParseFromWorkingContext;
use crate::elf::ArchitectureWidth;
use crate::elf::Endianess;
use crate::elf::NativeInteger;
use crate::errors::ParseError;
use crate::parsing::*;
use std::io::Read;

pub struct ProgramHeader {
    entries: Vec<Entry>,
}

impl ProgramHeader {
    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry)
    }
}

impl ParseFromWorkingContext<FileHeader> for ProgramHeader {
    fn parse_from_working_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
        working_context: &FileHeader,
    ) -> Result<Self, ParseError> {
        (0..working_context.program_header_entry_count)
            .map(|_| Entry::parse_from_context(reader, endianess, arch_width))
            .collect::<Result<_, _>>()
            .map(|entries| Self { entries })
    }
}

pub struct Entry {
    pub entry_type: EntryType,
    pub flags: u32,
    pub offset: NativeInteger,
    pub virtual_address: NativeInteger,
    pub physical_address: NativeInteger,
    pub segment_file_size: NativeInteger,
    pub segment_mem_size: NativeInteger,
    pub alignment: NativeInteger,
}

impl ParseFromContext for Entry {
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError> {
        let entry_type = EntryType::parse_from_endianess(reader, endianess)?;
        let flags = u32::parse_from_endianess(reader, endianess)?;
        let offset = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let virtual_address = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let physical_address = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let segment_file_size = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let segment_mem_size = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let alignment = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        Ok(Entry {
            entry_type,
            flags,
            offset,
            virtual_address,
            physical_address,
            segment_file_size,
            segment_mem_size,
            alignment,
        })
    }
}

pub enum EntryType {
    Null,
    Load,
    Dynamic,
    Interpreter,
    Auxillary,
    Reserved,
    ProgramHeader,
    ThreadLocalStorage,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

impl ParseFromEndianess for EntryType {
    fn parse_from_endianess<R: Read>(
        reader: &mut R,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let value = u32::parse_from_endianess(reader, endianess)?;
        Ok(match value {
            0x00000000 => Self::Null,
            0x00000001 => Self::Load,
            0x00000002 => Self::Dynamic,
            0x00000003 => Self::Interpreter,
            0x00000004 => Self::Auxillary,
            0x00000005 => Self::Reserved,
            0x00000006 => Self::ProgramHeader,
            0x00000007 => Self::ThreadLocalStorage,
            0x60000000 => Self::Loos,
            0x6FFFFFFF => Self::Hios,
            0x70000000 => Self::Loproc,
            0x7FFFFFFF => Self::Hiproc,
            other => {
                return Err(ParseError::InvalidValue {
                    value: other.to_bytes(endianess).into(),
                });
            }
        })
    }
}
