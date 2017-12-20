
use cgmath::Vector3;

use super::{Index, MeshElement, ElementProperties, ElementStatus, ElementIndex, IsValid};

pub type Position = Vector3<f64>;

/// Handle to Point data in a Mesh
pub type PointIndex = Index<Point>;
impl ElementIndex for PointIndex {}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub _props: ElementProperties,
    pub position: Position,
}

impl Default for Point {
    fn default() -> Point {
        Point {
            _props: ElementProperties::default(),
            position: Position::new(0.0, 0.0, 0.0),
        }
    }
}

impl IsValid for Point {
    fn is_valid(&self) -> bool {
        self._props.status == ElementStatus::ACTIVE
    }
}

impl MeshElement for Point {
    fn props(&self) -> &ElementProperties {
        &self._props
    }

    fn props_mut(&mut self) -> &mut ElementProperties {
        &mut self._props
    }
}