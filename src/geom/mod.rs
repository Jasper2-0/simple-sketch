//! A 2D geometry module providing basic structures and operations for points, lines, and rectangles.

mod point;
mod line;
mod rect;

pub use point::Point;
pub use line::Line;
pub use rect::Rect;

// You can add any module-level functions or constants here if needed

/// Calculates the distance between two points.
///
/// This is a convenience function that can be used without directly
/// creating Point instances.
///
/// # Examples
///
/// ```
/// use geom::distance;
///
/// let dist = distance(0.0, 0.0, 3.0, 4.0);
/// assert_eq!(dist, 5.0);
/// ```
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    Point::new(x1, y1).distance(&Point::new(x2, y2))
}