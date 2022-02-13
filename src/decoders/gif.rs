use std::io;

use byteorder::ReadBytesExt;

use super::{ImageSize, LoadImgInfo, LoadStatus};

const GIF_HEAD: &[u8; 3] = b"GIF";

pub struct Gif {
    width: usize,
    height: usize,
}

impl ImageSize for Gif {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl LoadImgInfo for Gif {
    type Result = Self;

    fn load<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
    ) -> std::io::Result<super::LoadStatus<Self::Result>> {
        if !Self::check_head(reader)? {
            return Ok(LoadStatus::Not);
        }
        let (width, height) = Self::read_size(reader)?;

        Ok(LoadStatus::Ok(Gif {
            width: width as usize,
            height: height as usize,
        }))
    }

    fn format() -> crate::Format {
        crate::Format::Gif
    }
}

impl Gif {
    fn check_head<R: std::io::Read + std::io::Seek>(reader: &mut R) -> io::Result<bool> {
        let mut buf = [0u8; 6];
        reader.read_exact(&mut buf)?;

        Ok(buf.starts_with(GIF_HEAD))
    }

    fn read_size<R: std::io::Read + std::io::Seek>(reader: &mut R) -> io::Result<(u16, u16)> {
        let w = reader.read_u16::<byteorder::LittleEndian>()?;
        let h = reader.read_u16::<byteorder::LittleEndian>()?;
        Ok((w, h))
    }
}
