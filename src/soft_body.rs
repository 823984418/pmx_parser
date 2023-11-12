use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_bool, read_vec};
use crate::{MaterialIndex, RigidBodyIndex, VertexIndex};

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SoftBodies {
    pub soft_bodies: Vec<SoftBody>,
}

impl SoftBodies {
    pub fn is_empty(&self) -> bool {
        self.soft_bodies.is_empty()
    }
    pub fn count(&self) -> u32 {
        self.soft_bodies.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(if header.version >= 2.1 * (1.0 - f32::EPSILON) {
            Self {
                soft_bodies: read_vec(read, |read| SoftBody::read(header, read))?,
            }
        } else {
            Self::default()
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        if header.version >= 2.1 * (1.0 - f32::EPSILON) {
            write.write_u32::<LittleEndian>(self.count())?;
            for i in &self.soft_bodies {
                i.write(header, write)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoftBody {
    pub name: String,
    pub name_en: String,
    pub form: SoftBodyForm, //i8
    pub material_index: MaterialIndex,
    pub group: u8,
    pub un_collision_group_flag: u16,
    pub bit_flag: u8,
    pub b_link_create_distance: i32,
    pub clusters: u32,
    pub mass: f32,
    pub collision_margin: f32,
    pub aero_model: SoftBodyAeroModel, //i32
    ///config
    pub vcf: f32,
    pub dp: f32,
    pub dg: f32,
    pub lf: f32,
    pub pr: f32,
    pub vc: f32,
    pub df: f32,
    pub mt: f32,
    pub chr: f32,
    pub khr: f32,
    pub shr: f32,
    pub ahr: f32,
    ///cluster
    pub srhr_cl: f32,
    pub skhr_cl: f32,
    pub sshr_cl: f32,
    pub sr_splt_cl: f32,
    pub sk_splt_cl: f32,
    pub ss_splt_cl: f32,
    ///iteration
    pub v_it: u32,
    pub p_it: u32,
    pub d_it: u32,
    pub c_it: u32,
    ///material
    pub lst: f32,
    pub ast: f32,
    pub vst: f32,
    pub anchor_rigid: Vec<SoftBodyAnchorRigid>,
    pub pin_vertex_index: Vec<VertexIndex>,
}

impl SoftBody {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            form: SoftBodyForm::try_from(read.read_u8()?)?,
            material_index: header.material_index.read(read)?,
            group: read.read_u8()?,
            un_collision_group_flag: read.read_u16::<LittleEndian>()?,
            bit_flag: read.read_u8()?,
            b_link_create_distance: read.read_i32::<LittleEndian>()?,
            clusters: read.read_u32::<LittleEndian>()?,
            mass: read.read_f32::<LittleEndian>()?,
            collision_margin: read.read_f32::<LittleEndian>()?,
            aero_model: SoftBodyAeroModel::try_from(read.read_u32::<LittleEndian>()?)?,
            vcf: read.read_f32::<LittleEndian>()?,
            dp: read.read_f32::<LittleEndian>()?,
            dg: read.read_f32::<LittleEndian>()?,
            lf: read.read_f32::<LittleEndian>()?,
            pr: read.read_f32::<LittleEndian>()?,
            vc: read.read_f32::<LittleEndian>()?,
            df: read.read_f32::<LittleEndian>()?,
            mt: read.read_f32::<LittleEndian>()?,
            chr: read.read_f32::<LittleEndian>()?,
            khr: read.read_f32::<LittleEndian>()?,
            shr: read.read_f32::<LittleEndian>()?,
            ahr: read.read_f32::<LittleEndian>()?,
            srhr_cl: read.read_f32::<LittleEndian>()?,
            skhr_cl: read.read_f32::<LittleEndian>()?,
            sshr_cl: read.read_f32::<LittleEndian>()?,
            sr_splt_cl: read.read_f32::<LittleEndian>()?,
            sk_splt_cl: read.read_f32::<LittleEndian>()?,
            ss_splt_cl: read.read_f32::<LittleEndian>()?,
            v_it: read.read_u32::<LittleEndian>()?,
            p_it: read.read_u32::<LittleEndian>()?,
            d_it: read.read_u32::<LittleEndian>()?,
            c_it: read.read_u32::<LittleEndian>()?,
            lst: read.read_f32::<LittleEndian>()?,
            ast: read.read_f32::<LittleEndian>()?,
            vst: read.read_f32::<LittleEndian>()?,
            anchor_rigid: read_vec(read, |read| SoftBodyAnchorRigid::read(header, read))?,
            pin_vertex_index: read_vec(read, |read| header.vertex_index.read(read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write.write_u8(self.form as u8)?;
        header.material_index.write(write, self.material_index)?;
        write.write_u8(self.group)?;
        write.write_u16::<LittleEndian>(self.un_collision_group_flag)?;
        write.write_u8(self.bit_flag)?;
        write.write_u32::<LittleEndian>(self.clusters)?;
        write.write_f32::<LittleEndian>(self.mass)?;
        write.write_f32::<LittleEndian>(self.collision_margin)?;
        write.write_u32::<LittleEndian>(self.aero_model as u32)?;
        write.write_f32::<LittleEndian>(self.vcf)?;
        write.write_f32::<LittleEndian>(self.dp)?;
        write.write_f32::<LittleEndian>(self.dg)?;
        write.write_f32::<LittleEndian>(self.lf)?;
        write.write_f32::<LittleEndian>(self.pr)?;
        write.write_f32::<LittleEndian>(self.vc)?;
        write.write_f32::<LittleEndian>(self.df)?;
        write.write_f32::<LittleEndian>(self.mt)?;
        write.write_f32::<LittleEndian>(self.chr)?;
        write.write_f32::<LittleEndian>(self.khr)?;
        write.write_f32::<LittleEndian>(self.shr)?;
        write.write_f32::<LittleEndian>(self.ahr)?;
        write.write_f32::<LittleEndian>(self.srhr_cl)?;
        write.write_f32::<LittleEndian>(self.skhr_cl)?;
        write.write_f32::<LittleEndian>(self.sshr_cl)?;
        write.write_f32::<LittleEndian>(self.sr_splt_cl)?;
        write.write_f32::<LittleEndian>(self.sk_splt_cl)?;
        write.write_f32::<LittleEndian>(self.ss_splt_cl)?;
        write.write_u32::<LittleEndian>(self.v_it)?;
        write.write_u32::<LittleEndian>(self.p_it)?;
        write.write_u32::<LittleEndian>(self.d_it)?;
        write.write_u32::<LittleEndian>(self.c_it)?;
        write.write_f32::<LittleEndian>(self.lst)?;
        write.write_f32::<LittleEndian>(self.ast)?;
        write.write_f32::<LittleEndian>(self.vst)?;
        write.write_u32::<LittleEndian>(self.anchor_rigid.len() as u32)?;
        for i in &self.anchor_rigid {
            i.write(header, write)?;
        }
        write.write_u32::<LittleEndian>(self.pin_vertex_index.len() as u32)?;
        for &i in &self.pin_vertex_index {
            header.vertex_index.write(write, i)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum SoftBodyForm {
    TriMesh = 0x00,
    Rope = 0x01,
}

impl TryFrom<u8> for SoftBodyForm {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::TriMesh),
            0x01 => Ok(Self::Rope),
            _ => Err(PmxError::SoftBodyFormError),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u32)]
pub enum SoftBodyAeroModel {
    VPoint = 0x00,
    VTwoSide = 0x01,
    VOneSided = 0x02,
    FTwoSided = 0x03,
    FOneSided = 0x04,
}

impl TryFrom<u32> for SoftBodyAeroModel {
    type Error = PmxError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::VPoint),
            0x01 => Ok(Self::VTwoSide),
            0x02 => Ok(Self::VOneSided),
            0x03 => Ok(Self::FTwoSided),
            0x04 => Ok(Self::FOneSided),
            _ => Err(PmxError::SoftBodyAeroModelError),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SoftBodyAnchorRigid {
    pub rigid_index: RigidBodyIndex,
    pub vertex_index: VertexIndex,
    pub near_mode: bool,
}

impl SoftBodyAnchorRigid {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            rigid_index: header.rigid_body_index.read(read)?,
            vertex_index: header.vertex_index.read(read)?,
            near_mode: read_bool(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.rigid_body_index.write(write, self.rigid_index)?;
        header.vertex_index.write(write, self.vertex_index)?;
        write.write_u8(self.near_mode as u8)?;
        Ok(())
    }
}
