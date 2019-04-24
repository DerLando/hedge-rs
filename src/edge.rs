///
///

use super::{Edge, IsValid, IsActive};

impl Edge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        let data = self.data.borrow();
        data.next_index.is_valid() && data.prev_index.is_valid()
    }
}

impl IsValid for Edge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        let data = self.data.borrow();
        self.is_active() &&
            data.vertex_index.is_valid() &&
            data.twin_index.is_valid() &&
            data.next_index.is_valid() &&
            data.prev_index.is_valid()
    }
}
