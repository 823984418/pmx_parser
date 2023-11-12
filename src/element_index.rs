use std::fmt::{Debug, Formatter};
use std::io::{Read, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::error::PmxError;
use crate::header::Header;
use crate::kits::read_vec;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct ElementIndices {
    pub element_indices: Vec<u32>,
}

impl Debug for ElementIndices {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("ElementIndices");
        s.field("count", &self.count());
        s.finish()
    }
}

impl ElementIndices {
    pub fn count(&self) -> u32 {
        self.element_indices.len() as u32
    }

    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            element_indices: read_vec(read, |read| header.vertex_index.read_u(read))?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        write.write_u32::<LittleEndian>(self.count())?;
        for i in &self.element_indices {
            header.vertex_index.write(write, *i)?;
        }
        Ok(())
    }
}
