use std::io::{Read, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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

#[inline(always)]
pub(crate) fn read_f32x3<R: Read>(read: &mut R) -> Result<[f32; 3], std::io::Error> {
    Ok([
        read.read_f32::<LittleEndian>()?,
        read.read_f32::<LittleEndian>()?,
        read.read_f32::<LittleEndian>()?,
    ])
}

#[inline(always)]
pub(crate) fn read_f32x4<R: Read>(read: &mut R) -> Result<[f32; 4], std::io::Error> {
    Ok([
        read.read_f32::<LittleEndian>()?,
        read.read_f32::<LittleEndian>()?,
        read.read_f32::<LittleEndian>()?,
        read.read_f32::<LittleEndian>()?,
    ])
}

#[inline(always)]
pub(crate) fn write_f32x3<W: Write>(write: &mut W, value: [f32; 3]) -> Result<(), std::io::Error> {
    write.write_f32::<LittleEndian>(value[0])?;
    write.write_f32::<LittleEndian>(value[1])?;
    write.write_f32::<LittleEndian>(value[2])?;
    Ok(())
}

#[inline(always)]
pub(crate) fn write_f32x4<W: Write>(write: &mut W, value: [f32; 4]) -> Result<(), std::io::Error> {
    write.write_f32::<LittleEndian>(value[0])?;
    write.write_f32::<LittleEndian>(value[1])?;
    write.write_f32::<LittleEndian>(value[2])?;
    write.write_f32::<LittleEndian>(value[3])?;
    Ok(())
}

#[inline(always)]
pub(crate) fn read_bool<R: Read>(read: &mut R) -> Result<bool, PmxError> {
    match read.read_u8()? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(PmxError::BoolError),
    }
}

#[inline(always)]
pub(crate) fn read_vec<R: Read, F: FnMut(&mut R) -> Result<T, PmxError>, T>(
    read: &mut R,
    mut f: F,
) -> Result<Vec<T>, PmxError> {
    let count = read.read_u32::<LittleEndian>()? as usize;
    let mut r = Vec::with_capacity(count);
    for _ in 0..count {
        r.push(f(read.by_ref())?);
    }
    Ok(r)
}
