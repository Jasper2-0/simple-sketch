use minifb::{Window, WindowOptions};
use crate::canvas::Canvas;

pub enum LoopMode {
    SingleFrame,
    Continuous,
}

pub trait Application: Sized {
    fn new() -> Result<Self, String>;
    fn setup(&mut self);
    fn update(&mut self);
    fn draw(&mut self, canvas: &mut Canvas);

    fn size(&mut self, _width: usize, _height: usize) {}
    fn title(&mut self, _title: &str) {}
    fn frame_rate(&mut self, _rate: usize) {}
    fn set_loop_mode(&mut self, _mode: LoopMode) {}
}

pub struct ApplicationRunner<A: Application> {
    application: A,
    window: Window,
    canvas: Canvas,
    width: usize,
    height: usize,
    title: String,
    frame_rate: usize,
    loop_mode: LoopMode,
}

impl<A: Application> ApplicationRunner<A> {
    pub fn new() -> Result<Self, String> {
        let mut runner = Self {
            application: A::new()?,
            window: Window::new(
                "2D Canvas",
                640,
                360,
                WindowOptions::default(),
            ).map_err(|e| e.to_string())?,
            canvas: Canvas::new(640, 360),
            width: 640,
            height: 360,
            title: "2D Canvas".to_string(),
            frame_rate: 60,
            loop_mode: LoopMode::Continuous,
        };

        runner.application.setup();
        
        Ok(runner)
    }

    pub fn run(&mut self) {
        match self.loop_mode {
            LoopMode::SingleFrame => self.run_single_frame(),
            LoopMode::Continuous => self.run_continuous(),
        }
    }

    fn run_single_frame(&mut self) {
        self.application.update();
        self.application.draw(&mut self.canvas);
        self.window
            .update_with_buffer(self.canvas.get_buffer(), self.width, self.height)
            .unwrap();

        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.window.update();
        }
    }

    fn run_continuous(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(minifb::Key::Escape) {
            self.application.update();
            self.application.draw(&mut self.canvas);
            self.window
                .update_with_buffer(self.canvas.get_buffer(), self.width, self.height)
                .unwrap();
        }
    }
}

impl<A: Application> Application for ApplicationRunner<A> {
    fn new() -> Result<Self, String> {
        Self::new()
    }

    fn setup(&mut self) {}

    fn update(&mut self) {}

    fn draw(&mut self, _canvas: &mut Canvas) {}

    fn size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.window = Window::new(
            &self.title,
            width,
            height,
            WindowOptions::default(),
        ).expect("Failed to resize window");
        self.canvas = Canvas::new(width, height);
    }

    fn title(&mut self, title: &str) {
        self.title = title.to_string();
        self.window.set_title(title);
    }

    fn frame_rate(&mut self, rate: usize) {
        self.frame_rate = rate;
        self.window.set_target_fps(rate);
    }

    fn set_loop_mode(&mut self, mode: LoopMode) {
        self.loop_mode = mode;
    }
}