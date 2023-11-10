use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::{read_bool, read_f32x3, read_vec, write_f32x3};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Bones {
    pub bones: Vec<Bone>,
}

impl Bones {
    pub fn count(&self) -> u32 {
        self.bones.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            bones: read_vec(read, |read| Bone::read(header, read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.bones {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq)]
pub struct Bone {
    pub name: String,
    pub name_en: String,
    pub position: [f32; 3],
    pub parent_bone_index: u32,
    pub priority: u32,
    pub connect: BoneConnection,
    pub rotatable: bool,
    pub translatable: bool,
    pub is_visible: bool,
    pub enable: bool,
    pub inherit_local: bool,
    pub inherit_rotate_or_translation: Option<InheritRotateOrTranslation>,
    pub fixed_axis: Option<[f32; 3]>,
    pub local_axis: Option<([f32; 3], [f32; 3])>,
    pub physics_after_deform: bool,
    pub external_parent_bone_index: Option<u32>,
    pub ik: Option<Ik>,
    pub unknown_0040: bool,
    pub unknown_2000: bool,
    pub unknown_4000: bool,
    pub unknown_8000: bool,
}

impl Debug for Bone {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("Bone");
        s.field("name", &self.name);
        s.field("name_en", &self.name_en);
        s.field("position", &self.position);
        s.field("parent_bone_index", &self.parent_bone_index);
        s.field("priority", &self.priority);
        s.finish()
    }
}

impl Bone {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        let name = header.encoding.read(read)?;
        let name_en = header.encoding.read(read)?;
        let position = read_f32x3(read)?;
        let parent_bone_index = header.bone_index.read(read)?;
        let priority = read.read_u32::<LittleEndian>()?;

        let flags = BoneFlags::from_bits_retain(read.read_u16::<LittleEndian>()?);
        let rotate = flags.contains(BoneFlags::INHERIT_ROTATION);
        let translation = flags.contains(BoneFlags::INHERIT_TRANSLATION);
        let rotate_or_translation = match (rotate, translation) {
            (false, false) => None,
            (true, false) => Some(RotateOrTranslation::Rotate),
            (false, true) => Some(RotateOrTranslation::Translation),
            (true, true) => Some(RotateOrTranslation::RotateTranslation),
        };

        Ok(Self {
            name,
            name_en,
            position,
            parent_bone_index,
            priority,
            rotatable: flags.contains(BoneFlags::ROTATABLE),
            translatable: flags.contains(BoneFlags::TRANSLATABLE),
            is_visible: flags.contains(BoneFlags::IS_VISIBLE),
            enable: flags.contains(BoneFlags::ENABLED),
            inherit_local: flags.contains(BoneFlags::INHERIT_LOCAL),
            physics_after_deform: flags.contains(BoneFlags::PHYSICS_AFTER_DEFORM),
            connect: if flags.contains(BoneFlags::CONNECT_TO_OTHER_BONE) {
                BoneConnection::BoneIndex(header.bone_index.read(read)?)
            } else {
                BoneConnection::Position(read_f32x3(read)?)
            },
            inherit_rotate_or_translation: match rotate_or_translation {
                Some(rotate_or_translation) => Some(InheritRotateOrTranslation {
                    rotate_or_translation,
                    bone_index: header.bone_index.read(read)?,
                    weight: read.read_f32::<LittleEndian>()?,
                }),
                None => None,
            },
            fixed_axis: if flags.contains(BoneFlags::FIXED_AXIS) {
                Some(read_f32x3(read)?)
            } else {
                None
            },
            local_axis: if flags.contains(BoneFlags::LOCAL_COORDINATE) {
                Some((read_f32x3(read)?, read_f32x3(read)?))
            } else {
                None
            },
            external_parent_bone_index: if flags.contains(BoneFlags::EXTERNAL_PARENT_DEFORM) {
                Some(header.bone_index.read(read)?)
            } else {
                None
            },
            ik: if flags.contains(BoneFlags::IK) {
                Some(Ik::read(header, read)?)
            } else {
                None
            },
            unknown_0040: flags.contains(BoneFlags::UNKNOWN_0040),
            unknown_2000: flags.contains(BoneFlags::UNKNOWN_2000),
            unknown_4000: flags.contains(BoneFlags::UNKNOWN_4000),
            unknown_8000: flags.contains(BoneFlags::UNKNOWN_8000),
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        write_f32x3(write, self.position)?;
        header.bone_index.write(write, self.parent_bone_index)?;
        write.write_u32::<LittleEndian>(self.priority)?;
        write.write_u16::<LittleEndian>(self.flags().bits())?;
        match self.connect {
            BoneConnection::BoneIndex(index) => {
                header.bone_index.write(write, index)?;
            }
            BoneConnection::Position(pos) => {
                write_f32x3(write, pos)?;
            }
        }
        if let Some(i) = self.inherit_rotate_or_translation {
            header.bone_index.write(write, i.bone_index)?;
            write.write_f32::<LittleEndian>(i.weight)?;
        }
        if let Some(i) = self.fixed_axis {
            write_f32x3(write, i)?;
        }
        if let Some((min_angle, max_angle)) = self.local_axis {
            write_f32x3(write, min_angle)?;
            write_f32x3(write, max_angle)?;
        }
        if let Some(i) = self.external_parent_bone_index {
            header.bone_index.write(write, i)?;
        }
        if let Some(i) = &self.ik {
            i.write(header, write)?;
        }
        Ok(())
    }

    pub fn flags(&self) -> BoneFlags {
        let mut flags = BoneFlags::empty();
        if matches!(self.connect, BoneConnection::BoneIndex(_)) {
            flags |= BoneFlags::CONNECT_TO_OTHER_BONE;
        }
        if self.rotatable {
            flags |= BoneFlags::ROTATABLE;
        }
        if self.translatable {
            flags |= BoneFlags::TRANSLATABLE;
        }
        if self.is_visible {
            flags |= BoneFlags::IS_VISIBLE;
        }
        if self.enable {
            flags |= BoneFlags::ENABLED;
        }
        if self.ik.is_some() {
            flags |= BoneFlags::IK;
        }
        if self.unknown_0040 {
            flags |= BoneFlags::UNKNOWN_0040;
        }
        if self.inherit_local {
            flags |= BoneFlags::INHERIT_LOCAL;
        }
        if let Some(x) = self.inherit_rotate_or_translation {
            match x.rotate_or_translation {
                RotateOrTranslation::Rotate => {
                    flags |= BoneFlags::INHERIT_ROTATION;
                }
                RotateOrTranslation::Translation => {
                    flags |= BoneFlags::INHERIT_TRANSLATION;
                }
                RotateOrTranslation::RotateTranslation => {
                    flags |= BoneFlags::INHERIT_ROTATION | BoneFlags::INHERIT_TRANSLATION;
                }
            }
        }
        if self.fixed_axis.is_some() {
            flags |= BoneFlags::FIXED_AXIS;
        }
        if self.local_axis.is_some() {
            flags |= BoneFlags::LOCAL_COORDINATE;
        }
        if self.physics_after_deform {
            flags |= BoneFlags::PHYSICS_AFTER_DEFORM;
        }
        if self.external_parent_bone_index.is_some() {
            flags |= BoneFlags::EXTERNAL_PARENT_DEFORM;
        }
        if self.unknown_2000 {
            flags |= BoneFlags::UNKNOWN_2000;
        }
        if self.unknown_4000 {
            flags |= BoneFlags::UNKNOWN_4000;
        }
        if self.unknown_8000 {
            flags |= BoneFlags::UNKNOWN_8000;
        }
        flags
    }
}

bitflags::bitflags! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct BoneFlags: u16 {
        const CONNECT_TO_OTHER_BONE = 0x0001;
        const ROTATABLE = 0x0002;
        const TRANSLATABLE = 0x0004;
        const IS_VISIBLE = 0x0008;
        const ENABLED = 0x0010;
        const IK = 0x0020;
        const UNKNOWN_0040 = 0x0040;
        const INHERIT_LOCAL = 0x0080;
        const INHERIT_ROTATION = 0x0100;
        const INHERIT_TRANSLATION = 0x0200;
        const FIXED_AXIS = 0x0400;
        const LOCAL_COORDINATE = 0x0800;
        const PHYSICS_AFTER_DEFORM = 0x1000;
        const EXTERNAL_PARENT_DEFORM = 0x2000;
        const UNKNOWN_2000 = 0x2000;
        const UNKNOWN_4000 = 0x2000;
        const UNKNOWN_8000 = 0x2000;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoneConnection {
    BoneIndex(u32),
    Position([f32; 3]),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct InheritRotateOrTranslation {
    rotate_or_translation: RotateOrTranslation,
    bone_index: u32,
    weight: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum RotateOrTranslation {
    Rotate,
    Translation,
    RotateTranslation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ik {
    pub target_bone_index: u32,
    pub iter_count: u32,
    pub limit_angle: f32,
    pub links: Vec<IkLink>,
}

impl Ik {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            target_bone_index: header.bone_index.read(read)?,
            iter_count: read.read_u32::<LittleEndian>()?,
            limit_angle: read.read_f32::<LittleEndian>()?,
            links: read_vec(read, |read| IkLink::read(header, read))?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.bone_index.write(write, self.target_bone_index)?;
        write.write_u32::<LittleEndian>(self.iter_count)?;
        write.write_f32::<LittleEndian>(self.limit_angle)?;
        write.write_u32::<LittleEndian>(self.links.len() as u32)?;
        for i in &self.links {
            i.write(header, write)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct IkLink {
    pub bone_index: u32,
    pub angle_limit: Option<([f32; 3], [f32; 3])>,
}

impl IkLink {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            bone_index: header.bone_index.read(read)?,
            angle_limit: match read_bool(read)? {
                true => Some((read_f32x3(read)?, read_f32x3(read)?)),
                false => None,
            },
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.bone_index.write(write, self.bone_index)?;
        if let Some((min_angle, max_angle)) = self.angle_limit {
            write.write_u8(1)?;
            write_f32x3(write, min_angle)?;
            write_f32x3(write, max_angle)?;
        } else {
            write.write_u8(0)?;
        }
        Ok(())
    }
}
