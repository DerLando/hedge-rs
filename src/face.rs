
use std::cell::RefCell;
use super::{
    Face, FaceData, FaceIndex, EdgeIndex,
    IsValid, ElementProperties, ElementStatus
};

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Self {
        Face {
            props: RefCell::default(),
            data: RefCell::new(FaceData {
                edge_index,
            }),
        }
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        let props = self.props.borrow();
        let data = self.data.borrow();
        props.status == ElementStatus::ACTIVE &&
            data.edge_index.is_valid()
    }
}

