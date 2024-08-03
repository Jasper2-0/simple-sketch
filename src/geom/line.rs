//! This module provides a 2D line segment structure and associated operations, building upon the `Point` struct.

use crate::geom::Point;

/// Represents a line segment in 2D space defined by its start and end points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    /// Creates a new `Line` with the given start and end points.
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
    /// ```
    pub fn new(start: Point, end: Point) -> Self {
        Line { start, end }
    }

    /// Calculates the length of the line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::new(Point::new(0.0, 0.0), Point::new(3.0, 4.0));
    /// assert_eq!(line.length(), 5.0);
    /// ```
    pub fn length(&self) -> f32 {
        self.start.distance(&self.end)
    }

    /// Calculates the midpoint of the line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// assert_eq!(line.midpoint(), Point::new(2.0, 2.0));
    /// ```
    pub fn midpoint(&self) -> Point {
        Point::new(
            (self.start.x + self.end.x) / 2.0,
            (self.start.y + self.end.y) / 2.0,
        )
    }

    /// Calculates the slope of the line segment.
    /// Returns None if the line is vertical (undefined slope).
    ///
    /// # Examples
    ///
    /// ```
    /// let line1 = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// assert_eq!(line1.slope(), Some(1.0));
    ///
    /// let line2 = Line::new(Point::new(0.0, 0.0), Point::new(0.0, 4.0));
    /// assert_eq!(line2.slope(), None);
    /// ```
    pub fn slope(&self) -> Option<f32> {
        let dx = self.end.x - self.start.x;
        if dx == 0.0 {
            None
        } else {
            Some((self.end.y - self.start.y) / dx)
        }
    }

    /// Determines if this line segment intersects with another line segment.
    ///
    /// # Examples
    ///
    /// ```
    /// let line1 = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// let line2 = Line::new(Point::new(0.0, 4.0), Point::new(4.0, 0.0));
    /// let line3 = Line::new(Point::new(5.0, 5.0), Point::new(6.0, 6.0));
    ///
    /// assert!(line1.intersects(&line2));
    /// assert!(!line1.intersects(&line3));
    /// ```
    pub fn intersects(&self, other: &Line) -> bool {
        // Implementation of line segment intersection
        // This uses the orientations of triplets of points
        fn orientation(p: &Point, q: &Point, r: &Point) -> i8 {
            let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
            if val == 0.0 { 0 } else if val > 0.0 { 1 } else { 2 }
        }

        fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
            q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) &&
            q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
        }

        let o1 = orientation(&self.start, &self.end, &other.start);
        let o2 = orientation(&self.start, &self.end, &other.end);
        let o3 = orientation(&other.start, &other.end, &self.start);
        let o4 = orientation(&other.start, &other.end, &self.end);

        if o1 != o2 && o3 != o4 {
            true
        } else if o1 == 0 && on_segment(&self.start, &other.start, &self.end) {
            true
        } else if o2 == 0 && on_segment(&self.start, &other.end, &self.end) {
            true
        } else if o3 == 0 && on_segment(&other.start, &self.start, &other.end) {
            true
        } else if o4 == 0 && on_segment(&other.start, &self.end, &other.end) {
            true
        } else {
            false
        }
    }

    /// Calculates the intersection point of this line segment with another line segment.
    /// Returns None if the lines do not intersect.
    ///
    /// # Examples
    ///
    /// ```
    /// let line1 = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// let line2 = Line::new(Point::new(0.0, 4.0), Point::new(4.0, 0.0));
    /// let intersection = line1.intersection_point(&line2).unwrap();
    /// assert_eq!(intersection, Point::new(2.0, 2.0));
    ///
    /// let line3 = Line::new(Point::new(5.0, 5.0), Point::new(6.0, 6.0));
    /// assert_eq!(line1.intersection_point(&line3), None);
    /// ```
    pub fn intersection_point(&self, other: &Line) -> Option<Point> {
        if !self.intersects(other) {
            return None;
        }

        let x1 = self.start.x;
        let y1 = self.start.y;
        let x2 = self.end.x;
        let y2 = self.end.y;
        let x3 = other.start.x;
        let y3 = other.start.y;
        let x4 = other.end.x;
        let y4 = other.end.y;

        let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

        if denom == 0.0 {
            return None; // Lines are parallel
        }

        let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;

        Some(Point::new(
            x1 + t * (x2 - x1),
            y1 + t * (y2 - y1)
        ))
    }

    /// Determines the point on this line segment that is closest to the given point.
    ///
    /// # Examples
    ///
    /// ```
    /// let line = Line::new(Point::new(0.0, 0.0), Point::new(4.0, 4.0));
    /// let point = Point::new(0.0, 4.0);
    /// let closest = line.closest_point(&point);
    /// assert_eq!(closest, Point::new(2.0, 2.0));
    /// ```
    pub fn closest_point(&self, point: &Point) -> Point {
        let line_vec = self.end - self.start;
        let point_vec = *point - self.start;

        let t = point_vec.dot(&line_vec) / line_vec.dot(&line_vec);

        if t <= 0.0 {
            self.start
        } else if t >= 1.0 {
            self.end
        } else {
            self.start + line_vec * t
        }
    }
}