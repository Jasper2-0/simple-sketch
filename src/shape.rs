pub trait Shape {
    fn contains(&self, x: f32, y: f32) -> bool;
    fn bounding_box(&self) -> (f32, f32, f32, f32);
    fn distance(&self, x: f32, y: f32) -> f32;
}

pub struct Ellipse {
    pub center_x: f32,
    pub center_y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Shape for Ellipse {
    fn contains(&self, x: f32, y: f32) -> bool {
        let dx = (x - self.center_x) / (self.width / 2.0);
        let dy = (y - self.center_y) / (self.height / 2.0);
        dx * dx + dy * dy <= 1.0
    }

    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (
            self.center_x - self.width / 2.0,
            self.center_y - self.height / 2.0,
            self.width,
            self.height,
        )
    }

    fn distance(&self, x: f32, y: f32) -> f32 {
        let dx = (x - self.center_x) / (self.width / 2.0);
        let dy = (y - self.center_y) / (self.height / 2.0);
        let distance_squared = dx * dx + dy * dy;
        (distance_squared.sqrt() - 1.0) * (self.width.min(self.height) / 2.0)
    }
}

impl Shape for Rectangle {
    fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }

    fn distance(&self, x: f32, y: f32) -> f32 {
        let dx = (x - self.x).max(0.0).min(self.width);
        let dy = (y - self.y).max(0.0).min(self.height);
        let cx = self.x + dx;
        let cy = self.y + dy;
        ((x - cx).powi(2) + (y - cy).powi(2)).sqrt()
    }
}