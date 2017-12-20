
use super::{Index, IsValid, ElementIndex, ElementProperties, MeshElement, ElementStatus,
            VertexIndex, FaceIndex};

/// Handle to Edge data in a Mesh
pub type EdgeIndex = Index<Edge>;
impl ElementIndex for EdgeIndex {}

/// The principle component in a half-edge mesh.
#[derive(Default, Debug, Copy, Clone)]
pub struct Edge {
    pub _props: ElementProperties,

    /// The adjacent or 'twin' half-edge
    pub twin_index: EdgeIndex,
    /// The index of the next edge in the loop
    pub next_index: EdgeIndex,
    /// The index of the previous edge in the loop
    pub prev_index: EdgeIndex,

    /// The index of the face this edge loop defines
    pub face_index: FaceIndex,

    /// The index of the Vertex for this edge.
    pub vertex_index: VertexIndex,
}

impl Edge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        self.next_index.is_valid() && self.prev_index.is_valid()
    }
}

impl IsValid for Edge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        self._props.status == ElementStatus::ACTIVE &&
            self.vertex_index.is_valid() &&
            self.twin_index.is_valid() &&
            self.is_connected()
    }
}

impl MeshElement for Edge {
    fn props(&self) -> &ElementProperties {
        &self._props
    }

    fn props_mut(&mut self) -> &mut ElementProperties {
        &mut self._props
    }
}