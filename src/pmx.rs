use std::io::{Read, Write};

use crate::bone::Bones;
use crate::display_frame::DisplayFrames;
use crate::element_index::ElementIndices;
use crate::error::PmxError;
use crate::header::Header;
use crate::joint::Joints;
use crate::material::Materials;
use crate::model_info::ModelInfo;
use crate::morph::Morphs;
use crate::rigid_body::RigidBodies;
use crate::soft_body::SoftBodies;
use crate::texture::Textures;
use crate::vertex::Vertices;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Pmx {
    pub info: ModelInfo,
    pub vertices: Vertices,
    pub elements: ElementIndices,
    pub textures: Textures,
    pub materials: Materials,
    pub bones: Bones,
    pub morphs: Morphs,
    pub display_frames: DisplayFrames,
    pub rigid_bodies: RigidBodies,
    pub joints: Joints,
    pub soft_bodies: SoftBodies,
}

impl Pmx {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            info: ModelInfo::read(header, read)?,
            vertices: Vertices::read(header, read)?,
            elements: ElementIndices::read(header, read)?,
            textures: Textures::read(header, read)?,
            materials: Materials::read(header, read)?,
            bones: Bones::read(header, read)?,
            morphs: Morphs::read(header, read)?,
            display_frames: DisplayFrames::read(header, read)?,
            rigid_bodies: RigidBodies::read(header, read)?,
            joints: Joints::read(header, read)?,
            soft_bodies: SoftBodies::read(header, read)?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        self.info.write(header, write)?;
        self.vertices.write(header, write)?;
        self.elements.write(header, write)?;
        self.textures.write(header, write)?;
        self.materials.write(header, write)?;
        self.bones.write(header, write)?;
        self.morphs.write(header, write)?;
        self.display_frames.write(header, write)?;
        self.rigid_bodies.write(header, write)?;
        self.joints.write(header, write)?;
        self.soft_bodies.write(header, write)?;
        Ok(())
    }
}
