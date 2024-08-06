use minifb::{Window, WindowOptions};
use crate::canvas::Canvas;
use crate::color::Color;
use crate::geom::Point;
use crate::shape::PolygonBuilder;
use std::f32::consts::PI;

//use std::time::Instant;

pub struct Sketch {
    window: Window,
    canvas: Canvas,
    angle: f32,
}

impl Sketch {
    pub fn new() -> Result<Self, String> {
        let width = 640;
        let height = 360;

        let mut window = Window::new(
            "Simple Sketch",
            width,
            height,
            WindowOptions::default(),
        ).map_err(|e| e.to_string())?;
        
        window.set_target_fps(60);

        Ok(Self {
            window,
            canvas: Canvas::new(width, height),
            angle: 0.0,
        })



    }

    pub fn run(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.update();
            //let start = Instant::now();
            self.draw();
            //let duration = start.elapsed();
            //println!("Time elapsed in draw() is: {:?}", duration);

            self.window
                .update_with_buffer(self.canvas.pixel_buffer.get_buffer(), self.canvas.width, self.canvas.height)
                .unwrap();
        }
    }

    fn update(&mut self) {
        self.angle += 0.0025;
        if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    fn draw(&mut self) {
        self.canvas.background(Color::new(0, 0, 0, 255));

        // Draw grid
        self.canvas.set_stroke(Some(Color::new(32, 32, 32, 255))); // Light gray color for grid
        self.canvas.set_stroke_weight(1.0); // Thin lines for grid

        // Vertical lines
        for x in (0..self.canvas.width).step_by(20) {
            let start = Point::new(x as f32, 0.0);
            let end = Point::new(x as f32, self.canvas.height as f32);
            self.canvas.line(start, end);
        }

        // Horizontal lines
        for y in (0..self.canvas.height).step_by(20) {
            let start = Point::new(0.0, y as f32);
            let end = Point::new(self.canvas.width as f32, y as f32);
            self.canvas.line(start, end);
        }

        // Drawing 10 rotating and waving circles
        let center = Point::new(self.canvas.width as f32 / 2.0, self.canvas.height as f32 / 2.0);
        let base_radius = 100.0; // Base radius for the circular motion
        let wave_amplitude = 10.0; // Amplitude of the wave motion
        let wave_frequency = 6.0; // Frequency of the wave motion

        let num_ellipses = 100;

        for i in 0..num_ellipses {
            let angle = self.angle + (i as f32 * PI * 2.0 / num_ellipses as f32);

            // Add a sine wave to the radius, with a phase offset for each circle
            let wave_offset = self.angle * wave_frequency + (i as f32 * PI / 5.0);
            let radius = base_radius + wave_amplitude * wave_offset.sin();

            let circle_center = center + Point::new(radius * angle.cos(), radius * angle.sin());

            // Set different colors for each circle
            let hue = (i as f32 / num_ellipses as f32) * 360.0;
            let color = Color::hsv_to_rgb(hue, 1.0, 1.0);
            self.canvas.set_stroke(Some(color));
            self.canvas.set_fill(Some(color));

            // Draw the circle
            self.canvas.set_stroke_weight(1.0);
            self.canvas.ellipse(circle_center, 5.0, 5.0);

            // Draw line from center to circle
            self.canvas.line(center, circle_center);
        }

        self.canvas.rectangle(Point::new(20.0,20.0), 40.0, 30.0);

        // Draw a polygon
        let mut polygon_builder = PolygonBuilder::new();
        polygon_builder.begin_shape();
        polygon_builder.vertex(100.0, 100.0);
        polygon_builder.vertex(200.0, 50.0);
        polygon_builder.vertex(300.0, 100.0);
        polygon_builder.vertex(250.0, 200.0);
        polygon_builder.vertex(150.0, 200.0);
        let polygon = polygon_builder.end_shape().unwrap();

        self.canvas.set_fill(Some(Color::new(255, 0, 0, 128))); // Semi-transparent red
        self.canvas.draw_polygon(&polygon);


    }

}