use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::BoneIndex;

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_f32x3, read_vec, write_f32x3};

#[derive(Debug, Default, PartialEq, Clone)]
pub struct RigidBodies {
    pub rigid_bodies: Vec<RigidBody>,
}

impl RigidBodies {
    pub fn count(&self) -> u32 {
        self.rigid_bodies.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            rigid_bodies: read_vec(read, |read| RigidBody::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.rigid_bodies {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct RigidBody {
    pub name: String,
    pub name_en: String,
    pub bone_index: BoneIndex,
    pub group: u8,
    pub un_collision_group_flag: u16,
    pub form: RigidForm,
    pub size: [f32; 3],
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub mass: f32,
    pub move_resist: f32,
    pub rotation_resist: f32,
    pub repulsion: f32,
    pub friction: f32,
    pub calc_method: RigidCalcMethod,
}

impl Debug for RigidBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("RigidBody");
        s.field("name", &self.name);
        s.finish()
    }
}

impl RigidBody {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            bone_index: header.bone_index.read(read)?,
            group: read.read_u8()?,
            un_collision_group_flag: read.read_u16::<LittleEndian>()?,
            form: RigidForm::try_from(read.read_u8()?)?,
            size: read_f32x3(read)?,
            position: read_f32x3(read)?,
            rotation: read_f32x3(read)?,
            mass: read.read_f32::<LittleEndian>()?,
            move_resist: read.read_f32::<LittleEndian>()?,
            rotation_resist: read.read_f32::<LittleEndian>()?,
            repulsion: read.read_f32::<LittleEndian>()?,
            friction: read.read_f32::<LittleEndian>()?,
            calc_method: RigidCalcMethod::try_from(read.read_u8()?)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        header.bone_index.write(write, self.bone_index)?;
        write.write_u8(self.group)?;
        write.write_u16::<LittleEndian>(self.un_collision_group_flag)?;
        write.write_u8(self.form as u8)?;
        write_f32x3(write, self.size)?;
        write_f32x3(write, self.position)?;
        write_f32x3(write, self.rotation)?;
        write.write_f32::<LittleEndian>(self.mass)?;
        write.write_f32::<LittleEndian>(self.move_resist)?;
        write.write_f32::<LittleEndian>(self.rotation_resist)?;
        write.write_f32::<LittleEndian>(self.repulsion)?;
        write.write_f32::<LittleEndian>(self.friction)?;
        write.write_u8(self.calc_method as u8)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum RigidForm {
    Sphere = 0x00,
    Box = 0x01,
    Capsule = 0x02,
}

impl TryFrom<u8> for RigidForm {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Sphere),
            0x01 => Ok(Self::Box),
            0x02 => Ok(Self::Capsule),
            _ => Err(PmxError::RigidFormError),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum RigidCalcMethod {
    Static = 0x00,
    Dynamic = 0x01,
    DynamicWithBonePosition = 0x02,
}

impl TryFrom<u8> for RigidCalcMethod {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Static),
            0x01 => Ok(Self::Dynamic),
            0x02 => Ok(Self::DynamicWithBonePosition),
            _ => Err(PmxError::RigidCalcMethodError),
        }
    }
}
