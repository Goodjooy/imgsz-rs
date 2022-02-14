#[derive(Debug, Clone)]
pub struct ImageInfo {
    pub size: (usize, usize),
    pub format: crate::Format,
}

impl ImageInfo {
    pub(crate) fn new(format: crate::Format, img: impl crate::decoders::ImageSize) -> Self {
        Self {
            size: img.size(),
            format,
        }
    }

    pub fn width(&self) -> usize {
        self.size.0
    }

    pub fn height(&self) -> usize {
        self.size.1
    }
}
