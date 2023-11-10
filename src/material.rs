use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_f32x3, read_f32x4, read_vec, write_f32x3, write_f32x4};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Materials {
    materials: Vec<Material>,
}

impl Materials {
    pub fn count(&self) -> u32 {
        self.materials.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            materials: read_vec(read, |read| Material::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.materials {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub name: String,
    pub name_en: String,
    pub diffuse: [f32; 4],
    pub specular: [f32; 4],
    pub ambient: [f32; 3],
    pub flags: MaterialFlags,
    pub edge_color: [f32; 4],
    pub edge_size: f32,
    pub texture_index: u32,
    pub env_texture_index: u32,
    pub mix: Mix,
    pub toon_texture: ToonTexture,
    pub comment: String,
    pub element_count: u32,
}

impl Material {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            diffuse: read_f32x4(read)?,
            specular: read_f32x4(read)?,
            ambient: read_f32x3(read)?,
            flags: MaterialFlags::from_bits_retain(read.read_u8()?),
            edge_color: read_f32x4(read)?,
            edge_size: read.read_f32::<LittleEndian>()?,
            texture_index: header.texture_index.read(read)?,
            env_texture_index: header.texture_index.read(read)?,
            mix: Mix::try_from(read.read_u8()?)?,
            toon_texture: ToonTexture::read(header, read)?,
            comment: header.encoding.read(read)?,
            element_count: read.read_u32::<LittleEndian>()?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write_f32x4(write, self.diffuse)?;
        write_f32x4(write, self.specular)?;
        write_f32x3(write, self.ambient)?;
        write.write_u8(self.flags.bits())?;
        write_f32x4(write, self.edge_color)?;
        write.write_f32::<LittleEndian>(self.edge_size)?;
        header.texture_index.write(write, self.texture_index)?;
        header.texture_index.write(write, self.env_texture_index)?;
        write.write_u8(self.mix as u8)?;
        self.toon_texture.write(header, write)?;
        header.encoding.write(write, self.comment.as_str())?;
        write.write_u32::<LittleEndian>(self.element_count)?;
        Ok(())
    }
}

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct MaterialFlags: u8 {
        const DISABLE_CULLING = 0x01;
        const GROUND_SHADOW = 0x02;
        const DRAW_SHADOW = 0x04;
        const RECEIVE_SHADOW = 0x08;
        const HAS_EDGE = 0x10;
        const VERTEX_COLOR = 0x20;
        const POINT_DRAW = 0x40;
        const LINE_DRAW =  0x80;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Mix {
    No = 0x00,
    Mul = 0x01,
    Add = 0x02,
    SubTexture = 0x03,
}

impl TryFrom<u8> for Mix {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::No),
            0x01 => Ok(Self::Mul),
            0x02 => Ok(Self::Add),
            0x03 => Ok(Self::SubTexture),
            _ => Err(PmxError::MixError),
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ToonTexture {
    TextureIndex(u32),
    CommonIndex(u8),
}

impl ToonTexture {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let t = read.read_u8()?;
        match t {
            0x00 => Ok(Self::TextureIndex(header.texture_index.read(read)?)),
            0x01 => Ok(Self::CommonIndex(read.read_u8()?)),
            _ => Err(PmxError::ToonError),
        }
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        match *self {
            ToonTexture::TextureIndex(texture_index) => {
                write.write_u8(0x00)?;
                header.texture_index.write(write, texture_index)?
            }
            ToonTexture::CommonIndex(common_index) => {
                write.write_u8(0x01)?;
                write.write_u8(common_index)?;
            }
        }
        Ok(())
    }
}
