use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_bool, read_f32x3, read_f32x4, read_vec, write_f32x3, write_f32x4};
use crate::{BoneIndex, MaterialIndex, MorphIndex, RigidBodyIndex, VertexIndex};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Morphs {
    pub morphs: Vec<Morph>,
}

impl Morphs {
    pub fn count(&self) -> u32 {
        self.morphs.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            morphs: read_vec(read, |read| Morph::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.morphs {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Morph {
    pub name: String,
    pub name_en: String,
    pub control_panel: ControlPanel,
    pub morph_data: MorphData,
}

impl Morph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            control_panel: read.read_u8()?.try_into()?,
            morph_data: MorphData::read(header, read)?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write.write_u8(self.control_panel as u8)?;
        self.morph_data.write(header, write)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Ord, Eq)]
#[repr(u8)]
pub enum ControlPanel {
    System = 0x00,
    BottomLeft = 0x01,
    TopLeft = 0x02,
    TopRight = 0x03,
    BottomRight = 0x04,
}

impl TryFrom<u8> for ControlPanel {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::System),
            0x01 => Ok(Self::BottomLeft),
            0x02 => Ok(Self::TopLeft),
            0x03 => Ok(Self::TopRight),
            0x04 => Ok(Self::BottomRight),
            _ => Err(PmxError::ControlPanelError),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum MorphData {
    Group(Vec<GroupMorph>),
    Vertex(Vec<VertexMorph>),
    Bone(Vec<BoneMorph>),
    UV(Vec<UVMorph>),
    UV1(Vec<UVMorph>),
    UV2(Vec<UVMorph>),
    UV3(Vec<UVMorph>),
    UV4(Vec<UVMorph>),
    Material(Vec<MaterialMorph>),
    Flip(Vec<FlipMorph>),
    Impulse(Vec<ImpulseMorph>),
}

impl Debug for MorphData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MorphData::Group(_) => f.write_str("Group"),
            MorphData::Vertex(_) => f.write_str("Vertex"),
            MorphData::Bone(_) => f.write_str("Bone"),
            MorphData::UV(_) => f.write_str("UV"),
            MorphData::UV1(_) => f.write_str("UV1"),
            MorphData::UV2(_) => f.write_str("UV2"),
            MorphData::UV3(_) => f.write_str("UV3"),
            MorphData::UV4(_) => f.write_str("UV4"),
            MorphData::Material(_) => f.write_str("Material"),
            MorphData::Flip(_) => f.write_str("Flip"),
            MorphData::Impulse(_) => f.write_str("Impulse"),
        }
    }
}

