use std::cell::Cell;
use super::{ElementIndex, ElementProperties, ElementStatus, FaceIndex, Index, IsValid,
            MeshElement, VertexIndex};

/// Handle to Edge data in a Mesh
pub type EdgeIndex = Index<Edge>;

impl ElementIndex for EdgeIndex {}

/// The principle component in a half-edge mesh.
#[derive(Default, Debug, Clone)]
pub struct Edge {
    pub _props: ElementProperties,

    /// The adjacent or 'twin' half-edge
    pub twin_index: Cell<EdgeIndex>,
    /// The index of the next edge in the loop
    pub next_index: Cell<EdgeIndex>,
    /// The index of the previous edge in the loop
    pub prev_index: Cell<EdgeIndex>,

    /// The index of the face this edge loop defines
    pub face_index: Cell<FaceIndex>,

    /// The index of the Vertex for this edge.
    pub vertex_index: Cell<VertexIndex>,
}

impl Edge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        self.next_index.get().is_valid() && self.prev_index.get().is_valid()
    }
}

impl IsValid for Edge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        self._props.status.get() == ElementStatus::ACTIVE && self.vertex_index.get().is_valid()
            && self.twin_index.get().is_valid() && self.is_connected()
    }
}

impl MeshElement for Edge {
    fn props(&self) -> &ElementProperties {
        &self._props
    }
}
