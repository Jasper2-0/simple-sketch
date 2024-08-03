use std::fmt;

#[derive(Clone, Copy)]
pub struct Color(pub u32);

pub enum ColorFormat {
    Rgba,
    RGBHex,
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

    // Helper function to convert HSV to RGB
    pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
    
        let (r, g, b) = match (h % 360.0) as u32 {
            0..=59 => (c, x, 0.0),
            60..=119 => (x, c, 0.0),
            120..=179 => (0.0, c, x),
            180..=239 => (0.0, x, c),
            240..=299 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };
    
        Color::new(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
            255,
        )
    }

    pub fn fmt_debug(&self, f: &mut fmt::Formatter<'_>, format: ColorFormat) -> fmt::Result {
        match format {
            ColorFormat::Rgba => {
                write!(f, "Color(r: {}, g: {}, b: {}, a: {})", self.r(), self.g(), self.b(), self.a())
            }
            ColorFormat::RGBHex => {
                write!(f, "Color(#{:02X}{:02X}{:02X})", self.r(), self.g(), self.b())
            }
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.fmt_debug(f, ColorFormat::RGBHex)
        } else {
            self.fmt_debug(f, ColorFormat::Rgba)
        }
    }
}