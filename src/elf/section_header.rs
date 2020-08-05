use crate::elf::parsing::ParseFromContext;
use crate::elf::parsing::ParseFromEndianess;
use crate::elf::parsing::ParseFromWorkingContext;
use crate::elf::ArchitectureWidth;
use crate::elf::Endianess;
use crate::elf::FileHeader;
use crate::elf::NativeInteger;
use crate::errors::ParseError;
use crate::parsing::*;
use std::io::Read;

pub struct SectionHeader {
    entries: Vec<SectionHeaderEntry>,
}

impl SectionHeader {
    pub fn parse_entries<'a>(
        reader: &'a mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
        working_context: &FileHeader,
    ) -> impl Iterator<Item = Result<SectionHeaderEntry, ParseError>> + 'a {
        (0..working_context.section_header_entry_count)
            .map(move |_| SectionHeaderEntry::parse_from_context(reader, endianess, arch_width))
    }
}

impl ParseFromWorkingContext<FileHeader> for SectionHeader {
    fn parse_from_working_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
        working_context: &FileHeader,
    ) -> Result<Self, ParseError> {
        Self::parse_entries(reader, endianess, arch_width, working_context)
            .collect::<Result<_, _>>()
            .map(|entries| Self { entries })
    }
}

pub struct SectionHeaderEntry {
    pub name_offset: u32,
    pub entry_type: EntryType,
    pub flags: Flags,
    pub addr: NativeInteger,
    pub offset: NativeInteger,
    pub size: NativeInteger,
    pub associated_section: u32,
    pub section_info: u32,
    pub alignment: NativeInteger,
    pub entry_size: NativeInteger,
}

impl ParseFromContext for SectionHeaderEntry {
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError> {
        let name_offset = u32::parse_from_context(reader, endianess, arch_width)?;
        let entry_type = EntryType::parse_from_context(reader, endianess, arch_width)?;
        let flags = Flags::parse_from_context(reader, endianess, arch_width)?;
        let addr = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let offset = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let size = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let associated_section = u32::parse_from_context(reader, endianess, arch_width)?;
        let section_info = u32::parse_from_context(reader, endianess, arch_width)?;
        let alignment = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        let entry_size = NativeInteger::parse_from_context(reader, endianess, arch_width)?;

        Ok(Self {
            name_offset,
            entry_type,
            flags,
            addr,
            offset,
            size,
            associated_section,
            section_info,
            alignment,
            entry_size,
        })
    }
}

pub enum EntryType {
    Null,
    ProgramData,
    SymbolTable,
    StringTable,
    RelocationEntriesWithAddens,
    SymbolHashTable,
    DynamicLinkingInformation,
    Notes,
    ProgramSpaceNoData,
    RelocationEntriesNoAddens,
    Reserved,
    DynamicLinkerSymbolTable,
    ArrayOfConstructors,
    ArrayOfDestructors,
    ArrayOfPreConstructors,
    SectionGroup,
    ExtendedSectionIndices,
    NumberOfDefinedTypes,
    StartOsSpecific,
}

impl ParseFromEndianess for EntryType {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let val = u32::parse_from_endianess(reader, endianess)?;
        Ok(match val {
            0x00000000 => EntryType::Null,
            0x00000001 => EntryType::ProgramData,
            0x00000002 => EntryType::SymbolTable,
            0x00000003 => EntryType::StringTable,
            0x00000004 => EntryType::RelocationEntriesWithAddens,
            0x00000005 => EntryType::SymbolHashTable,
            0x00000006 => EntryType::DynamicLinkingInformation,
            0x00000007 => EntryType::Notes,
            0x00000008 => EntryType::ProgramSpaceNoData,
            0x00000009 => EntryType::RelocationEntriesNoAddens,
            0x0000000A => EntryType::Reserved,
            0x0000000B => EntryType::DynamicLinkerSymbolTable,
            0x0000000E => EntryType::ArrayOfConstructors,
            0x0000000F => EntryType::ArrayOfDestructors,
            0x00000010 => EntryType::ArrayOfPreConstructors,
            0x00000011 => EntryType::SectionGroup,
            0x00000012 => EntryType::ExtendedSectionIndices,
            0x00000013 => EntryType::NumberOfDefinedTypes,
            0x60000000 => EntryType::StartOsSpecific,
            other => {
                return Err(ParseError::InvalidValue {
                    value: other.to_bytes(endianess).into(),
                });
            }
        })
    }
}

pub struct Flags(NativeInteger);

impl ParseFromContext for Flags {
    fn parse_from_context(
        reader: &mut impl Read,
        endianess: Endianess,
        arch_width: ArchitectureWidth,
    ) -> Result<Self, ParseError> {
        let val = NativeInteger::parse_from_context(reader, endianess, arch_width)?;
        Ok(Self(val))
    }
}
