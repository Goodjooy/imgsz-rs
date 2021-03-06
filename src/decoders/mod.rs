mod gif;
mod jpeg;
mod png;
mod webp;

use std::io::Read;
use std::io::SeekFrom;
use std::io::{self, Seek};

use crate::{error, ImageInfo};

#[derive(Debug, Clone)]
pub enum Format {
    Gif,
    Jpeg,
    Png,
    Webp,
    /// 暂时不支持的格式
    Unsupported,
}


pub(crate) trait LoadImgInfo {
    type Result: ImageSize;
    /// 尝试加载图片信息
    fn load<R: Read + Seek>(reader: &mut R) -> io::Result<LoadStatus<Self::Result>>;
    /// 加载成功后，使用这个的format
    fn format() -> Format {
        Format::Unsupported
    }
}

pub(crate) trait ImageSize {
    fn size(&self) -> (usize, usize);
    fn width(&self) -> usize {
        self.size().0
    }
    fn height(&self) -> usize {
        self.size().1
    }
}

pub(crate) enum LoadStatus<T> {
    Ok(T),
    Not,
    LoadErr(String),
}

macro_rules! format_load_fn {
    [$($it:ty),*] => {
        pub(crate) fn imgsz<R: Read + Seek>(reader: &mut R) -> Result<ImageInfo, error::Error> {
            $(
                let loaded = <$it as LoadImgInfo>::load(reader)?;
                match loaded {
                    LoadStatus::Ok(img) => {
                        return Ok(ImageInfo::new(<$it as LoadImgInfo>::format(), img));
                    }
                    LoadStatus::Not => (),
                    LoadStatus::LoadErr(err) => {
                        return Err(error::Error::Other(format!("Decoder Error[{}] : {}",stringify!($it), err)))
                    }
                }

                reader.seek(SeekFrom::Start(0))?;
            )*

            Err(error::Error::Format)
        }
    };
}

format_load_fn![png::Png, gif::Gif, jpeg::Jpeg, webp::WebP];
