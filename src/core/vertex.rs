
use super::{Index, IsValid, ComponentIndex, Component, EdgeIndex, PointIndex, Generation};

/// Handle to Vertex data in a Mesh
pub type VertexIndex = Index<Vertex>;
impl ComponentIndex for VertexIndex {}

/// Represents the point where two edges meet.
#[derive(Default, Debug, Copy, Clone)]
pub struct Vertex {
    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    /// Index of point this vertex belongs to
    pub point_index: PointIndex,
    pub generation: Generation,
}

impl Vertex {
    pub fn from_edge(edge_index: EdgeIndex) -> Vertex {
        Vertex {
            edge_index,
            ..Vertex::default()
        }
    }

    pub fn from_point(point_index: PointIndex) -> Vertex {
        Vertex {
            point_index,
            ..Vertex::default()
        }
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.edge_index.is_valid()
    }
}

impl Component for Vertex {}