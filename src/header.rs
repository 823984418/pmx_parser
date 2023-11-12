use std::fmt::Debug;
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::pmx::Pmx;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum Encoding {
    Utf16Le = 0x00,
    Utf8 = 0x01,
}

impl TryFrom<u8> for Encoding {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Utf16Le),
            0x01 => Ok(Self::Utf8),
            x => Err(PmxError::InvalidEncoding(x)),
        }
    }
}

impl Encoding {
    pub fn read<R: Read>(&self, read: &mut R) -> Result<String, PmxError> {
        let length = read.read_u32::<LittleEndian>()?;
        let mut buffer = vec![0_u8; length as usize];
        read.read_exact(buffer.as_mut_slice())?;
        match self {
            Encoding::Utf16Le => {
                // TODO: use String::from_utf16le when it's stable
                let (str, error) =
                    encoding_rs::UTF_16LE.decode_without_bom_handling(buffer.as_slice());
                if error {
                    return Err(PmxError::EncodingError);
                }
                Ok(str.to_string())
            }
            Encoding::Utf8 => String::from_utf8(buffer).map_err(|_| PmxError::EncodingError),
        }
    }
    pub fn write<W: Write>(&self, write: &mut W, value: &str) -> Result<(), PmxError> {
        match self {
            Encoding::Utf16Le => {
                let buffer = value
                    .encode_utf16()
                    .flat_map(|i| i.to_le_bytes())
                    .collect::<Vec<_>>();
                write.write_u32::<LittleEndian>(buffer.len() as u32)?;
                write.write_all(buffer.as_slice())?;
            }
            Encoding::Utf8 => {
                let buffer = value.as_bytes();
                write.write_u32::<LittleEndian>(buffer.len() as u32)?;
                write.write_all(buffer)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum IndexSize {
    Bit8 = 0x01,
    Bit16 = 0x02,
    Bit32 = 0x04,
}

impl TryFrom<u8> for IndexSize {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(Self::Bit8),
            0x02 => Ok(Self::Bit16),
            0x04 => Ok(Self::Bit32),
            x => Err(PmxError::InvalidIndexSize(x)),
        }
    }
}

pub(crate) trait PmxIndexType: Sized {
    fn read_pmx_index<R: Read>(read: &mut R, size: IndexSize) -> Result<Self, PmxError>;
    fn write_pmx_index<W: Write>(write: &mut W, size: IndexSize, index: Self) -> Result<(), PmxError>;
}

impl PmxIndexType for u32 {
    fn read_pmx_index<R: Read>(read: &mut R, size: IndexSize) -> Result<Self, PmxError> {
        match size {
            IndexSize::Bit8 => Ok(read.read_u8()? as u32),
            IndexSize::Bit16 => Ok(read.read_u16::<LittleEndian>()? as u32),
            IndexSize::Bit32 => Ok(read.read_u32::<LittleEndian>()?),
        }
    }

    fn write_pmx_index<W: Write>(write: &mut W, size: IndexSize, index: Self) -> Result<(), PmxError> {
        match size {
            IndexSize::Bit8 => {
                write.write_u8(index.try_into().map_err(|_| PmxError::IndexError)?)?
            }
            IndexSize::Bit16 => write
                .write_u16::<LittleEndian>(index.try_into().map_err(|_| PmxError::IndexError)?)?,
            IndexSize::Bit32 => write.write_u32::<LittleEndian>(index)?,
        }
        Ok(())
    }
}

impl PmxIndexType for i32 {
    fn read_pmx_index<R: Read>(read: &mut R, size: IndexSize) -> Result<Self, PmxError> {
        match size {
            IndexSize::Bit8 => Ok(read.read_i8()? as i32),
            IndexSize::Bit16 => Ok(read.read_i16::<LittleEndian>()? as i32),
            IndexSize::Bit32 => Ok(read.read_i32::<LittleEndian>()?),
        }
    }

    fn write_pmx_index<W: Write>(write: &mut W, size: IndexSize, index: Self) -> Result<(), PmxError> {
        match size {
            IndexSize::Bit8 => {
                write.write_i8(index.try_into().map_err(|_| PmxError::IndexError)?)?
            }
            IndexSize::Bit16 => write
                .write_i16::<LittleEndian>(index.try_into().map_err(|_| PmxError::IndexError)?)?,
            IndexSize::Bit32 => write.write_i32::<LittleEndian>(index)?,
        }
        Ok(())
    }
}

impl IndexSize {
    pub fn from_count_u(count: u32) -> Self {
        match count {
            0..=0xFE => Self::Bit8,
            0xFF..=0xFFFE => Self::Bit16,
            0xFFFF.. => Self::Bit32,
        }
    }

    pub fn from_count_i(count: u32) -> Self {
        match count {
            0..=0x7E => Self::Bit8,
            0x7F..=0x7FFE => Self::Bit16,
            0x7FFF.. => Self::Bit32,
        }
    }

    #[inline(always)]
    pub(crate) fn read<R: Read, T: PmxIndexType>(self, read: &mut R) -> Result<T, PmxError> {
        T::read_pmx_index(read, self)
    }

    #[inline(always)]
    pub(crate) fn write<W: Write, T: PmxIndexType>(self, write: &mut W, index: T) -> Result<(), PmxError> {
        T::write_pmx_index(write, self, index)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
    pub version: f32,
    pub encoding: Encoding,
    pub vertex_ext_vec4: u8,
    pub vertex_index: IndexSize,
    pub texture_index: IndexSize,
    pub material_index: IndexSize,
    pub bone_index: IndexSize,
    pub morph_index: IndexSize,
    pub rigid_body_index: IndexSize,
    pub unknown_data: Vec<u8>,
}

impl Header {
    pub fn from_best(version: f32, pmx: &Pmx) -> Self {
        Self {
            version,
            encoding: Encoding::Utf16Le,
            vertex_ext_vec4: pmx.vertices.ext_vec4s.len() as u8,
            vertex_index: IndexSize::from_count_i(pmx.vertices.count()),
            texture_index: IndexSize::from_count_u(pmx.textures.count()),
            material_index: IndexSize::from_count_u(pmx.materials.count()),
            bone_index: IndexSize::from_count_u(pmx.bones.count()),
            morph_index: IndexSize::from_count_u(pmx.morphs.count()),
            rigid_body_index: IndexSize::from_count_u(pmx.rigid_bodies.count()),
            unknown_data: vec![],
        }
    }

    pub fn read<R: Read>(read: &mut R) -> Result<Self, PmxError> {
        let magic = read.read_u32::<LittleEndian>()?;
        if magic != 0x20584D50 {
            return Err(PmxError::MagicError);
        }

        let version = read.read_f32::<LittleEndian>()?;
        let global_data_length = read.read_u8()?;
        if global_data_length < 8 {
            return Err(PmxError::GlobalDataError);
        }
        let mut global_data = vec![0_u8; global_data_length as usize];
        read.read_exact(global_data.as_mut_slice())?;
        Ok(Self {
            version,
            encoding: global_data[0].try_into()?,
            vertex_ext_vec4: global_data[1],
            vertex_index: global_data[2].try_into()?,
            texture_index: global_data[3].try_into()?,
            material_index: global_data[4].try_into()?,
            bone_index: global_data[5].try_into()?,
            morph_index: global_data[6].try_into()?,
            rigid_body_index: global_data[7].try_into()?,
            unknown_data: global_data[8..].to_vec(),
        })
    }

    pub fn write<W: Write>(&self, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(0x20584D50)?;
        write.write_f32::<LittleEndian>(self.version)?;
        write.write_u8(self.unknown_data.len() as u8 + 8)?;
        write.write_u8(self.encoding as u8)?;
        write.write_u8(self.vertex_ext_vec4)?;
        write.write_u8(self.vertex_index as u8)?;
        write.write_u8(self.texture_index as u8)?;
        write.write_u8(self.material_index as u8)?;
        write.write_u8(self.bone_index as u8)?;
        write.write_u8(self.morph_index as u8)?;
        write.write_u8(self.rigid_body_index as u8)?;
        write.write_all(self.unknown_data.as_slice())?;
        Ok(())
    }
}
