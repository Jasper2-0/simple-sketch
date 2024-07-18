use crate::color::Color;
use crate::canvas::Canvas;
use crate::application::{Application, LoopMode};
use std::f32::consts::PI;

pub struct Sketch {
    angle: f32,
}

impl Application for Sketch {
    fn new() -> Result<Self, String> {
        Ok(Self {
            angle: 0.0,
        })
    }

    fn setup(&mut self) {
        self.size(640, 360);
        self.title("Rotating Ellipse");
        self.frame_rate(60);
        self.set_loop_mode(LoopMode::Continuous);


        println!("Setting up the application...");
    }

    fn update(&mut self) {
        self.angle += 0.0025;
        if self.angle > 2.0 * PI {
            self.angle -= 2.0 * PI;
        }
    }

    fn draw(&mut self, canvas: &mut Canvas) {
        canvas.background(Color::new(0, 0, 0, 255));
        canvas.set_stroke(Some(Color::new(255, 255, 255, 255)));
        canvas.set_stroke_weight(2.0);

        let center_x = 640.0 / 2.0;
        let center_y = 360.0 / 2.0;
        let radius = 90.0;

        let x = center_x + radius * self.angle.cos();
        let y = center_y + radius * self.angle.sin();

        canvas.ellipse(x, y, 50.0, 50.0);
    }

}