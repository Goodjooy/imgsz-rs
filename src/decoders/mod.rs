use std::io::{BufRead, Seek};

use image::ImageFormat;

pub enum Format {
    Gif,
    Jpeg,
    Png,
    Webp,
    /// 暂时不支持的格式
    Unsupported,
}

impl From<ImageFormat> for Format {
    fn from(f: ImageFormat) -> Self {
        match f {
            ImageFormat::Png => Self::Png,
            ImageFormat::Jpeg => Self::Jpeg,
            ImageFormat::Gif => Self::Gif,
            ImageFormat::WebP => Self::Webp,
            _ => Self::Unsupported,
        }
    }
}

pub(crate) trait LoadImgInfo {
    type Result: ImgInfo;
    fn load<R: BufRead + Seek>(reader: &mut R) -> Option<Self::Result>;
}

pub(crate) trait ImgInfo {
    fn size(&self) -> (usize, usize);
    fn format(&self) -> Format;
}
