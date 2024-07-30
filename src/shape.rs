use crate::point::Point;

pub trait Shape {
    fn contains(&self, point:Point) -> bool;
    fn bounding_box(&self) -> (Point, Point);
    fn distance(&self, point:Point) -> f32;
}

pub struct Ellipse {
    pub center: Point,
    pub width: f32,
    pub height: f32,
}

pub struct Rectangle {
    pub top_left:Point,
    pub width: f32,
    pub height: f32,
}

pub struct Polygon {

}

impl Polygon {
    pub fn vertex() {
        
    }
}

impl Shape for Ellipse {
    fn contains(&self, point:Point) -> bool {
        let dx = (point.x - self.center.x) / (self.width / 2.0);
        let dy = (point.y - self.center.y) / (self.height / 2.0);
        dx * dx + dy * dy <= 1.0
    }

    fn bounding_box(&self) -> (Point, Point) {
        (
            Point::new(self.center.x - self.width / 2.0, self.center.y - self.height / 2.0),
            Point::new(self.center.x + self.width / 2.0, self.center.y + self.height / 2.0),
        )
    }

    fn distance(&self, point: Point) -> f32 {
        let dx = (point.x - self.center.x) / (self.width / 2.0);
        let dy = (point.y - self.center.y) / (self.height / 2.0);
        let distance_squared = dx * dx + dy * dy;
        (distance_squared.sqrt() - 1.0) * (self.width.min(self.height) / 2.0)
    }
}

impl Shape for Rectangle {
    fn contains(&self, point: Point) -> bool {
        point.x >= self.top_left.x && point.x <= self.top_left.x + self.width &&
        point.y >= self.top_left.y && point.y <= self.top_left.y + self.height
    }

    fn bounding_box(&self) -> (Point, Point) {
        (
            self.top_left,
            Point::new(self.top_left.x + self.width, self.top_left.y + self.height),
        )
    }

    fn distance(&self, point: Point) -> f32 {
        let dx = (point.x - self.top_left.x).max(0.0).min(self.width);
        let dy = (point.y - self.top_left.y).max(0.0).min(self.height);
        let closest_point = Point::new(self.top_left.x + dx, self.top_left.y + dy);
        point.distance(&closest_point)
    }
}