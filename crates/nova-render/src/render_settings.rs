use crate::color::Color;

#[derive(Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
pub struct RenderSettings {
    pub msaa: u32,
    pub clear: Color,
}

impl Default for RenderSettings {
    #[inline]
    fn default() -> Self {
        Self {
            msaa: 1,
            clear: Color::TRANSPARENT,
        }
    }
}
