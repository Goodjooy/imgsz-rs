

#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub size: (usize, usize),
    pub format: crate::Format,
}

impl ImageInfo {
    #[cfg(feature="use-img")]
    pub(crate) fn from_img_reader<R>(img: image::io::Reader<R>) -> crate::ImgResult<Self>
    where
        R: std::io::BufRead + std::io::Seek,
    {
        let format = img.format().ok_or(crate::error::Error::Format)?.into();
        let size = img.into_dimensions()?;

        Ok(ImageInfo {
            size: (size.0 as usize, size.1 as usize),
            format,
        })
    }

    pub(crate) fn new(format: crate::Format, img: impl crate::decoders::ImageSize) -> Self {
        Self {
            size: img.size(),
            format,
        }
    }
}
