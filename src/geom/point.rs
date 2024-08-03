//! This module provides a 2D point structure and associated operations.

use std::ops::{Add, Sub, Mul};

/// Represents a point in 2D space with x and y coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    /// Creates a new `Point` with the given x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.x, 3.0);
    /// assert_eq!(p.y, 4.0);
    /// ```
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    /// Calculates the length (magnitude) of the vector from the origin to this point.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(3.0, 4.0);
    /// assert_eq!(p.length(), 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns a new `Point` that is the normalized version of this point (same direction, but length 1).
    /// If the point is (0, 0), it returns itself.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(3.0, 4.0);
    /// let normalized = p.normalize();
    /// assert_eq!(normalized.length(), 1.0);
    /// ```
    pub fn normalize(&self) -> Point {
        let length = self.length();
        if length == 0.0 {
            *self
        } else {
            Point {
                x: self.x / length,
                y: self.y / length,
            }
        }
    }

    /// Returns a new `Point` with the absolute values of x and y coordinates.
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(-3.0, 4.0);
    /// let abs_p = p.abs();
    /// assert_eq!(abs_p, Point::new(3.0, 4.0));
    /// ```
    pub fn abs(&self) -> Point {
        Point {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a new `Point` that is perpendicular to this point (rotated 90 degrees counterclockwise).
    ///
    /// # Examples
    ///
    /// ```
    /// let p = Point::new(3.0, 4.0);
    /// let perp = p.perpendicular();
    /// assert_eq!(perp, Point::new(-4.0, 3.0));
    /// ```
    pub fn perpendicular(&self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    /// Calculates the Euclidean distance between this point and another point.
    ///
    /// # Examples
    ///
    /// ```
    /// let p1 = Point::new(0.0, 0.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.distance(&p2), 5.0);
    /// ```
    pub fn distance(&self, other: &Point) -> f32 {
        (*self - *other).length()
    }

    /// Calculates the dot product of this point and another point.
    ///
    /// # Examples
    ///
    /// ```
    /// let p1 = Point::new(1.0, 2.0);
    /// let p2 = Point::new(3.0, 4.0);
    /// assert_eq!(p1.dot(&p2), 11.0);
    /// ```
    pub fn dot(&self, other: &Point) -> f32 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Point {
    type Output = Point;

    fn mul(self, scalar: f32) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Point> for f32 {
    type Output = Point;

    fn mul(self, point: Point) -> Point {
        Point {
            x: self * point.x,
            y: self * point.y,
        }
    }
}