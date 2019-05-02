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

pub trait Storable {
    fn generation(&self) -> Generation;
    fn set_generation(&self, generation: Generation);
    fn status(&self) -> ElementStatus;
    fn set_status(&self, status: ElementStatus);
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

/// Trait for accessing Mesh element properties.
#[derive(Debug, Clone)]
pub struct MeshElement<D: ElementData + Default> {
    tag: Cell<Tag>,
    generation: Cell<Generation>,
    status: Cell<ElementStatus>,
    data: RefCell<D>,
}

impl<D: ElementData + Default> Default for MeshElement<D> {
    fn default() -> Self {
        MeshElement {
            tag: Cell::new(0),
            generation: Cell::new(1),
            status: Cell::new(ElementStatus::INACTIVE),
            data: RefCell::default()
        }
    }
}

impl<D: ElementData + Default> MeshElement<D> {
    pub fn with_data(data: D) -> Self {
        MeshElement {
            data: RefCell::new(data),
            ..MeshElement::default()
        }
    }

    pub fn data(&self) -> Ref<D> {
        self.data.borrow()
    }

    pub fn data_mut(&self) -> RefMut<D> {
        self.data.borrow_mut()
    }
}

impl<D: ElementData + Default> Storable for MeshElement<D> {
    fn generation(&self) -> usize {
        self.generation.get()
    }

    fn set_generation(&self, generation: usize) {
        self.generation.set(generation);
    }

    fn status(&self) -> ElementStatus {
        self.status.get()
    }

    fn set_status(&self, status: ElementStatus) {
        self.status.set(status);
    }
}

impl<D: ElementData + Default> Taggable for MeshElement<D> {
    fn tag(&self) -> usize {
        self.tag.get()
    }

    fn set_tag(&self, tag: usize) {
        self.tag.set(tag);
    }
}

impl<D: ElementData + Default> IsActive for MeshElement<D> {
    fn is_active(&self) -> bool {
        self.status.get() == ElementStatus::ACTIVE
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

/// TODO: Documentation
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
mod tests {
    use super::*;
    use log::*;

    #[test]
    fn basic_debug_printing() {
        let _ = env_logger::try_init();

        let edge = Edge::default();
        debug!("{:?}", edge);

        let vertex = Vertex::default();
        debug!("{:?}", vertex);

        let face = Face::default();
        debug!("{:?}", face);

        let point = Point::default();
        debug!("{:?}", point);

        let mesh = Mesh::new();
        debug!("{:?}", mesh);
    }

    #[test]
    fn index_types_are_invalid_by_default() {
        let vert = EdgeIndex::default();
        assert!(!vert.is_valid());

        let edge = EdgeIndex::default();
        assert!(!edge.is_valid());

        let point = PointIndex::default();
        assert!(!point.is_valid());

        let face = FaceIndex::default();
        assert!(!face.is_valid());
    }

    #[test]
    fn default_edge_is_invalid() {
        let edge = Edge::default();
        assert_eq!(edge.is_valid(), false);
    }

    #[test]
    fn default_vertex_is_invalid() {
        let vertex = Vertex::default();
        assert_eq!(vertex.is_valid(), false);
    }

    #[test]
    fn default_face_is_invalid() {
        let face = Face::default();
        assert_eq!(face.is_valid(), false);
    }

    #[test]
    fn default_point_is_invalid() {
        let point = Point::default();
        assert_eq!(point.is_valid(), false);
    }

    #[test]
    fn default_point_is_valid_after_added_to_mesh() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();

        let pindex = {
            let point = Point::default();
            assert_eq!(point.is_valid(), false);
            mesh.add_element(point)
        };

        assert_eq!(mesh.get_element(&pindex).is_some(), true);
    }

    #[test]
    fn initial_mesh_has_default_elements() {
        let _ = env_logger::try_init();
        let mesh = Mesh::new();

        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.get_element(&EdgeIndex::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);

        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.get_element(&FaceIndex::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);

        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.get_element(&VertexIndex::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);

        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.get_element(&PointIndex::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_vertices() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let v0 = mesh.add_element(Vertex::default());
        assert_eq!(mesh.vertex_count(), 1);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 2);
        mesh.remove_element(v0);
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_edges() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let e0 = mesh.add_element(Edge::default());
        assert_eq!(mesh.edge_count(), 1);
        assert_eq!(mesh.kernel.edge_buffer.len(), 2);
        mesh.remove_element(e0);
        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_faces() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let f0 = mesh.add_element(Face::default());
        assert_eq!(mesh.face_count(), 1);
        assert_eq!(mesh.kernel.face_buffer.len(), 2);
        mesh.remove_element(f0);
        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_points() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let p0 = mesh.add_element(Point::default());
        assert_eq!(mesh.point_count(), 1);
        assert_eq!(mesh.kernel.point_buffer.len(), 2);
        mesh.remove_element(p0);
        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[test]
    fn can_add_triangles_to_mesh() {
        let _ = env_logger::try_init();
        unimplemented!();
    }

    #[test]
    fn can_build_a_simple_mesh() {
        unimplemented!();
    }
}
