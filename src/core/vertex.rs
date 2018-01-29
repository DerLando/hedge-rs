use std::cell::Cell;
use super::{EdgeIndex, ElementIndex, ElementProperties, ElementStatus, Index, IsValid,
            MeshElement, PointIndex};

/// Handle to Vertex data in a Mesh
pub type VertexIndex = Index<Vertex>;
impl ElementIndex for VertexIndex {}

/// Represents the point where two edges meet.
#[derive(Default, Debug, Clone)]
pub struct Vertex {
    pub _props: ElementProperties,

    /// Index of the outgoing edge
    pub edge_index: Cell<EdgeIndex>,
    /// Index of point this vertex belongs to
    pub point_index: Cell<PointIndex>,
}

impl Vertex {
    pub fn new(edge_index: EdgeIndex, point_index: PointIndex) -> Vertex {
        Vertex {
            edge_index: edge_index.into_cell(),
            point_index: point_index.into_cell(),
            ..Vertex::default()
        }
    }

    pub fn from_edge(edge_index: EdgeIndex) -> Vertex {
        Vertex {
            edge_index: edge_index.into_cell(),
            ..Vertex::default()
        }
    }

    pub fn from_point(point_index: PointIndex) -> Vertex {
        Vertex {
            point_index: point_index.into_cell(),
            ..Vertex::default()
        }
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self._props.status.get() == ElementStatus::ACTIVE && self.edge_index.get().is_valid()
    }
}

impl MeshElement for Vertex {
    fn props(&self) -> &ElementProperties {
        &self._props
    }
}
