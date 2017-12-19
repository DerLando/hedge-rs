
use std::marker;
use std::cmp::Ordering;

pub use self::point::*;
pub use self::vertex::*;
pub use self::face::*;
pub use self::edge::*;

mod point;
mod vertex;
mod face;
mod edge;

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_OFFSET: Offset = 0;

pub type Offset = usize;
pub type Generation = usize;

/// Marker trait for operators using Index types
pub trait ComponentIndex {}

/// Marker trait for operators using Mesh components
/// Components are expected to have a field `generation: Generation`
pub trait Component {}

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}

/// Interface for kernel implementations
pub trait Kernel {
    fn edge_count(&self) -> usize;
    fn face_count(&self) -> usize;
    fn vertex_count(&self) -> usize;
    fn point_count(&self) -> usize;

    fn get_edge(&self, index: EdgeIndex) -> &Edge;
    fn get_face(&self, index: FaceIndex) -> &Face;
    fn get_vertex(&self, index: VertexIndex) -> &Vertex;
    fn get_point(&self, index: PointIndex) -> &Point;

    fn get_edge_mut(&mut self, index: EdgeIndex) -> &mut Edge;
    fn get_face_mut(&mut self, index: FaceIndex) -> &mut Face;
    fn get_vertex_mut(&mut self, index: VertexIndex) -> &mut Vertex;
    fn get_point_mut(&mut self, index: PointIndex) -> &mut Point;

    fn add_edge(&mut self, edge: Edge) -> EdgeIndex;
    fn add_face(&mut self, face: Face) -> FaceIndex;
    fn add_vertex(&mut self, vertex: Vertex) -> VertexIndex;
    fn add_point(&mut self, point: Point) -> PointIndex;

    fn remove_edge(&mut self, index: EdgeIndex);
    fn remove_face(&mut self, index: FaceIndex);
    fn remove_vertex(&mut self, index: VertexIndex);
    fn remove_point(&mut self, index: PointIndex);
}

/// Marker trait for index types.
#[derive(Default, Debug, Clone, Copy)]
pub struct Index<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: marker::PhantomData<T>,
}

impl <T> Index<T> {
    pub fn new(offset: Offset) -> Index<T> {
        Index {
            offset,
            generation: 0,
            _marker: marker::PhantomData::default(),
        }
    }

    pub fn with_generation(offset: Offset, generation: Generation) -> Index<T> {
        Index {
            offset,
            generation,
            _marker: marker::PhantomData::default(),
        }
    }
}

impl <T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Index<T>) -> Option<Ordering> {
        // Only the offset should matter when it comes to ordering
        self.offset.partial_cmp(&other.offset)
    }
}

impl <T> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        self.offset.eq(&other.offset) && self.generation.eq(&other.generation)
    }
}



impl <T> IsValid for Index<T> {
    fn is_valid(&self) -> bool {
        self.offset != INVALID_COMPONENT_OFFSET
    }
}