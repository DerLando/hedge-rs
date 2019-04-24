///
///

use std::cell::RefCell;
use super::{Edge, EdgeData, IsValid, ElementStatus};

impl Edge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        let data = self.data.borrow();
        data.next_index.is_valid() && data.prev_index.is_valid()
    }

    pub fn with_data(data: EdgeData) -> Self {
        Edge {
            data: RefCell::new(data),
            ..Edge::default()
        }
    }
}

impl IsValid for Edge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        let props = self.props.borrow();
        let data = self.data.borrow();
        props.status == ElementStatus::ACTIVE &&
            data.vertex_index.is_valid() &&
            data.twin_index.is_valid() &&
            // TODO: maybe don't use is_connected here?
            self.is_connected()
    }
}
