//!
//! An index based half-edge mesh implementation.
//!

use std::fmt;
use std::sync::atomic;
use std::cmp;
use std::cell::{Cell, RefCell, Ref, RefMut};
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};

pub use crate::edge::*;
pub use crate::face::*;
pub use crate::vertex::*;
pub use crate::point::*;
pub use crate::kernel::*;
pub use crate::function_sets::*;
pub use crate::iterators::*;

pub mod edge;
pub mod face;
pub mod vertex;
pub mod point;
pub mod kernel;
pub mod utils;
pub mod function_sets;
pub mod iterators;

pub type Tag = usize;
pub type Offset = usize;
pub type Generation = usize;
pub type Position = [f32; 3];
pub type Normal = [f32; 3];

////////////////////////////////////////////////////////////////////////////////

/// Marker trait for Index types
pub trait ElementIndex {}

/// Marker trait for structs holding element specific data
pub trait ElementData {}

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

pub trait IsActive {
    fn is_active(&self) -> bool;
}

pub trait Taggable {
    fn tag(&self) -> Tag;
    fn set_tag(&self, tag: Tag);
}

pub trait Indexable {
    fn generation(&self) -> Generation;
    fn set_generation(&self, generation: Generation);
}

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_OFFSET: Offset = 0;

/// Type-safe index into kernel storage.
#[derive(Default, Debug, Clone, Eq)]
pub struct Index<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: PhantomData<T>,
}

impl<T: Clone> Copy for Index<T> {}

impl<T> Hash for Index<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.offset.hash(state);
        self.generation.hash(state);
    }
}

impl<T> Index<T> {
    pub fn new(offset: Offset) -> Index<T> {
        Index {
            offset,
            generation: 0,
            _marker: PhantomData::default(),
        }
    }

    pub fn with_generation(offset: Offset, generation: Generation) -> Index<T> {
        Index {
            offset,
            generation,
            _marker: PhantomData::default(),
        }
    }
}

impl<T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Index<T>) -> Option<cmp::Ordering> {
        // Only the offset should matter when it comes to ordering
        self.offset.partial_cmp(&other.offset)
    }
}

impl<T> PartialEq for Index<T> {
    fn eq(&self, other: &Index<T>) -> bool {
        self.offset.eq(&other.offset) && self.generation.eq(&other.generation)
    }
}

impl<T> IsValid for Index<T> {
    fn is_valid(&self) -> bool {
        self.offset != INVALID_COMPONENT_OFFSET
    }
}

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

/// The 3 fields our element buffers needs to do its work
#[derive(Debug, Clone)]
pub struct ElementProperties {
    pub generation: Generation,
    pub status: ElementStatus,
    pub tag: Tag,
}

/// A custom impl for Default so that generation defaults to 1
impl Default for ElementProperties {
    fn default() -> Self {
        ElementProperties {
            generation: 1,
            status: ElementStatus::INACTIVE,
            tag: 0,
        }
    }
}

/// Trait for accessing Mesh element properties.
#[derive(Debug, Clone, Default)]
pub struct MeshElement<D: ElementData + Default> {
    pub props: RefCell<ElementProperties>,
    data: RefCell<D>,
}

impl<D: ElementData + Default> MeshElement<D> {
    pub fn data(&self) -> Ref<D> {
        self.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<D> {
        self.data.borrow_mut()
    }
}

impl<D: ElementData + Default> Indexable for MeshElement<D> {
    fn generation(&self) -> usize {
        self.props.borrow().generation
    }

    fn set_generation(&self, generation: usize) {
        self.props.borrow_mut().generation = generation;
    }
}

impl<D: ElementData + Default> Taggable for MeshElement<D> {
    fn tag(&self) -> usize {
        self.props.borrow().tag
    }

    fn set_tag(&self, tag: usize) {
        self.props.borrow_mut().tag = tag;
    }
}

impl<D: ElementData + Default> IsActive for MeshElement<D> {
    fn is_active(&self) -> bool {
        let props = self.props.borrow();
        props.status == ElementStatus::ACTIVE
    }
}

/// TODO
#[derive(Debug, Clone, Default)]
pub struct EdgeData {
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
pub type Edge = MeshElement<EdgeData>;
pub type EdgeIndex = Index<Edge>;
impl ElementData for EdgeData {}
impl ElementIndex for  EdgeIndex {}

/// TODO
#[derive(Debug, Clone, Default)]
pub struct VertexData {
    /// Index of the outgoing edge
    pub edge_index: EdgeIndex,
    /// Index of point this vertex belongs to
    pub point_index: PointIndex,
}
pub type Vertex = MeshElement<VertexData>;
pub type VertexIndex = Index<Vertex>;
impl ElementData for VertexData {}
impl ElementIndex for VertexIndex {}

/// TODO
#[derive(Debug, Clone, Default)]
pub struct FaceData {
    /// The "root" of an edge loop that defines this face.
    pub edge_index: EdgeIndex,
}
pub type Face = MeshElement<FaceData>;
pub type FaceIndex = Index<Face>;
impl ElementData for FaceData {}
impl ElementIndex for FaceIndex {}

#[derive(Debug, Clone)]
pub struct PointData {
    pub position: Position,
}
impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: [0.0; 3],
        }
    }
}
pub type Point = MeshElement<PointData>;
pub type PointIndex = Index<Point>;
impl ElementData for PointData {}
impl ElementIndex for PointIndex {}

////////////////////////////////////////////////////////////////////////////////

/// Interface for adding elements to a `Mesh`.
pub trait AddElement<E> {
    fn add_element(&mut self, element: E) -> Index<E>;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<E> {
    fn remove_element(&mut self, index: Index<E>);
}

/// Interface for getting an element reference.
pub trait GetElement<E> {
    fn get_element(&self, index: &Index<E>) -> Option<&E>;
}

pub struct Mesh {
    kernel: Kernel,
    tag: atomic::AtomicUsize,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mesh {{ {} points, {} vertices, {} edges, {} faces }}",
            self.point_count(),
            self.vertex_count(),
            self.edge_count(),
            self.face_count()
        )
    }
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            kernel: Kernel::default(),
            tag: atomic::AtomicUsize::new(1),
        }
    }

    fn next_tag(&self) -> usize {
        self.tag.fetch_add(1, atomic::Ordering::SeqCst)
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceIndex) -> FaceFn {
        FaceFn::new(index, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_buffer.len() - 1
    }

    //pub fn faces(&self) -> FaceFnIterator {
    //    FaceFnIterator::new(&self)
    //}

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeIndex) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    //pub fn edges(&self) -> EdgeFnIterator {
    //    EdgeFnIterator::new(&self)
    //}

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexIndex) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    //pub fn vertices(&self) -> VertexFnIterator {
    //    VertexFnIterator::new(&self)
    //}

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }

    pub fn add_element<E>(&mut self, element: E) -> Index<E>
        where kernel::Kernel: AddElement<E>
    {
        self.kernel.add_element(element)
    }

    pub fn remove_element<E>(&mut self, index: Index<E>)
        where kernel::Kernel: RemoveElement<E>
    {
        self.kernel.remove_element(index)
    }

    pub fn get_element<E>(&self, index: &Index<E>) -> Option<&E>
        where kernel::Kernel: GetElement<E>
    {
        self.kernel.get_element(index)
    }
}

#[cfg(test)]
mod tests;