impl MorphData {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let t = read.read_u8()?;
        match t {
            0x00 => Ok(MorphData::Group(read_vec(read, |read| {
                GroupMorph::read(header, read)
            })?)),
            0x01 => Ok(MorphData::Vertex(read_vec(read, |read| {
                VertexMorph::read(header, read)
            })?)),
            0x02 => Ok(MorphData::Bone(read_vec(read, |read| {
                BoneMorph::read(header, read)
            })?)),
            0x03 => Ok(MorphData::UV(read_vec(read, |read| {
                UVMorph::read(header, read)
            })?)),
            0x04 => Ok(MorphData::UV1(read_vec(read, |read| {
                UVMorph::read(header, read)
            })?)),
            0x05 => Ok(MorphData::UV2(read_vec(read, |read| {
                UVMorph::read(header, read)
            })?)),
            0x06 => Ok(MorphData::UV3(read_vec(read, |read| {
                UVMorph::read(header, read)
            })?)),
            0x07 => Ok(MorphData::UV4(read_vec(read, |read| {
                UVMorph::read(header, read)
            })?)),
            0x08 => Ok(MorphData::Material(read_vec(read, |read| {
                MaterialMorph::read(header, read)
            })?)),
            0x09 => Ok(MorphData::Flip(read_vec(read, |read| {
                FlipMorph::read(header, read)
            })?)),
            0x0A => Ok(MorphData::Impulse(read_vec(read, |read| {
                ImpulseMorph::read(header, read)
            })?)),
            _ => Err(PmxError::MorphError),
        }
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        match self {
            MorphData::Group(i) => {
                write.write_u8(0x00)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::Vertex(i) => {
                write.write_u8(0x01)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::Bone(i) => {
                write.write_u8(0x02)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::UV(i) => {
                write.write_u8(0x03)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::UV1(i) => {
                write.write_u8(0x04)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::UV2(i) => {
                write.write_u8(0x05)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::UV3(i) => {
                write.write_u8(0x06)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::UV4(i) => {
                write.write_u8(0x07)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::Material(i) => {
                write.write_u8(0x08)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::Flip(i) => {
                write.write_u8(0x09)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
            MorphData::Impulse(i) => {
                write.write_u8(0x0A)?;
                write.write_u32::<LittleEndian>(i.len() as u32)?;
                for x in i {
                    x.write(header, write)?;
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GroupMorph {
    pub morph_index: MorphIndex,
    pub morph_factor: f32,
}

impl GroupMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            morph_index: header.morph_index.read(read)?,
            morph_factor: read.read_f32::<LittleEndian>()?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.morph_index.write(write, self.morph_index)?;
        write.write_f32::<LittleEndian>(self.morph_factor)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VertexMorph {
    pub vertex_index: VertexIndex,
    pub offset: [f32; 3],
}

impl VertexMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            vertex_index: header.vertex_index.read(read)?,
            offset: read_f32x3(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.vertex_index.write(write, self.vertex_index)?;
        write_f32x3(write, self.offset)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BoneMorph {
    pub bone_index: BoneIndex,
    pub translates: [f32; 3],
    pub rotates: [f32; 4],
}

impl BoneMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            bone_index: header.bone_index.read(read)?,
            translates: read_f32x3(read)?,
            rotates: read_f32x4(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.bone_index.write(write, self.bone_index)?;
        write_f32x3(write, self.translates)?;
        write_f32x4(write, self.rotates)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UVMorph {
    pub vertex_index: VertexIndex,
    pub offset: [f32; 4],
}

impl UVMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            vertex_index: header.vertex_index.read(read)?,
            offset: read_f32x4(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.vertex_index.write(write, self.vertex_index)?;
        write_f32x4(write, self.offset)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialMorph {
    pub material_index: MaterialIndex,
    pub formula: u8,
    pub diffuse: [f32; 4],
    pub specular: [f32; 3],
    pub specular_factor: f32,
    pub ambient: [f32; 3],
    pub edge_color: [f32; 4],
    pub edge_size: f32,
    pub texture_factor: [f32; 4],
    pub sphere_texture_factor: [f32; 4],
    pub toon_texture_factor: [f32; 4],
}

impl MaterialMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            material_index: header.material_index.read(read)?,
            formula: read.read_u8()?,
            diffuse: read_f32x4(read)?,
            specular: read_f32x3(read)?,
            specular_factor: 0.0,
            ambient: read_f32x3(read)?,
            edge_color: read_f32x4(read)?,
            edge_size: 0.0,
            texture_factor: read_f32x4(read)?,
            sphere_texture_factor: read_f32x4(read)?,
            toon_texture_factor: read_f32x4(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.material_index.write(write, self.material_index)?;
        write.write_u8(self.formula)?;
        write_f32x4(write, self.diffuse)?;
        write_f32x3(write, self.specular)?;
        write.write_f32::<LittleEndian>(self.specular_factor)?;
        write_f32x3(write, self.ambient)?;
        write_f32x4(write, self.edge_color)?;
        write.write_f32::<LittleEndian>(self.edge_size)?;
        write_f32x4(write, self.texture_factor)?;
        write_f32x4(write, self.sphere_texture_factor)?;
        write_f32x4(write, self.toon_texture_factor)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlipMorph {
    pub morph_index: MorphIndex,
    pub morph_factor: f32,
}

impl FlipMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            morph_index: header.morph_index.read(read)?,
            morph_factor: read.read_f32::<LittleEndian>()?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.morph_index.write(write, self.morph_index)?;
        write.write_f32::<LittleEndian>(self.morph_factor)?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImpulseMorph {
    pub rigid_index: RigidBodyIndex,
    pub is_local: bool,
    pub velocity: [f32; 3],
    pub torque: [f32; 3],
}

impl ImpulseMorph {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            rigid_index: header.rigid_body_index.read(read)?,
            is_local: read_bool(read)?,
            velocity: read_f32x3(read)?,
            torque: read_f32x3(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.rigid_body_index.write(write, self.rigid_index)?;
        write.write_u8(self.is_local as u8)?;
        write_f32x3(write, self.velocity)?;
        write_f32x3(write, self.torque)?;
        Ok(())
    }
}
