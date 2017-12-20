
use super::{Index, IsValid, ElementIndex, ElementProperties, MeshElement, ElementStatus,
            EdgeIndex, PointIndex};

/// Handle to Vertex data in a Mesh
pub type VertexIndex = Index<Vertex>;
impl ElementIndex for VertexIndex {}

/// Represents the point where two edges meet.
#[derive(Default, Debug, Copy, Clone)]
pub struct Vertex {
    pub _props: ElementProperties,

    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    /// Index of point this vertex belongs to
    pub point_index: PointIndex,
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
        self._props.status == ElementStatus::ACTIVE && self.edge_index.is_valid()
    }
}

impl MeshElement for Vertex {
    fn props(&self) -> &ElementProperties {
        &self._props
    }

    fn props_mut(&mut self) -> &mut ElementProperties {
        &mut self._props
    }
}