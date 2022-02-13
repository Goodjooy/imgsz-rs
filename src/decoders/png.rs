use std::io::{self, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use super::{ImageSize, LoadImgInfo, LoadStatus};

const PNG_HEAD: &[u8; 8] = b"\x89PNG\r\n\x1a\n";
const PNG_INFO_TAG: &[u8; 4] = b"IHDR";

pub struct Png {
    width: usize,
    height: usize,
}

impl ImageSize for Png {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl LoadImgInfo for Png {
    type Result = Self;

    fn load<R: Read + Seek>(reader: &mut R) -> io::Result<LoadStatus<Self::Result>> {
        if Png::check_head(reader)? {
            let size = Png::read_ihdr_size(reader)?;
            if size == 13 {
                if Png::match_idhr(reader)? {
                    let (w, h) = Self::read_size(reader)?;
                    let png = Self {
                        width: w as usize,
                        height: h as usize,
                    };
                    Ok(LoadStatus::Ok(png))
                } else {
                    Ok(LoadStatus::LoadErr(format!("`IDHR` not found")))
                }
            } else {
                Ok(LoadStatus::LoadErr(format!("bad `IDHR` size[{}]", size)))
            }
        } else {
            Ok(LoadStatus::Not)
        }
    }

    fn format() -> crate::Format {
        crate::Format::Png
    }
}

impl Png {
    fn check_head<R: Read + Seek>(reader: &mut R) -> Result<bool, io::Error> {
        let mut buf = [0u8; 8];

        let _ = reader.read_exact(&mut buf)?;

        if &buf == PNG_HEAD {
            Ok(true)
        } else {
            reader.seek(SeekFrom::Start(0))?;
            Ok(false)
        }
    }

    fn read_ihdr_size<R: Read + Seek>(reader: &mut R) -> Result<u32, io::Error> {
        reader.read_u32::<BigEndian>()
    }

    fn match_idhr<R: Read + Seek>(reader: &mut R) -> Result<bool, io::Error> {
        let mut buf = [0u8; 4];

        reader.read_exact(&mut buf)?;

        Ok(&buf == PNG_INFO_TAG)
    }

    fn read_size<R: Read + Seek>(reader: &mut R) -> Result<(u32, u32), io::Error> {
        let w = reader.read_u32::<BigEndian>()?;
        let h = reader.read_u32::<BigEndian>()?;
        Ok((w, h))
    }
}
