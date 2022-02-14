use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use super::{ImageSize, LoadImgInfo, LoadStatus};

pub struct Jpeg {
    width: usize,
    height: usize,
}

impl ImageSize for Jpeg {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl LoadImgInfo for Jpeg {
    type Result = Self;

    fn load<R>(reader: &mut R) -> io::Result<LoadStatus<Self::Result>>
    where
        R: io::Read + io::Seek,
    {
        let mut buf = [0u8; 2];

        reader.read_exact(&mut buf)?;

        if buf != [0xff, 0xd8] {
            return Ok(LoadStatus::Not);
        }

        while buf != [0xff, 0xc0] {
            reader.read_exact(&mut buf)?;
        }

        let le = reader.read_u16::<BigEndian>()?;

        if le != 17 {
            return Ok(LoadStatus::LoadErr(format!("bad `SOF` size[{}]", le)));
        }

        let _ = reader.read_u8()?;

        let height = reader.read_u16::<BigEndian>()? as usize;
        let width = reader.read_u16::<BigEndian>()? as usize;

        Ok(LoadStatus::Ok(Self { width, height }))
    }

    fn format() -> crate::Format {
        crate::Format::Jpeg
    }
}
