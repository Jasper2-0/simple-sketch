//! This module provides a rectangle structure and associated operations, building upon the `Point` struct.

use crate::geom::Point;

/// Represents a rectangle in 2D space defined by its minimum (top-left) and maximum (bottom-right) points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub min: Point,
    pub max: Point,
}

impl Rect {
    /// Creates a new `Rect` with the given minimum and maximum points.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// ```
    pub fn new(min: Point, max: Point) -> Self {
        Rect { min, max }
    }

    /// Creates a new `Rect` from any two opposite corners.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::from_points(Point::new(5.0, 5.0), Point::new(0.0, 0.0));
    /// assert_eq!(rect.min, Point::new(0.0, 0.0));
    /// assert_eq!(rect.max, Point::new(5.0, 5.0));
    /// ```
    pub fn from_points(p1: Point, p2: Point) -> Self {
        Rect {
            min: Point::new(p1.x.min(p2.x), p1.y.min(p2.y)),
            max: Point::new(p1.x.max(p2.x), p1.y.max(p2.y)),
        }
    }

    /// Calculates the width of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// assert_eq!(rect.width(), 5.0);
    /// ```
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    /// Calculates the height of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// assert_eq!(rect.height(), 5.0);
    /// ```
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }

    /// Calculates the area of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// assert_eq!(rect.area(), 25.0);
    /// ```
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    /// Calculates the center point of the rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// assert_eq!(rect.center(), Point::new(2.5, 2.5));
    /// ```
    pub fn center(&self) -> Point {
        Point::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
        )
    }

    /// Checks if the rectangle contains a given point.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// assert!(rect.contains_point(&Point::new(2.5, 2.5)));
    /// assert!(!rect.contains_point(&Point::new(6.0, 6.0)));
    /// ```
    pub fn contains_point(&self, point: &Point) -> bool {
        point.x >= self.min.x && point.x <= self.max.x &&
        point.y >= self.min.y && point.y <= self.max.y
    }

    /// Checks if this rectangle intersects with another rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect1 = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// let rect2 = Rect::new(Point::new(3.0, 3.0), Point::new(8.0, 8.0));
    /// let rect3 = Rect::new(Point::new(6.0, 6.0), Point::new(9.0, 9.0));
    /// assert!(rect1.intersects(&rect2));
    /// assert!(!rect1.intersects(&rect3));
    /// ```
    pub fn intersects(&self, other: &Rect) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y
    }

    /// Creates a new rectangle that is the union of this rectangle and another.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect1 = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// let rect2 = Rect::new(Point::new(3.0, 3.0), Point::new(8.0, 8.0));
    /// let union = rect1.union(&rect2);
    /// assert_eq!(union, Rect::new(Point::new(0.0, 0.0), Point::new(8.0, 8.0)));
    /// ```
    pub fn union(&self, other: &Rect) -> Rect {
        Rect {
            min: Point::new(self.min.x.min(other.min.x), self.min.y.min(other.min.y)),
            max: Point::new(self.max.x.max(other.max.x), self.max.y.max(other.max.y)),
        }
    }

    /// Creates a new rectangle that is the intersection of this rectangle and another, if they intersect.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect1 = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// let rect2 = Rect::new(Point::new(3.0, 3.0), Point::new(8.0, 8.0));
    /// let intersection = rect1.intersection(&rect2).unwrap();
    /// assert_eq!(intersection, Rect::new(Point::new(3.0, 3.0), Point::new(5.0, 5.0)));
    /// ```
    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        let min = Point::new(self.min.x.max(other.min.x), self.min.y.max(other.min.y));
        let max = Point::new(self.max.x.min(other.max.x), self.max.y.min(other.max.y));

        if min.x <= max.x && min.y <= max.y {
            Some(Rect { min, max })
        } else {
            None
        }
    }

    /// Creates a new rectangle by translating this rectangle by the given offset.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(5.0, 5.0));
    /// let translated = rect.translate(Point::new(1.0, 1.0));
    /// assert_eq!(translated, Rect::new(Point::new(1.0, 1.0), Point::new(6.0, 6.0)));
    /// ```
    pub fn translate(&self, offset: Point) -> Rect {
        Rect {
            min: self.min + offset,
            max: self.max + offset,
        }
    }

    /// Creates a new rectangle by scaling this rectangle from its center by the given factor.
    ///
    /// # Examples
    ///
    /// ```
    /// let rect = Rect::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// let scaled = rect.scale(2.0);
    /// assert_eq!(scaled, Rect::new(Point::new(-2.0, -2.0), Point::new(6.0, 6.0)));
    /// ```
    pub fn scale(&self, factor: f32) -> Rect {
        let center = self.center();
        let half_size = (self.max - self.min) * 0.5 * factor;
        Rect {
            min: center - half_size,
            max: center + half_size,
        }
    }
}