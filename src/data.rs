use crate::handles::{FaceHandle, HalfEdgeHandle, PointHandle, VertexHandle};
use crate::traits::ElementData;
use nalgebra as na;

pub type Tag = u32;
pub type Index = u32;
pub type Generation = u32;
pub type Position = na::Point3<f32>;
pub type Normal = na::Vector3<f32>;
pub type Color = na::Vector4<f32>;

#[derive(Debug, Clone)]
pub struct VertexAttributes {
    normal: Normal,
    color: Color,
}

impl Default for VertexAttributes {
    fn default() -> Self {
        VertexAttributes {
            normal: na::zero(),
            color: na::Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

#[derive(Debug, Clone, Default)]
pub struct HalfEdgeData {
    /// The adjacent half-edge
    pub adjacent: HalfEdgeHandle,
    /// The Handle of the next edge in the loop
    pub next: HalfEdgeHandle,
    /// The Handle of the previous edge in the loop
    pub prev: HalfEdgeHandle,
    /// The Handle of the face this edge loop defines
    pub face: FaceHandle,
    /// The Handle of the Vertex for this edge.
    pub vertex: VertexHandle,
}
impl ElementData for HalfEdgeData {}

#[derive(Debug, Clone, Default)]
pub struct VertexData {
    /// Handle of the outgoing edge
    pub edge: HalfEdgeHandle,
    /// Handle of point this vertex belongs to
    pub point: PointHandle,
    /// Vertex attributes
    pub attrs: VertexAttributes,
}
impl ElementData for VertexData {}

#[derive(Debug, Clone, Default)]
pub struct FaceData {
    /// The "root" of an edge loop that defines this face.
    pub root_edge: HalfEdgeHandle,
}
impl ElementData for FaceData {}

#[derive(Debug, Clone)]
pub struct PointData {
    pub position: Position,
    // TODO: vertices
}

impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: na::Point3::new(0.0, 0.0, 0.0),
        }
    }
}

impl PointData {
    pub fn new(position: Position, _normal: Normal) -> Self {
        PointData { position }
    }

    pub fn from_position(position: Position) -> Self {
        PointData {
            position,
            //..Default::default()
        }
    }
}

impl ElementData for PointData {}

#[allow(unused)]
struct TriangleList {
    indices: Vec<u16>,
    vertices: Vec<f32>,
}
