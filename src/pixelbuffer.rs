use crate::color::Color;
use crate::point::Point;

pub struct PixelBuffer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<u32>,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        PixelBuffer {
            width,
            height,
            buffer: vec![0;width* height],
        }
    }

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

    pub fn clear(&mut self, color: Color) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color.0;
        }
    }


    /**
    * Pixel setting methods
    */
    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        // Check if the pixel is within the canvas bounds
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            
            // Directly set the 32-bit color value in the buffer
            self.buffer[index] = color.0;
        }
    }
    pub fn blend_pixel(&mut self, x: i32, y: i32, color: &Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            let background = self.buffer[index];
    
            let bg_color = Color(background);
            let alpha = color.a() as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;
    
            let new_r = (inv_alpha * bg_color.r() as f32 + alpha * color.r() as f32) as u8;
            let new_g = (inv_alpha * bg_color.g() as f32 + alpha * color.g() as f32) as u8;
            let new_b = (inv_alpha * bg_color.b() as f32 + alpha * color.b() as f32) as u8;
            let new_a = 255; // Assuming full opacity for the final color
    
            self.buffer[index] = Color::new(new_r, new_g, new_b, new_a).0;
        }
    }

    pub fn plot(&mut self, x: i32, y: i32, color: Color, alpha: f32) {
        let aa_color = color.with_alpha((color.a() as f32 * alpha) as u8);
        self.blend_pixel(x, y, &aa_color);
    }

    #[allow(dead_code)]
    // bresenham's line drawing algorithm
    pub fn draw_line(&mut self, start: Point, end: Point, color: Color) {
        
        let start_x = start.x.floor() as i32;
        let start_y = start.y.floor() as i32;

        let end_x = end.x.floor() as i32;
        let end_y = end.y.floor() as i32;
        
        let dx = (end_x - start_x).abs();
        let dy = (end_y - start_y).abs();
        let sx = if start_x < end_x { 1 } else { -1 };
        let sy = if start_y < end_y { 1 } else { -1 };
        let mut err = dx - dy;
    
        let mut x = start_x;
        let mut y = start_y;
    
        loop {
            self.set_pixel(x, y, color);
            if x == end_x && y == end_y { break; }
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }
    

    // Wu's anti-aliased line drawing algorithm
    pub fn draw_line_aa(&mut self, mut start: Point, mut end: Point, color: Color) {
        let steep = (end.y - start.y).abs() > (end.x - start.x).abs();
        
        if steep {
            std::mem::swap(&mut start.x, &mut start.y);
            std::mem::swap(&mut end.x, &mut end.y);
        }
        
        if start.x > end.x {
            std::mem::swap(&mut start, &mut end);
        }

        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let gradient = if dx == 0.0 { 1.0 } else { dy / dx };

        // Handle first endpoint
        let mut xend = start.x.round();
        let mut yend = start.y + gradient * (xend - start.x);
        let mut xgap = 1.0 - (start.x + 0.5).fract();
        let xpxl1 = xend as i32;
        let ypxl1 = yend.floor() as i32;
        
        if steep {
            self.plot(ypxl1, xpxl1, color, (1.0 - yend.fract()) * xgap);
            self.plot(ypxl1 + 1, xpxl1, color, yend.fract() * xgap);
        } else {
            self.plot(xpxl1, ypxl1, color, (1.0 - yend.fract()) * xgap);
            self.plot(xpxl1, ypxl1 + 1, color, yend.fract() * xgap);
        }
        
        let mut intery = yend + gradient;

        // Handle second endpoint
        xend = end.x.round();
        yend = end.y + gradient * (xend - end.x);
        xgap = (end.x + 0.5).fract();
        let xpxl2 = xend as i32;
        let ypxl2 = yend.floor() as i32;
        
        if steep {
            self.plot(ypxl2, xpxl2, color, (1.0 - yend.fract()) * xgap);
            self.plot(ypxl2 + 1, xpxl2, color, yend.fract() * xgap);
        } else {
            self.plot(xpxl2, ypxl2, color, (1.0 - yend.fract()) * xgap);
            self.plot(xpxl2, ypxl2 + 1, color, yend.fract() * xgap);
        }

        // Main loop
        if steep {
            for x in (xpxl1 + 1)..xpxl2 {
                self.plot(intery.floor() as i32, x, color, 1.0 - intery.fract());
                self.plot(intery.floor() as i32 + 1, x, color, intery.fract());
                intery += gradient;
            }
        } else {
            for x in (xpxl1 + 1)..xpxl2 {
                self.plot(x, intery.floor() as i32, color, 1.0 - intery.fract());
                self.plot(x, intery.floor() as i32 + 1, color, intery.fract());
                intery += gradient;
            }
        }
    }
/* 
        // Fill a rectangular area with a specific color
        pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
            let x_end = (x + width).min(self.width);
            let y_end = (y + height).min(self.height);
    
            for cy in y..y_end {
                for cx in x..x_end {
                    self.set_pixel(cx, cy, color);
                }
            }
        }*/
        /* 
        // Fill an arbitrary shape using flood fill algorithm
        pub fn flood_fill(&mut self, x: u32, y: u32, fill_color: u32) {
            let target_color = self.get_pixel(x, y).unwrap_or(0);
            if target_color == fill_color {
                return;
            }
    
            let mut stack = vec![(x, y)];
    
            while let Some((cx, cy)) = stack.pop() {
                if self.get_pixel(cx, cy) == Some(target_color) {
                    self.set_pixel(cx, cy, fill_color);
    
                    if cx > 0 { stack.push((cx - 1, cy)); }
                    if cy > 0 { stack.push((cx, cy - 1)); }
                    if cx < self.width - 1 { stack.push((cx + 1, cy)); }
                    if cy < self.height - 1 { stack.push((cx, cy + 1)); }
                }
            }
        }*/




}