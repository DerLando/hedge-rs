
use super::{
    Face, FaceData, EdgeIndex,
    IsValid, IsActive
};

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Self {
        Face::with_data(FaceData { edge_index })
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge_index.is_valid()
    }
}

