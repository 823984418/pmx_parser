use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_f32x3, write_f32x3};

#[derive(Default, Clone, PartialEq)]
pub struct Vertices {
    pub position3s: Vec<f32>,
    pub normal3s: Vec<f32>,
    pub uv2s: Vec<f32>,
    pub ext_vec4s: Vec<Vec<f32>>,
    pub skins: Vec<Skin>,
    pub edges: Vec<f32>,
}

impl Debug for Vertices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Vertices");
        s.field("count", &self.count());
        s.finish()
    }
}

impl Vertices {
    pub fn count(&self) -> u32 {
        (self.position3s.len() / 3) as u32
    }

    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let count = read.read_u32::<LittleEndian>()? as usize;
        let mut position3s = Vec::with_capacity(count * 3);
        let mut normal3s = Vec::with_capacity(count * 3);
        let mut uv2s = Vec::with_capacity(count * 2);
        let mut skins = Vec::with_capacity(count);
        let mut ext_vec4s = Vec::with_capacity(header.vertex_ext_vec4 as usize);
        for _ in 0..header.vertex_ext_vec4 {
            ext_vec4s.push(Vec::with_capacity(count * 4));
        }
        let mut edges = Vec::with_capacity(count);

        for _ in 0..count {
            for _ in 0..3 {
                position3s.push(read.read_f32::<LittleEndian>()?);
            }
            for _ in 0..3 {
                normal3s.push(read.read_f32::<LittleEndian>()?);
            }
            for _ in 0..2 {
                uv2s.push(read.read_f32::<LittleEndian>()?);
            }
            for e in &mut ext_vec4s {
                for _ in 0..4 {
                    e.push(read.read_f32::<LittleEndian>()?);
                }
            }
            skins.push(Skin::read(header, read)?);
            edges.push(read.read_f32::<LittleEndian>()?);
        }

