use std::io::{Read, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::read_vec;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Textures {
    pub textures: Vec<String>,
}

impl Textures {
    pub fn count(&self) -> u32 {
        self.textures.len() as u32
    }
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            textures: read_vec(read, |read| header.encoding.read(read))?,
        })
    }
    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.textures {
            header.encoding.write(write, i.as_str())?;
        }
        Ok(())
    }
}
