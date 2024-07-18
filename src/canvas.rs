//use std::fs::File;
//use std::io::{Write, Result};
use crate::color::Color;
use crate::shape::{Shape, Ellipse, Rectangle};

pub struct Canvas {
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    stroke: bool,
    fill: bool,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    stroke_weight: f32,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            buffer: vec![0; width * height],
            fill:true,
            fill_color: None,
            stroke: true,
            stroke_color: None,
            stroke_weight: 1.0,
        }
    }

    pub fn background(&mut self, color: Color) {
        for pixel in self.buffer.iter_mut() {
            *pixel = color.0;
        }
    }

    pub fn set_fill(&mut self, color: Option<Color>) {
        self.fill_color = color;
        self.fill = true;
    }

    pub fn set_stroke(&mut self, color: Option<Color>) {
        self.stroke_color = color;
        self.stroke = true;
    }

    pub fn set_stroke_weight(&mut self, weight: f32) {
        self.stroke_weight = weight;
    }

    pub fn ellipse(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let shape = Ellipse {
            center_x: x + w / 2.0,
            center_y: y + h / 2.0,
            width: w,
            height: h,
        };
        self.draw_shape_aa(&shape);
    }

   pub fn rectangle(&mut self, x: f32, y: f32, w: f32, h: f32) {
        let shape = Rectangle {
            x,
            y,
            width: w,
            height: h,
        };
        self.draw_shape_aa(&shape);
    }

    fn draw_shape_aa(&mut self, shape: &impl Shape) {
        if let Some(fill_color) = &self.fill_color {
            self.fill_shape_aa(shape, fill_color.clone());

        }
        if let Some(stroke_color) = &self.stroke_color {
            self.stroke_shape_aa(shape, stroke_color.clone());
        }
    }

    fn fill_shape_aa(&mut self, shape: &impl Shape, color: Color) {
        let (x, y, w, h) = shape.bounding_box();
        let (x1, y1) = (x.floor() as i32, y.floor() as i32);
        let (x2, y2) = ((x + w).ceil() as i32, (y + h).ceil() as i32);

        for px in x1..=x2 {
            for py in y1..=y2 {
                let coverage = self.calculate_coverage(shape, px as f32, py as f32);
                if coverage > 1.0 {
                    let aa_color = color.with_alpha((color.a() as f32 * coverage) as u8);
                    self.blend_pixel(px, py, &aa_color);
                }
            }
        }
    }

    fn stroke_shape_aa(&mut self, shape: &impl Shape, color: Color) {
        let (x, y, w, h) = shape.bounding_box();
        let (x1, y1) = (x.floor() as i32 - self.stroke_weight as i32, y.floor() as i32 - self.stroke_weight as i32);
        let (x2, y2) = ((x + w).ceil() as i32 + self.stroke_weight as i32, (y + h).ceil() as i32 + self.stroke_weight as i32);

        for px in x1..=x2 {
            for py in y1..=y2 {
                let distance = shape.distance(px as f32, py as f32);
                let coverage = self.calculate_stroke_coverage(distance);
                if coverage > 0.0 {
                    let aa_color = color.with_alpha((color.a() as f32 * coverage) as u8);
                    self.blend_pixel(px, py, &aa_color);
                }
            }
        }
    }

    fn calculate_coverage(&self, shape: &impl Shape, x: f32, y: f32) -> f32 {
        let samples = [
            (0.125, 0.125), (0.375, 0.125), (0.625, 0.125), (0.875, 0.125),
            (0.125, 0.375), (0.375, 0.375), (0.625, 0.375), (0.875, 0.375),
            (0.125, 0.625), (0.375, 0.625), (0.625, 0.625), (0.875, 0.625),
            (0.125, 0.875), (0.375, 0.875), (0.625, 0.875), (0.875, 0.875)
        ];
        
        let count = samples.iter()
        .filter(|&&(dx, dy)| shape.contains(x + dx, y + dy))
        .count();
    
        // Calculate the coverage as the ratio of points inside the shape to total sample points
        count as f32 / samples.len() as f32
    }

    fn calculate_stroke_coverage(&self, distance: f32) -> f32 {
        let half_stroke = self.stroke_weight / 2.0;
        if distance.abs() > half_stroke {
            0.0
        } else {
            (half_stroke - distance.abs()) / self.stroke_weight
        }
    }

    fn blend_pixel(&mut self, x: i32, y: i32, color: &Color) {
        if x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32 {
            let index = y as usize * self.width + x as usize;
            let bg = self.buffer[index];
    
            let bg_color = Color(bg);
            let alpha = color.a() as f32 / 255.0;
            let inv_alpha = 1.0 - alpha;
    
            let new_r = (inv_alpha * bg_color.r() as f32 + alpha * color.r() as f32) as u8;
            let new_g = (inv_alpha * bg_color.g() as f32 + alpha * color.g() as f32) as u8;
            let new_b = (inv_alpha * bg_color.b() as f32 + alpha * color.b() as f32) as u8;
            let new_a = 255; // Assuming full opacity for the final color
    
            self.buffer[index] = Color::new(new_r, new_g, new_b, new_a).0;
        }
    }
    

    /*pub fn save_as_ppm(&self, filename: &str) -> Result<()> {
        let mut file = File::create(filename)?;

        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.width, self.height)?;
        writeln!(file, "255")?;

        for pixel in &self.buffer {
            writeln!(file, "{} {} {}", pixel.r(), pixel.g(), pixel.b())?;
        }

        Ok(())
    }*/

    pub fn get_buffer(&self) -> &[u32] {
        &self.buffer
    }

}