        Ok(Self {
            position3s,
            normal3s,
            uv2s,
            skins,
            ext_vec4s,
            edges,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        let count = self.count() as usize;
        let ext_vec4s = &self.ext_vec4s[..header.vertex_ext_vec4 as usize];
        if self.position3s.len() != count * 3
            || self.normal3s.len() != count * 3
            || self.uv2s.len() != count * 2
            || ext_vec4s.iter().any(|i| i.len() != count * 4)
            || self.skins.len() != count
            || self.edges.len() != count
        {
            return Err(PmxError::VertexCountError);
        }
        write.write_u32::<LittleEndian>(self.count())?;
        for index in 0..count {
            for i in 0..3 {
                write.write_f32::<LittleEndian>(self.position3s[index * 3 + i])?;
            }
            for i in 0..3 {
                write.write_f32::<LittleEndian>(self.normal3s[index * 3 + i])?;
            }
            for i in 0..2 {
                write.write_f32::<LittleEndian>(self.uv2s[index * 2 + i])?;
            }
            self.skins[index].write(header, write)?;
            write.write_f32::<LittleEndian>(self.uv2s[index])?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Skin {
    /// a bone with weight 1.0
    BDEF1 { bone_index: u32 },
    /// 2 bones with normalized weight
    /// * bone_weight_1 : weight of bone_index_1
    /// * bone_weight_2 : 1.0 - bone_weight_1
    BDEF2 {
        bone_index_1: u32,
        bone_index_2: u32,
        bone_weight_1: f32,
    },
    /// 4 bones without normalized weights guaranty.
    BDEF4 {
        bone_index_1: u32,
        bone_index_2: u32,
        bone_index_3: u32,
        bone_index_4: u32,
        bone_weight_1: f32,
        bone_weight_2: f32,
        bone_weight_3: f32,
        bone_weight_4: f32,
    },
    /// spherical deforming bones
    SDEF {
        bone_index_1: u32,
        bone_index_2: u32,
        bone_weight_1: f32,
        sdef_c: [f32; 3],
        sdef_r0: [f32; 3],
        sdef_r1: [f32; 3],
    },
    /// DualQuaternion deforming
    QDEF {
        bone_index_1: u32,
        bone_index_2: u32,
        bone_index_3: u32,
        bone_index_4: u32,
        bone_weight_1: f32,
        bone_weight_2: f32,
        bone_weight_3: f32,
        bone_weight_4: f32,
    },
}

impl Skin {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let t = read.read_u8()?;
        match t {
            0 => Ok(Skin::BDEF1 {
                bone_index: header.bone_index.read_i(read)?,
            }),
            1 => Ok(Skin::BDEF2 {
                bone_index_1: header.bone_index.read_i(read)?,
                bone_index_2: header.bone_index.read_i(read)?,
                bone_weight_1: read.read_f32::<LittleEndian>()?,
            }),
            2 => Ok(Skin::BDEF4 {
                bone_index_1: header.bone_index.read_i(read)?,
                bone_index_2: header.bone_index.read_i(read)?,
                bone_index_3: header.bone_index.read_i(read)?,
                bone_index_4: header.bone_index.read_i(read)?,
                bone_weight_1: read.read_f32::<LittleEndian>()?,
                bone_weight_2: read.read_f32::<LittleEndian>()?,
                bone_weight_3: read.read_f32::<LittleEndian>()?,
                bone_weight_4: read.read_f32::<LittleEndian>()?,
            }),
            3 => Ok(Skin::SDEF {
                bone_index_1: header.bone_index.read_i(read)?,
                bone_index_2: header.bone_index.read_i(read)?,
                bone_weight_1: read.read_f32::<LittleEndian>()?,
                sdef_c: read_f32x3(read)?,
                sdef_r0: read_f32x3(read)?,
                sdef_r1: read_f32x3(read)?,
            }),
            4 => Ok(Skin::QDEF {
                bone_index_1: header.bone_index.read_i(read)?,
                bone_index_2: header.bone_index.read_i(read)?,
                bone_index_3: header.bone_index.read_i(read)?,
                bone_index_4: header.bone_index.read_i(read)?,
                bone_weight_1: read.read_f32::<LittleEndian>()?,
                bone_weight_2: read.read_f32::<LittleEndian>()?,
                bone_weight_3: read.read_f32::<LittleEndian>()?,
                bone_weight_4: read.read_f32::<LittleEndian>()?,
            }),
            _ => Err(PmxError::SkinError),
        }
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        let bone_index_size = header.bone_index;
        match *self {
            Skin::BDEF1 { bone_index } => {
                write.write_u8(0)?;
                bone_index_size.write(write, bone_index)?;
            }
            Skin::BDEF2 {
                bone_index_1,
                bone_index_2,
                bone_weight_1,
            } => {
                write.write_u8(1)?;
                bone_index_size.write(write, bone_index_1)?;
                bone_index_size.write(write, bone_index_2)?;
                write.write_f32::<LittleEndian>(bone_weight_1)?;
            }
            Skin::BDEF4 {
                bone_index_1,
                bone_index_2,
                bone_index_3,
                bone_index_4,
                bone_weight_1,
                bone_weight_2,
                bone_weight_3,
                bone_weight_4,
            } => {
                write.write_u8(2)?;
                bone_index_size.write(write, bone_index_1)?;
                bone_index_size.write(write, bone_index_2)?;
                bone_index_size.write(write, bone_index_3)?;
                bone_index_size.write(write, bone_index_4)?;
                write.write_f32::<LittleEndian>(bone_weight_1)?;
                write.write_f32::<LittleEndian>(bone_weight_2)?;
                write.write_f32::<LittleEndian>(bone_weight_3)?;
                write.write_f32::<LittleEndian>(bone_weight_4)?;
            }
            Skin::SDEF {
                bone_index_1,
                bone_index_2,
                bone_weight_1,
                sdef_c,
                sdef_r0,
                sdef_r1,
            } => {
                write.write_u8(3)?;
                bone_index_size.write(write, bone_index_1)?;
                bone_index_size.write(write, bone_index_2)?;
                write.write_f32::<LittleEndian>(bone_weight_1)?;
                write_f32x3(write, sdef_c)?;
                write_f32x3(write, sdef_r0)?;
                write_f32x3(write, sdef_r1)?;
            }
            Skin::QDEF {
                bone_index_1,
                bone_index_2,
                bone_index_3,
                bone_index_4,
                bone_weight_1,
                bone_weight_2,
                bone_weight_3,
                bone_weight_4,
            } => {
                write.write_u8(4)?;
                bone_index_size.write(write, bone_index_1)?;
                bone_index_size.write(write, bone_index_2)?;
                bone_index_size.write(write, bone_index_3)?;
                bone_index_size.write(write, bone_index_4)?;
                write.write_f32::<LittleEndian>(bone_weight_1)?;
                write.write_f32::<LittleEndian>(bone_weight_2)?;
                write.write_f32::<LittleEndian>(bone_weight_3)?;
                write.write_f32::<LittleEndian>(bone_weight_4)?;
            }
        }
        Ok(())
    }
}
