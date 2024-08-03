use crate::point::Point;

/// Represents a geometric shape in 2D space.
pub trait Shape {
    /// Determines if a point is contained within the shape.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// `true` if the point is inside or on the boundary of the shape, `false` otherwise.
    fn contains(&self, point: Point) -> bool;

    /// Calculates the bounding box of the shape.
    ///
    /// # Returns
    ///
    /// A tuple of two `Point`s representing the top-left and bottom-right
    /// corners of the bounding box that fully encloses the shape.
    fn bounding_box(&self) -> (Point, Point);

    /// Calculates the shortest distance from a point to the shape's boundary.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to calculate the distance from.
    ///
    /// # Returns
    ///
    /// The shortest distance from the point to the shape's boundary.
    /// May be negative if the point is inside the shape, depending on the implementation.
    fn distance(&self, point: Point) -> f32;
}

/// Represents an ellipse in 2D space.
pub struct Ellipse {
    /// The center point of the ellipse.
    pub center: Point,
    /// The width of the ellipse (diameter along the x-axis).
    pub width: f32,
    /// The height of the ellipse (diameter along the y-axis).
    pub height: f32,
}

/// Represents a rectangle in 2D space.
pub struct Rectangle {
    /// The top-left corner of the rectangle.
    pub top_left: Point,
    /// The width of the rectangle.
    pub width: f32,
    /// The height of the rectangle.
    pub height: f32,
}


pub struct Polygon {

}

impl Polygon {
    #[allow(dead_code)]
    pub fn vertex() {

    }
}

/// Implements the `Shape` trait for an `Ellipse`.
///
/// An `Ellipse` is defined by its center point, width, and height.
impl Shape for Ellipse {
    /// Determines if a point is contained within the ellipse.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// `true` if the point is inside or on the ellipse, `false` otherwise.
    fn contains(&self, point: Point) -> bool {
        // Normalize the point relative to the ellipse's center and dimensions
        let dx = (point.x - self.center.x) / (self.width / 2.0);
        let dy = (point.y - self.center.y) / (self.height / 2.0);
        
        // Check if the normalized point satisfies the ellipse equation
        dx * dx + dy * dy <= 1.0
    }

    /// Calculates the bounding box of the ellipse.
    ///
    /// # Returns
    ///
    /// A tuple of two `Point`s representing the top-left and bottom-right
    /// corners of the bounding box.
    fn bounding_box(&self) -> (Point, Point) {
        (
            Point::new(self.center.x - self.width / 2.0, self.center.y - self.height / 2.0),
            Point::new(self.center.x + self.width / 2.0, self.center.y + self.height / 2.0),
        )
    }

    /// Calculates the shortest distance from a point to the ellipse's boundary.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to calculate the distance from.
    ///
    /// # Returns
    ///
    /// The shortest distance from the point to the ellipse's boundary.
    /// Negative if the point is inside the ellipse.
    fn distance(&self, point: Point) -> f32 {
        // Normalize the point relative to the ellipse's center and dimensions
        let dx = (point.x - self.center.x) / (self.width / 2.0);
        let dy = (point.y - self.center.y) / (self.height / 2.0);
        
        // Calculate the distance using the ellipse's equation
        let distance_squared = dx * dx + dy * dy;
        (distance_squared.sqrt() - 1.0) * (self.width.min(self.height) / 2.0)
    }
}

/// Implements the `Shape` trait for a `Rectangle`.
impl Shape for Rectangle {
    /// Determines if a point is contained within the rectangle.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to check.
    ///
    /// # Returns
    ///
    /// `true` if the point is inside or on the boundary of the rectangle, `false` otherwise.
    fn contains(&self, point: Point) -> bool {
        point.x >= self.top_left.x && point.x <= self.top_left.x + self.width &&
        point.y >= self.top_left.y && point.y <= self.top_left.y + self.height
    }

    /// Calculates the bounding box of the rectangle.
    ///
    /// # Returns
    ///
    /// A tuple of two `Point`s representing the top-left and bottom-right
    /// corners of the bounding box. For a rectangle, this is simply its own corners.
    fn bounding_box(&self) -> (Point, Point) {
        (
            self.top_left,
            Point::new(self.top_left.x + self.width, self.top_left.y + self.height),
        )
    }

    /// Calculates the shortest distance from a point to the rectangle's boundary.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to calculate the distance from.
    ///
    /// # Returns
    ///
    /// The shortest distance from the point to the rectangle's boundary.
    /// Returns 0 if the point is inside the rectangle.
    fn distance(&self, point: Point) -> f32 {
        // Calculate the closest point on the rectangle to the given point
        let dx = (point.x - self.top_left.x).max(0.0).min(self.width);
        let dy = (point.y - self.top_left.y).max(0.0).min(self.height);
        let closest_point = Point::new(self.top_left.x + dx, self.top_left.y + dy);

        // Calculate the distance between the given point and the closest point
        point.distance(&closest_point)
    }
}