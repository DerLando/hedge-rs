
use super::{Index, IsValid, ComponentIndex, Component, EdgeIndex, Generation};


/// Handle to Face data in a Mesh
pub type FaceIndex = Index<Face>;
impl ComponentIndex for FaceIndex {}

/// A face is defined by the looping connectivity of edges.
#[derive(Default, Debug, Copy, Clone)]
pub struct Face {
    /// The "root" of an edge loop that defines this face.
    pub edge_index: EdgeIndex,

    pub generation: Generation,
}

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Face {
        Face {
            edge_index,
            generation: 0,
        }
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.edge_index.is_valid()
    }
}

impl Component for Face {}