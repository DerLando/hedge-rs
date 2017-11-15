
//!
//! An index based half-edge mesh implementation.
//!
//! `Mesh` implements the fundamental storage operations and represents the principle
//! grouping of all components. Most operations available are trait impls for `Mesh`.

#[macro_use]
extern crate log;
#[cfg(not(feature = "with_nalgebra"))]
extern crate cgmath;
#[cfg(feature = "with_nalgebra")]
extern crate nalgebra;

use std::fmt;

pub use iterators::*;
pub use operations::*;
pub use function_sets::*;

pub mod operations;
pub mod iterators;
pub mod function_sets;

/// Marker trait for index types.
pub trait Handle {}

/// An interface for asserting the validity of components in the mesh.
pub trait IsValid {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: usize = 0;

/// Handle to Vertex data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct VertexIndex(usize);

impl IsValid for VertexIndex {
    /// A valid VertexIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for VertexIndex {}

/// Handle to Edge data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct EdgeIndex(usize);

impl IsValid for EdgeIndex {
    /// A valid EdgeIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for EdgeIndex {}

/// Handle to Face data in a Mesh
#[derive(Default, Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct FaceIndex(usize);

impl IsValid for FaceIndex {
    /// A valid FaceIndex has an index that does not equal INVALID_COMPONENT_INDEX
    fn is_valid(&self) -> bool {
        self.0 != INVALID_COMPONENT_INDEX
    }
}

impl Handle for FaceIndex {}

#[cfg(not(feature = "with_nalgebra"))]
pub type Point = cgmath::Vector3<f64>;
#[cfg(feature = "with_nalgebra")]
pub type Point = nalgebra::Point3<f64>;

/// Represents the point where two edges meet.
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    pub point: Point,
}

impl Vertex {
    pub fn new(edge_index: EdgeIndex) -> Vertex {
        Vertex {
            edge_index,
            point: default_point(),
        }
    }

    pub fn from_point(point: Point) -> Vertex {
        Vertex {
            edge_index: EdgeIndex::default(),
            point,
        }
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.edge_index.is_valid()
    }
}

#[cfg(not(feature="with_nalgebra"))]
fn default_point() -> Point {
    Point {
        x: 0.0, y: 0.0, z: 0.0
    }
}
#[cfg(feature="with_nalgebra")]
fn default_point() -> Point {
    Point::new(0.0, 0.0, 0.0)
}

impl Default for Vertex {
    fn default() -> Vertex {
        Vertex {
            edge_index: EdgeIndex(INVALID_COMPONENT_INDEX),
            point: default_point(),
        }
    }
}


/// The principle component in a half-edge mesh.
#[derive(Default, Debug, Copy, Clone)]
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
#[derive(Default, Debug, Copy, Clone)]
pub struct Face {
    /// The "root" of an edge loop that defines this face.
    pub edge_index: EdgeIndex,
}

impl Face {
    pub fn new(edge_index: EdgeIndex) -> Face {
        Face { edge_index }
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.edge_index.is_valid()
    }
}

pub type EdgeList = Vec<Edge>;
pub type FaceList = Vec<Face>;
pub type VertexList = Vec<Vertex>;

#[derive(Clone)]
pub struct Mesh {
    edge_list: EdgeList,
    face_list: FaceList,
    vertex_list: VertexList,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Half-Edge Mesh {{ {} vertices, {} edges, {} faces }}",
               self.num_vertices(), self.num_edges(), self.num_faces())
    }
}

impl Mesh {
    /// Creates a new Mesh with an initial component added to each Vec.
    ///
    /// The idea behind having a single invalid component at the front of each
    /// Vec comes from the blog http://ourmachinery.com/post/defaulting-to-zero/
    pub fn new() -> Mesh {
        Mesh {
            edge_list: vec! [ Edge::default() ],
            face_list: vec! [ Face::default() ],
            vertex_list: vec! [ Vertex::default() ],
        }
    }

    /// Connects the two edges as part of an edge loop.
    ///
    /// _In debug builds we assert that neither index is the default index._
    pub fn connect_edges(&mut self, prev: EdgeIndex, next: EdgeIndex) {
        debug_assert!(prev.is_valid());
        debug_assert!(next.is_valid());
        self.edge_mut(prev).next_index = next;
        self.edge_mut(next).prev_index = prev;
        trace!("Connected {:?} -> {:?}", prev, next);
    }

