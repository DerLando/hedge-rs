
use std::cell::Cell;
use super::{Index, IsValid, ElementIndex, ElementProperties, MeshElement, ElementStatus,
            EdgeIndex};


/// Handle to Face data in a Mesh
pub type FaceIndex = Index<Face>;
impl ElementIndex for FaceIndex {}

/// A face is defined by the looping connectivity of edges.
#[derive(Default, Debug, Clone)]
pub struct Face {
    pub _props: ElementProperties,

    /// The "root" of an edge loop that defines this face.
    pub edge_index: Cell<EdgeIndex>,
}

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Face {
        Face {
            _props: ElementProperties::default(),
            edge_index: edge_index.into_cell(),
        }
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self._props.status.get() == ElementStatus::ACTIVE && self.edge_index.get().is_valid()
    }
}

impl MeshElement for Face {
    fn props(&self) -> &ElementProperties {
        &self._props
    }
}
