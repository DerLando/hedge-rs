
use crate::traits::ElementData;
use crate::handles::{
    HalfEdgeHandle, FaceHandle, VertexHandle, PointHandle,
};

pub type Tag = u32;
pub type Offset = u32;
pub type Generation = u32;
pub type Position = [f32; 3];
pub type Normal = [f32; 3];

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

/// TODO: Documentation
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

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct VertexData {
    /// Index of the outgoing edge
    pub edge: HalfEdgeHandle,
    /// Index of point this vertex belongs to
    pub point: PointHandle,
}
impl ElementData for VertexData {}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct FaceData {
    /// The "root" of an edge loop that defines this face.
    pub root_edge: HalfEdgeHandle,
}
impl ElementData for FaceData {}

#[derive(Debug, Clone)]
pub struct PointData {
    pub position: Position,
}
impl PointData {
    pub fn new(position: Position) -> Self {
        PointData { position }
    }
}
impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: [0.0; 3],
        }
    }
}
impl ElementData for PointData {}