    pub fn is_boundary_edge(&self, eindex: EdgeIndex) -> bool {
        debug_assert!(eindex.is_valid());
        debug_assert!(self.edge(eindex).is_valid());
        debug_assert!(self.edge_fn(eindex).twin().is_valid());
        !self.edge_fn(eindex).face().is_valid() ||
            !self.edge_fn(eindex).twin().face().is_valid()
    }

    pub fn foreach_edge_mut<F>(&mut self, eindex: EdgeIndex, mut callback: F)
        where F: FnMut(&mut Edge)
    {
        let edge_indices: Vec<EdgeIndex> = EdgeLoop::new(eindex, &self.edge_list).collect();
        for index in edge_indices {
            let edge = &mut self.edge_mut(index);
            callback(edge);
        }
    }

    /// Returns a `Faces` iterator for this mesh.
    ///
    /// ```
    /// use hedge::*;
    /// let mesh = Mesh::new();
    /// for index in mesh.faces() {
    ///    let face = &mesh.face(index);
    /// }
    /// ```
    pub fn faces(&self) -> Faces {
        Faces::new(self.face_list.len())
    }

    /// Returns an `EdgeLoop` iterator for the edges around the specified face.
    ///
    /// ```
    /// use hedge::*;
    /// let mesh = Mesh::new();
    /// for findex in mesh.faces() {
    ///    let face = &mesh.face(findex);
    ///    for eindex in mesh.edges(face) {
    ///        let edge = &mesh.edge(eindex);
    ///    }
    /// }
    /// ```
    pub fn edges(&self, face: &Face) -> EdgeLoop {
        EdgeLoop::new(face.edge_index, &self.edge_list)
    }

    /// Returns an `EdgeLoopVertices` iterator for the vertices around the specified face.
    ///
    /// ```
    /// use hedge::*;
    /// let mesh = Mesh::new();
    /// for findex in mesh.faces() {
    ///    let face = &mesh.face(findex);
    ///    for vindex in mesh.vertices(face) {
    ///        let vertex = &mesh.vertex(vindex);
    ///    }
    /// }
    /// ```
    pub fn vertices(&self, face: &Face) -> EdgeLoopVertices {
        EdgeLoopVertices::new(face.edge_index, &self.edge_list)
    }

    pub fn edges_around_vertex(&self, vertex: &Vertex) -> EdgesAroundVertex {
        EdgesAroundVertex::new(vertex.edge_index, &self)
    }

    /// Returns a `FaceFn` for the given index.
    ///
    /// ```
    /// use hedge::*;
    /// let mut mesh = Mesh::new();
    ///
    /// let v1 = mesh.add(Vertex::default());
    /// let v2 = mesh.add(Vertex::default());
    /// let v3 = mesh.add(Vertex::default());
    ///
    /// let f1 = mesh.add(triangle::FromVerts(v1, v2, v3));
    ///
    /// assert_eq!(mesh.face_fn(f1).edge().next().vertex().index, v2);
    /// ```
    pub fn face_fn(&self, index: FaceIndex) -> FaceFn {
        FaceFn::new(index, &self)
    }

    pub fn face_mut(&mut self, index: FaceIndex) -> &mut Face {
        &mut self.face_list[index.0]
    }

    pub fn face(&self, index: FaceIndex) -> &Face {
        if let Some(ref result) = self.face_list.get(index.0) {
            result
        } else {
            &self.face_list[0]
        }
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge_fn(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_mut(&mut self, index: EdgeIndex) -> &mut Edge {
        &mut self.edge_list[index.0]
    }

    pub fn edge(&self, index: EdgeIndex) -> &Edge {
        if let Some(result) = self.edge_list.get(index.0) {
            result
        } else {
            trace!("Unable to find an edge at {:?}", index);
            &self.edge_list[0]
        }
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex_fn(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_mut(&mut self, index: VertexIndex) -> &mut Vertex {
        &mut self.vertex_list[index.0]
    }

    pub fn vertex(&self, index: VertexIndex) -> &Vertex {
        if let Some(result) = self.vertex_list.get(index.0) {
            result
        } else {
            &self.vertex_list[0]
        }
    }

    pub fn num_vertices(&self) -> usize {
        self.vertex_list.len() - 1
    }

    pub fn num_faces(&self) -> usize {
        self.face_list.len() - 1
    }

    pub fn num_edges(&self) -> usize {
        self.edge_list.len() - 1
    }
}


#[cfg(test)]
mod tests;
