
use nalgebra_glm as glm;
use crate::traits::ElementData;
use crate::handles::{
    HalfEdgeHandle, FaceHandle, VertexHandle, PointHandle,
};

pub type Tag = u32;
pub type Offset = u32;
pub type Generation = u32;
pub type Position = glm::Vec3;
pub type Normal = glm::Vec3;
pub type Color = glm::Vec4;

#[derive(Debug, Clone)]
pub struct UV {
    set: String,
    coord: glm::Vec2,
}

impl Default for UV {
    fn default() -> Self {
        UV {
            set: "Tex0".to_owned(),
            coord: glm::zero(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VertexAttributes {
    position: Position,
    normal: Normal,
    color: Option<Color>,
    uvs: Vec<UV>,
}

impl Default for VertexAttributes {
    fn default() -> Self {
        VertexAttributes {
            position: glm::zero(),
            normal: glm::zero(),
            color: None,
            uvs: Vec::default(),
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
    /// Index of the outgoing edge
    pub edge: HalfEdgeHandle,
    /// Index of point this vertex belongs to
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
    pub normal: Normal,
}

impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: glm::zero(),
            normal: glm::zero(),
        }
    }
}

impl PointData {
    pub fn new(position: Position) -> Self {
        PointData {
            position,
            ..Default::default()
        }
    }
}

impl ElementData for PointData {}
