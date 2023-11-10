use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_f32x3, read_vec, write_f32x3};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Joints {
    pub joints: Vec<Joint>,
}

impl Joints {
    pub fn count(&self) -> u32 {
        self.joints.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            joints: read_vec(read, |read| Joint::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.joints {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Joint {
    pub name: String,
    pub name_en: String,
    pub joint_type: JointType,
    pub a_rigid_index: u32,
    pub b_rigid_index: u32,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub move_limit_down: [f32; 3],
    pub move_limit_up: [f32; 3],
    pub rotation_limit_down: [f32; 3],
    pub rotation_limit_up: [f32; 3],
    pub spring_const_move: [f32; 3],
    pub spring_const_rotation: [f32; 3],
}

impl Debug for Joint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Joint");
        s.field("name", &self.name);
        s.finish()
    }
}

impl Joint {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            joint_type: JointType::try_from(read.read_u8()?)?,
            a_rigid_index: header.rigid_body_index.read(read)?,
            b_rigid_index: header.rigid_body_index.read(read)?,
            position: read_f32x3(read)?,
            rotation: read_f32x3(read)?,
            move_limit_down: read_f32x3(read)?,
            move_limit_up: read_f32x3(read)?,
            rotation_limit_down: read_f32x3(read)?,
            rotation_limit_up: read_f32x3(read)?,
            spring_const_move: read_f32x3(read)?,
            spring_const_rotation: read_f32x3(read)?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write.write_u8(self.joint_type as u8)?;
        header.rigid_body_index.write(write, self.a_rigid_index)?;
        header.rigid_body_index.write(write, self.b_rigid_index)?;
        write_f32x3(write, self.position)?;
        write_f32x3(write, self.rotation)?;
        write_f32x3(write, self.move_limit_down)?;
        write_f32x3(write, self.move_limit_up)?;
        write_f32x3(write, self.rotation_limit_down)?;
        write_f32x3(write, self.rotation_limit_up)?;
        write_f32x3(write, self.spring_const_move)?;
        write_f32x3(write, self.spring_const_rotation)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JointType {
    Spring6DOF = 0x00,
    SixDof = 0x01,
    P2P = 0x02,
    ConeTwist = 0x03,
    Slider = 0x04,
    Hinge = 0x05,
}

impl TryFrom<u8> for JointType {
    type Error = PmxError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Spring6DOF),
            0x01 => Ok(Self::SixDof),
            0x02 => Ok(Self::P2P),
            0x03 => Ok(Self::ConeTwist),
            0x04 => Ok(Self::Slider),
            0x05 => Ok(Self::Hinge),
            _ => Err(PmxError::JointTypeError),
        }
    }
}
