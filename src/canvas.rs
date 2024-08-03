use crate::pixelbuffer::PixelBuffer;
use crate::color::Color;
use crate::geom::Point;
use crate::shape::{Shape, Ellipse, Rectangle};

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixel_buffer: PixelBuffer,
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
            pixel_buffer:PixelBuffer::new(width,height),
            fill:true,
            fill_color: None,
            stroke: true,
            stroke_color: None,
            stroke_weight: 1.0,
        }
    }

    pub fn background(&mut self, color: Color) {
        self.pixel_buffer.clear(color);
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
    pub fn line(&mut self, start: Point, end: Point) {
        if let Some(stroke_color) = &self.stroke_color {
            //self.pixel_buffer.draw_line(start, end, *stroke_color);
            self.pixel_buffer.draw_line_aa(start, end, *stroke_color);
        }
    }





    pub fn ellipse(&mut self, center: Point, width: f32, height: f32) {
        let shape = Ellipse {
            center,
            width,
            height,
        };
        self.draw_shape_aa(&shape);
    }

    pub fn rectangle(&mut self, top_left: Point, width: f32, height: f32) {
        let shape = Rectangle {
            top_left,
            width,
            height,
        };
        self.draw_shape_aa(&shape);
    }

    fn draw_shape_aa(&mut self, shape: &impl Shape) {
        if let Some(fill_color) = &self.fill_color {
            self.fill_shape_aa(shape, *fill_color);

        }
        if let Some(stroke_color) = &self.stroke_color {
            self.stroke_shape(shape, *stroke_color);
//            self.stroke_shape_aa(shape, stroke_color.clone());
        }
    }

    fn fill_shape_aa(&mut self, shape: &impl Shape, color: Color) {
        let (top_left, bottom_right) = shape.bounding_box();
        let (x1, y1) = (top_left.x.floor() as i32, top_left.y.floor() as i32);
        let (x2, y2) = (bottom_right.x.ceil() as i32, bottom_right.y.ceil() as i32);

        for px in x1..=x2 {
            for py in y1..=y2 {
                let point = Point::new(px as f32, py as f32);
                let coverage = self.calculate_coverage(shape, point);
                if coverage > 0.0 {
                    let aa_color = color.with_alpha((color.a() as f32 * coverage) as u8);
                    self.pixel_buffer.blend_pixel(px, py, &aa_color);
                }
            }
        }
    }

    fn stroke_shape(&mut self, shape: &impl Shape, color: Color) {
        let (top_left, bottom_right) = shape.bounding_box();
        let stroke_offset = self.stroke_weight / 2.0;
        let x1 = (top_left.x - stroke_offset).floor() as i32;
        let y1 = (top_left.y - stroke_offset).floor() as i32;
        let x2 = (bottom_right.x + stroke_offset).ceil() as i32;
        let y2 = (bottom_right.y + stroke_offset).ceil() as i32;
        
        for px in x1..=x2 {
            for py in y1..=y2 {
                let point = Point::new(px as f32, py as f32);
                let distance = shape.distance(point);
                
                // Check if the pixel is within the stroke width
                if distance.abs() <= self.stroke_weight / 2.0 {
                    // For sharper lines, don't use anti-aliasing
                    self.pixel_buffer.set_pixel(px, py, color);
                }
                // Optional: Add minimal anti-aliasing at the edges
                else if distance.abs() <= (self.stroke_weight / 2.0) + 1.0 {
                    let alpha = ((self.stroke_weight / 2.0) + 1.0 - distance.abs()) * 255.0;
                    let aa_color = color.with_alpha(alpha as u8);
                    self.pixel_buffer.blend_pixel(px, py, &aa_color);
                }
            }
        }
    }


    fn calculate_coverage(&self, shape: &impl Shape, point: Point) -> f32 {
        let samples = [
            Point::new(0.25, 0.25),
            Point::new(0.75, 0.25),
            Point::new(0.25, 0.75),
            Point::new(0.75, 0.75)
        ];
        
        let count = samples.iter()
            .filter(|&&sample| shape.contains(point + sample))
            .count();
        // Calculate the coverage as the ratio of points inside the shape to total sample points    
        count as f32 / samples.len() as f32
    }
    #[allow(dead_code)]
    fn calculate_stroke_coverage(&self, distance: f32) -> f32 {
        let half_stroke = self.stroke_weight / 2.0;
        if distance.abs() > half_stroke {
            0.0
        } else {
            (half_stroke - distance.abs()) / self.stroke_weight
        }
    }



}