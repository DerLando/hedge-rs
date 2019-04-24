
use std::cell::RefCell;
use super::{Vertex, VertexData, IsValid, ElementStatus, EdgeIndex, PointIndex};


impl Vertex {
    pub fn new(edge_index: EdgeIndex, point_index: PointIndex) -> Self {
        Vertex {
            props: RefCell::default(),
            data: RefCell::new(VertexData {
                edge_index,
                point_index,
            }),
        }
    }

    pub fn from_edge(edge_index: EdgeIndex) -> Self {
        Vertex {
            props: RefCell::default(),
            data: RefCell::new(VertexData {
                edge_index,
                point_index: PointIndex::default(),
            }),
        }
    }

    pub fn from_point(point_index: PointIndex) -> Self {
        Vertex {
            props: RefCell::default(),
            data: RefCell::new(VertexData {
                edge_index: EdgeIndex::default(),
                point_index,
            }),
        }
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        let props = self.props.borrow();
        let data = self.data.borrow();
        props.status == ElementStatus::ACTIVE &&
            data.edge_index.is_valid()
    }
}
