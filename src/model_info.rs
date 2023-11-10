use std::io::{Read, Write};

use crate::error::PmxError;
use crate::header::Header;

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ModelInfo {
    pub name: String,
    pub name_en: String,
    pub comment: String,
    pub comment_en: String,
}

impl ModelInfo {
    pub fn read<R: Read>(header: &Header, read: &mut R) -> Result<Self, PmxError> {
        Ok(Self {
            name: header.encoding.read(read)?,
            name_en: header.encoding.read(read)?,
            comment: header.encoding.read(read)?,
            comment_en: header.encoding.read(read)?,
        })
    }

    pub fn write<W: Write>(&self, header: &Header, write: &mut W) -> Result<(), PmxError> {
        header.encoding.write(write, self.name.as_str())?;
        header.encoding.write(write, self.name_en.as_str())?;
        header.encoding.write(write, self.comment.as_str())?;
        header.encoding.write(write, self.comment_en.as_str())?;
        Ok(())
    }
}
