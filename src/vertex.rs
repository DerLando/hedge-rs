

use super::{Vertex, VertexData, IsValid, IsActive, EdgeIndex, PointIndex};


impl Vertex {
    pub fn new(edge_index: EdgeIndex, point_index: PointIndex) -> Self {
        Vertex::with_data(VertexData {
            edge_index,
            point_index,
        })
    }

    pub fn from_edge(edge_index: EdgeIndex) -> Self {
        Vertex::with_data(VertexData {
            edge_index,
            ..VertexData::default()
        })
    }

    pub fn from_point(point_index: PointIndex) -> Self {
        Vertex::with_data(VertexData {
            point_index,
            ..VertexData::default()
        })
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge_index.is_valid()
    }
}
