use std::io::{self, Read};
use std::io::{Seek, SeekFrom};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::decoders::LoadStatus;

use super::{ImageSize, LoadImgInfo};

pub struct WebP {
    width: usize,
    height: usize,
}

impl ImageSize for WebP {
    fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl LoadImgInfo for WebP {
    type Result = Self;

    fn load<R: std::io::Read + std::io::Seek>(
        reader: &mut R,
    ) -> std::io::Result<super::LoadStatus<Self::Result>> {
        let mut buf = [0u8; 4];

        reader.read_exact(&mut buf)?;

        if &buf != b"RIFF" {
            return Ok(LoadStatus::Not);
        }

        let _size = reader.read_u32::<BigEndian>()?;

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;

        if &buf != b"WEBP" {
            return Ok(LoadStatus::Not);
        }

        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;

        let mut width = 100usize;
        let mut height = 100usize;

        match &buf {
            b"VP8 " => {
                let _full_size = reader.read_u32::<LittleEndian>()?;
                let tag = reader.read_u24::<LittleEndian>()?;

                let keyframe = tag & 1 == 0;
                print!("VV{}",keyframe);
                if keyframe {
                    let mut buf = [0u8; 3];
                    reader.read_exact(&mut buf)?;
                }
                let w = reader.read_u16::<LittleEndian>()?;
                let h = reader.read_u16::<LittleEndian>()?;

                width = (w as usize) & 0x3fff;
                height = (h as usize) & 0x3fff;
            }
            b"VP8L" => {
                let _full_size = reader.read_u32::<LittleEndian>()?;
                let _signature = reader.read_u8()?;
                let mut bit_count = 0;

                let w = read_bits(reader, 14, &mut bit_count)? + 1;
                let h = read_bits(reader, 14, &mut bit_count)? + 1;

                width = w as usize;
                height = h as usize;
            }
            _ => {}
        }
        Ok(LoadStatus::Ok(Self { width, height }))
    }

    fn format() -> crate::Format {
        crate::Format::Webp
    }
}
fn read_bits<R: Read + Seek>(reader: &mut R, num: u8, bit_count: &mut u8) -> io::Result<u16> {
    let mut value = 0;

    let mut buf = [0u8; 1];
    for i in 0..num {
        reader.read_exact(&mut buf)?;
        let bit_true = buf[0] & (1 << *bit_count) != 0;
        value += u16::from(bit_true) << i;
        *bit_count = if *bit_count == 7 {
            0
        } else {
            reader.seek(SeekFrom::Current(-1))?;
            *bit_count + 1
        };
    }

    Ok(value)
}
