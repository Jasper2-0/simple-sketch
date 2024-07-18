use std::fmt;

#[derive(Clone, Copy)]
pub struct Color(pub u32);

pub enum ColorFormat {
    Rgba,
    RGBHex,
    Raw,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32))
    }

    pub fn r(&self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }
    pub fn g(&self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }
    pub fn b(&self) -> u8 { (self.0 & 0xFF) as u8 }
    pub fn a(&self) -> u8 { ((self.0 >> 24) & 0xFF) as u8 }

    pub fn with_alpha(&self, alpha: u8) -> Color {
        Color::new(self.r(), self.g(), self.b(), alpha)
    }

    pub fn fmt_debug(&self, f: &mut fmt::Formatter<'_>, format: ColorFormat) -> fmt::Result {
        match format {
            ColorFormat::Rgba => {
                write!(f, "Color(r: {}, g: {}, b: {}, a: {})", self.r(), self.g(), self.b(), self.a())
            }
            ColorFormat::RGBHex => {
                write!(f, "Color(#{:02X}{:02X}{:02X})", self.r(), self.g(), self.b())
            }
            ColorFormat::Raw => {
                write!(f, "Color({})", self.0)
            }
        }
    }

}



impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Default to RGBA format if no special formatting is specified
        if f.alternate() {
            // Use '#' flag for RGBHex format
            self.fmt_debug(f, ColorFormat::RGBHex)
        } else {
            self.fmt_debug(f, ColorFormat::Rgba)
        }
    }
}