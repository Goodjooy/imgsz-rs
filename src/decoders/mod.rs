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
