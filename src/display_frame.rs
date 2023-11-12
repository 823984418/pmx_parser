use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_bool, read_vec};

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct DisplayFrames {
    pub display_frames: Vec<DisplayFrame>,
}

impl DisplayFrames {
    pub fn count(&self) -> u32 {
        self.display_frames.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            display_frames: read_vec(read, |read| DisplayFrame::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.display_frames {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DisplayFrame {
    pub name: String,
    pub name_en: String,
    pub is_special: bool,
    pub items: Vec<DisplayFrameItem>,
}

impl DisplayFrame {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            is_special: read_bool(read)?,
            items: read_vec(read, |read| DisplayFrameItem::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write.write_u8(self.is_special as u8)?;
        write.write_u32::<LittleEndian>(self.items.len() as u32)?;
        for i in &self.items {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DisplayFrameItem {
    BoneIndex(u32),
    MorphIndex(u32),
}

impl DisplayFrameItem {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let t = read.read_u8()?;
        match t {
            0 => Ok(Self::BoneIndex(header.bone_index.read_i(read)?)),
            1 => Ok(Self::MorphIndex(header.morph_index.read_i(read)?)),
            _ => Err(PmxError::DisplayFrameError),
        }
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        match *self {
            DisplayFrameItem::BoneIndex(i) => {
                write.write_u8(0x00)?;
                header.bone_index.write(write, i)?;
            }
            DisplayFrameItem::MorphIndex(i) => {
                write.write_u8(0x01)?;
                header.morph_index.write(write, i)?;
            }
        }
        Ok(())
    }
}
