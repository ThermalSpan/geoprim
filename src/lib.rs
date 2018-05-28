extern crate serde;
#[macro_use]
extern crate serde_derive;

/// A simple point 3D point
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Point { x: f32, y: f32, z: f32 }

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point { x, y, z}
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct LineSegment {p1: Point, p2: Point}

impl LineSegment {
    pub fn new(p1: Point, p2: Point) -> LineSegment {
        LineSegment { p1, p2}
    }
}
#[derive(Serialize, Deserialize)]
pub struct Plot {
    point: Vec<Point>,
    lines: Vec<LineSegment>
}
