//! load and save '.pmx' file

use std::io::{Read, Write};

use crate::error::PmxError;
use crate::header::Header;
use crate::pmx::Pmx;

pub mod bone;
pub mod display_frame;
pub mod element_index;
pub mod error;
pub mod header;
pub mod joint;
pub mod material;
pub mod model_info;
pub mod morph;
pub mod pmx;
pub mod rigid_body;
pub mod soft_body;
pub mod texture;
pub mod vertex;

pub(crate) mod kits;

pub fn pmx_read<R: Read>(read: &mut R) -> Result<(Header, Pmx), PmxError> {
    let header = Header::read(read)?;
    let pmx = Pmx::read(&header, read)?;
    Ok((header, pmx))
}

pub fn pmx_write<W: Write>(write: &mut W, pmx: &Pmx, version: f32) -> Result<(), PmxError> {
    let header = Header::from_best(version, pmx);
    header.write(write)?;
    pmx.write(&header, write)?;
    Ok(())
}
