
use cgmath::Vector3;

use super::{Index, Generation, Component, ComponentIndex};

pub type Position = Vector3<f64>;

/// Handle to Point data in a Mesh
pub type PointIndex = Index<Point>;
impl ComponentIndex for PointIndex {}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub position: Position,
    pub generation: Generation,
}

impl Default for Point {
    fn default() -> Point {
        Point {
            position: Position::new(0.0, 0.0, 0.0),
            generation: 0,
        }
    }
}

impl Component for Point {}