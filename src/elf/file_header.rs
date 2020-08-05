use crate::elf::parsing::ParseFromContext;
use crate::elf::parsing::ParseFromEndianess;
use crate::elf::ArchitectureWidth;
use crate::errors::ParseError;
use crate::parsing::*;
use crate::shared::Endianess;
use crate::shared::NativeInteger;
use std::io::Read;

pub struct FileHeader {
    pub width: ArchitectureWidth,
    pub endianess: Endianess,
    pub elf_version: ElfVersion,
    pub os_abi: OsAbi,
    pub object_file_type: ObjectFileType,
    pub instruction_set_arch: InstructionSetArch,
    pub entry_point: NativeInteger,
    pub program_header: NativeInteger,
    pub section_header: NativeInteger,
    pub flags: u32,
    pub file_header_size: u16,
    pub program_header_entry_size: u16,
    pub program_header_entry_count: u16,
    pub section_header_entry_size: u16,
    pub section_header_entry_count: u16,
    pub section_names_entry_index: u16,
}

impl Parse for FileHeader {
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError> {
        let mut buffer = [0; 24];
        if reader.read(&mut buffer)? < 24 {
            return Err(ParseError::MissingData);
        }

        // Ensure magic number is present
        const MAGIC_NUMBER: [u8; 4] = [0x7f, b'E', b'L', b'F'];
        if buffer[0..4] != MAGIC_NUMBER {
            return Err(ParseError::MissingData);
        }

        let width = ArchitectureWidth::parse(reader)?;
        let endianess = Endianess::parse(reader)?;
        let elf_version = ElfVersion::parse(reader)?;
        let os_abi = OsAbi::parse(reader)?;
        // abi_version &buffer[8]
        // pad &buffer[9..16]
        let object_file_type = ObjectFileType::parse_from_endianess(reader, endianess)?;
        let instruction_set_arch = InstructionSetArch::parse_from_endianess(reader, endianess)?;
        // version &buffer[20..24]
        let entry_point = NativeInteger::parse_from_context(reader, endianess, width)?;
        let program_header = NativeInteger::parse_from_context(reader, endianess, width)?;
        let section_header = NativeInteger::parse_from_context(reader, endianess, width)?;

        let flags = u32::parse_from_endianess(reader, endianess)?;
        let file_header_size = u16::parse_from_endianess(reader, endianess)?;
        let program_header_entry_size = u16::parse_from_endianess(reader, endianess)?;
        let program_header_entry_count = u16::parse_from_endianess(reader, endianess)?;
        let section_header_entry_size = u16::parse_from_endianess(reader, endianess)?;
        let section_header_entry_count = u16::parse_from_endianess(reader, endianess)?;
        let section_names_entry_index = u16::parse_from_endianess(reader, endianess)?;

        Ok(FileHeader {
            width,
            endianess,
            elf_version,
            os_abi,
            object_file_type,
            instruction_set_arch,
            entry_point,
            program_header,
            section_header,
            flags,
            file_header_size,
            program_header_entry_count,
            program_header_entry_size,
            section_header_entry_count,
            section_header_entry_size,
            section_names_entry_index,
        })
    }
}

pub enum ElfVersion {
    One,
}

impl Parse for ElfVersion {
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError> {
        let val = u8::parse(reader)?;
        Ok(match val {
            1 => Self::One,
            other => {
                return Err(ParseError::InvalidValue {
                    value: val.to_ne_bytes().into(),
                })
            }
        })
    }
}

pub enum OsAbi {
    SystemV,
    HpUx,
    NetBsd,
    Linux,
    GnuHurd,
    Solaris,
    Aix,
    Irix,
    FreeBsd,
    Tru64,
    NovellModesto,
    OpenBsd,
    OpenVms,
    NonStopKernel,
    Aros,
    FenixOs,
    CloudAbi,
    StratusTechnologiesOpenVos,
}

impl Parse for OsAbi {
    fn parse(reader: &mut impl Read) -> Result<Self, ParseError> {
        let val = u8::parse(reader)?;
        Ok(match val {
            0x00 => Self::SystemV,
            0x01 => Self::HpUx,
            0x02 => Self::NetBsd,
            0x03 => Self::Linux,
            0x04 => Self::GnuHurd,
            0x06 => Self::Solaris,
            0x07 => Self::Aix,
            0x08 => Self::Irix,
            0x09 => Self::FreeBsd,
            0x0A => Self::Tru64,
            0x0B => Self::NovellModesto,
            0x0C => Self::OpenBsd,
            0x0D => Self::OpenVms,
            0x0E => Self::NonStopKernel,
            0x0F => Self::Aros,
            0x10 => Self::FenixOs,
            0x11 => Self::CloudAbi,
            0x12 => Self::StratusTechnologiesOpenVos,
            other => {
                return Err(ParseError::InvalidValue {
                    value: Box::new([other]),
                })
            }
        })
    }
}

pub enum ObjectFileType {
    None,
    Rel,
    Exec,
    Dyn,
    Core,
    Loos,
    Hios,
    Loproc,
    Hiproc,
}

impl ParseFromEndianess for ObjectFileType {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let val = u16::parse_from_endianess(reader, endianess)?;
        Ok(match val {
            0x00 => Self::None,
            0x01 => Self::Rel,
            0x02 => Self::Exec,
            0x03 => Self::Dyn,
            0x04 => Self::Core,
            0xfe00 => Self::Loos,
            0xfeff => Self::Hios,
            0xff00 => Self::Loproc,
            0xffff => Self::Hiproc,
            other => {
                return Err(ParseError::InvalidValue {
                    value: other.to_bytes(endianess).into(),
                });
            }
        })
    }
}

pub enum InstructionSetArch {
    NoSpecific,
    AttWe32100,
    Sparc,
    X86,
    Motorolla68000,
    Motorolla88000,
    IntelMcu,
    Intel80860,
    Mips,
    IbmSystem370,
    MipsRs3000LittleEndian,
    Reserved,
    HewlettPackardPaRisc,
    Intel80960,
    PowerPc,
    PowerPc64,
    S390,
    Arm,
    SuperH,
    IA64,
    Amd64,
    Tms320C6000,
    Arm64,
    RiscV,
}

impl ParseFromEndianess for InstructionSetArch {
    fn parse_from_endianess(
        reader: &mut impl Read,
        endianess: Endianess,
    ) -> Result<Self, ParseError> {
        let val = u16::parse_from_endianess(reader, endianess)?;
        Ok(match val {
            0x00 => Self::NoSpecific,
            0x01 => Self::AttWe32100,
            0x02 => Self::Sparc,
            0x03 => Self::X86,
            0x04 => Self::Motorolla68000,
            0x05 => Self::Motorolla88000,
            0x06 => Self::IntelMcu,
            0x07 => Self::Intel80860,
            0x08 => Self::Mips,
            0x09 => Self::IbmSystem370,
            0x0A => Self::MipsRs3000LittleEndian,
            0x0B..=0x0D => Self::Reserved,
            0x0E => Self::HewlettPackardPaRisc,
            0x0F => Self::Reserved,
            0x13 => Self::Intel80960,
            0x14 => Self::PowerPc,
            0x15 => Self::PowerPc64,
            0x16 => Self::S390,
            0x28 => Self::Arm,
            0x2A => Self::SuperH,
            0x32 => Self::IA64,
            0x3E => Self::Amd64,
            0x8C => Self::Tms320C6000,
            0xB7 => Self::Arm64,
            0xF3 => Self::RiscV,
            other => {
                return Err(ParseError::InvalidValue {
                    value: other.to_bytes(endianess).into(),
                })
            }
        })
    }
}
