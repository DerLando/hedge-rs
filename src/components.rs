//! Principle types of the half-edge mesh.


/// Marker trait for index types.
pub trait Handle {}

/// An interface for asserting the validity of components in the mesh.
pub trait IsValid {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}


/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: usize = 0;

/// Type alias for indices into vertex attribute storage
pub type VertexAttributeIndex = usize;

/// Handle to Vertex data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct VertexIndex(pub usize);

impl IsValid for VertexIndex {
    /// A valid VertexIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for VertexIndex {}

/// Handle to Edge data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct EdgeIndex(pub usize);

impl IsValid for EdgeIndex {
    /// A valid EdgeIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for EdgeIndex {}

/// Handle to Face data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FaceIndex(pub usize);

impl IsValid for FaceIndex {
    /// A valid FaceIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for FaceIndex {}

/// Represents the point where two edges meet.
#[derive(Default, Debug)]
pub struct Vertex {
    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    /// Index of this vertex's attributes. _unused currently_
    pub attr_index: VertexAttributeIndex,
}

impl Vertex {
    pub fn new(edge_index: EdgeIndex) -> Vertex {
        Vertex {
            edge_index: edge_index,
            attr_index: INVALID_COMPONENT_INDEX
        }
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.edge_index.is_valid() /*&&
            self.attr_index.is_valid()*/
    }
}


/// The principle component in a half-edge mesh.
#[derive(Default, Debug)]
pub struct Edge {
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
    /// An Edge is valid when it has a twin and a vertex
    fn is_valid(&self) -> bool {
        self.vertex_index.is_valid() && self.twin_index.is_valid()
    }
}


/// A face is defined by the looping connectivity of edges.
#[derive(Default, Debug)]
pub struct Face {
    /// The "root" of an edge loop that defines this face.
    pub edge_index: EdgeIndex,
}

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Face {
        Face {
            edge_index
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
