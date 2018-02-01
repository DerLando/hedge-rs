use std::fmt;
use std::marker;
use std::cmp::Ordering;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Enumerate;
use std::hash::{Hash, Hasher};
use cgmath::Vector3;

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

pub type Tag = usize;
pub type Offset = usize;
pub type Generation = usize;
pub type Position = Vector3<f64>;
pub type Normal = Vector3<f64>;

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    /// A general blanket test for validity
    fn is_valid(&self) -> bool;
}

/// Marker trait for index types.
#[derive(Default, Debug, Clone, Eq)]
pub struct Index<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: marker::PhantomData<T>,
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

    pub fn into_cell(self) -> Cell<Index<T>> {
        Cell::new(self)
    }
}

impl<T> PartialOrd for Index<T> {
    fn partial_cmp(&self, other: &Index<T>) -> Option<Ordering> {
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

/// Marker trait for operators using Index types
pub trait ElementIndex {}

/// Marker trait for operators using Mesh components
/// Components are expected to have a field `component: ComponentProperties`
pub trait MeshElement: Default {
    fn props(&self) -> &ElementProperties;
}

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}

impl Default for ElementStatus {
    fn default() -> Self {
        ElementStatus::INACTIVE
    }
}

/// The 3 fields our component buffers needs to do its work
#[derive(Debug, Default, Clone)]
pub struct ElementProperties {
    pub generation: Cell<Generation>,
    pub status: Cell<ElementStatus>,
    pub tag: Cell<Tag>,
}

///
/// Blah blah blah
///
pub struct ElementBuffer<T: MeshElement + Default> {
    pub free_cells: Vec<Index<T>>,
    pub buffer: Vec<T>,
}

impl<T: MeshElement + Default> Default for ElementBuffer<T> {
    fn default() -> ElementBuffer<T> {
        ElementBuffer {
            free_cells: Vec::new(),
            buffer: vec![T::default()],
        }
    }
}

impl<T: MeshElement + Default> fmt::Debug for ElementBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComponentBuffer<> {{ {} items }}", self.len())
    }
}

impl<T: MeshElement + Default> ElementBuffer<T> {
    pub fn len(&self) -> usize {
        self.buffer.len() - self.free_cells.len()
    }

    pub fn enumerate<'mesh>(&'mesh self) -> Enumerate<Iter<'mesh, T>> {
        let mut it = self.buffer.iter().enumerate();
        let _ = it.next(); // Always skip the first element since we know it's invalid
        return it;
    }

    pub fn get(&self, index: &Index<T>) -> &T {
        let mut result = &self.buffer[0];
        if let Some(element) = self.buffer.get(index.offset) {
            if index.generation == element.props().generation.get()
                && element.props().status.get() == ElementStatus::ACTIVE
            {
                result = element;
            }
        }
        return result;
    }

    pub fn get_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        let element = &mut self.buffer[index.offset];
        if element.props().generation.get() == index.generation
            && element.props().status.get() == ElementStatus::ACTIVE
        {
            Some(element)
        } else {
            None
        }
    }

    pub fn add(&mut self, element: T) -> Index<T> {
        if let Some(index) = self.free_cells.pop() {
            let cell = &mut self.buffer[index.offset];
            *cell = element;
            let props = cell.props();
            props.generation.set(index.generation);
            props.status.set(ElementStatus::ACTIVE);
            return index;
        } else {
            let index = Index::with_generation(self.buffer.len(), element.props().generation.get());
            element.props().status.set(ElementStatus::ACTIVE);
            self.buffer.push(element);
            return index;
        }
    }

    pub fn remove(&mut self, index: Index<T>) {
        let removed = self.get_mut(&index).map(|cell| {
            let props = cell.props();
            props.generation.set(index.generation + 1);
            props.status.set(ElementStatus::INACTIVE);
            Index::with_generation(index.offset, index.generation + 1)
        });
        if let Some(removed) = removed {
            self.free_cells.push(removed);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Interface for adding elements to a `Mesh`.
pub trait AddElement<E: MeshElement> {
    fn add(&mut self, element: E) -> Index<E>;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<E: MeshElement> {
    fn remove(&mut self, index: Index<E>);
}

/// Interface for getting an element reference.
pub trait GetElement<E: MeshElement> {
    fn get(&self, index: &Index<E>) -> &E;
}

////////////////////////////////////////////////////////////////////////////////

pub struct ElementEnumerator<'mesh, E: 'mesh + MeshElement> {
    tag: Tag,
    iter: Enumerate<Iter<'mesh, E>>,
}

impl<'mesh, E> ElementEnumerator<'mesh, E>
where
    E: 'mesh + MeshElement,
{
    pub fn new(tag: Tag, iter: Enumerate<Iter<'mesh, E>>) -> ElementEnumerator<'mesh, E> {
        debug!("New element enumerator");
        ElementEnumerator { tag, iter }
    }

    pub fn next_element(&mut self) -> Option<(Index<E>, &'mesh E)> {
        for (offset, element) in self.iter.by_ref() {
            let props = element.props();
            let is_next =
                props.status.get() == ElementStatus::ACTIVE && props.tag.get() != self.tag;
            if is_next {
                props.tag.set(self.tag);
                let index = Index::with_generation(offset, props.generation.get());
                return Some((index, element));
            }
        }
        debug!("Element enumeration completed.");
        return None;
    }
}

pub type VertexEnumerator<'mesh> = ElementEnumerator<'mesh, Vertex>;
pub type FaceEnumerator<'mesh> = ElementEnumerator<'mesh, Face>;
pub type EdgeEnumerator<'mesh> = ElementEnumerator<'mesh, Edge>;
pub type PointEnumerator<'mesh> = ElementEnumerator<'mesh, Point>;

///////////////////////////////////////////////////////////////////////////////

/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    edge_buffer: ElementBuffer<Edge>,
    face_buffer: ElementBuffer<Face>,
    vertex_buffer: ElementBuffer<Vertex>,
    point_buffer: ElementBuffer<Point>,
}

impl Kernel {
    /// Sorts contents of each buffer moving inactive elements to the back.
    #[allow(dead_code)]
    pub fn defrag(&mut self) {
        unimplemented!()
    }

    /// Drops all inactive elements and shrinks buffers.
    #[allow(dead_code)]
    pub fn collect(&mut self) {
        unimplemented!()
    }

    pub fn edge_count(&self) -> usize {
        self.edge_buffer.len()
    }

    pub fn face_count(&self) -> usize {
        self.face_buffer.len()
    }

    pub fn vertex_count(&self) -> usize {
        self.vertex_buffer.len()
    }

    pub fn point_count(&self) -> usize {
        self.point_buffer.len()
    }

    pub fn enumerate_edges<'mesh>(&'mesh self, tag: Tag) -> EdgeEnumerator {
        EdgeEnumerator::new(tag, self.edge_buffer.enumerate())
    }

    pub fn enumerate_vertices<'mesh>(&'mesh self, tag: Tag) -> VertexEnumerator {
        VertexEnumerator::new(tag, self.vertex_buffer.enumerate())
    }

    pub fn enumerate_faces<'mesh>(&'mesh self, tag: Tag) -> FaceEnumerator {
        FaceEnumerator::new(tag, self.face_buffer.enumerate())
    }

    pub fn enumerate_points<'mesh>(&'mesh self, tag: Tag) -> PointEnumerator {
        PointEnumerator::new(tag, self.point_buffer.enumerate())
    }
}

impl AddElement<Vertex> for Kernel {
    fn add(&mut self, vertex: Vertex) -> VertexIndex {
        self.vertex_buffer.add(vertex)
    }
}

impl AddElement<Edge> for Kernel {
    fn add(&mut self, edge: Edge) -> EdgeIndex {
        self.edge_buffer.add(edge)
    }
}

impl AddElement<Face> for Kernel {
    fn add(&mut self, face: Face) -> FaceIndex {
        self.face_buffer.add(face)
    }
}

impl AddElement<Point> for Kernel {
    fn add(&mut self, point: Point) -> PointIndex {
        self.point_buffer.add(point)
    }
}

impl RemoveElement<Vertex> for Kernel {
    fn remove(&mut self, index: VertexIndex) {
        self.vertex_buffer.remove(index);
    }
}

impl RemoveElement<Edge> for Kernel {
    fn remove(&mut self, index: EdgeIndex) {
        self.edge_buffer.remove(index);
    }
}

impl RemoveElement<Face> for Kernel {
    fn remove(&mut self, index: FaceIndex) {
        self.face_buffer.remove(index);
    }
}

impl RemoveElement<Point> for Kernel {
    fn remove(&mut self, index: PointIndex) {
        self.point_buffer.remove(index);
    }
}

impl GetElement<Vertex> for Kernel {
    fn get(&self, index: &VertexIndex) -> &Vertex {
        self.vertex_buffer.get(index)
    }
}

impl GetElement<Edge> for Kernel {
    fn get(&self, index: &EdgeIndex) -> &Edge {
        self.edge_buffer.get(index)
    }
}

impl GetElement<Face> for Kernel {
    fn get(&self, index: &FaceIndex) -> &Face {
        self.face_buffer.get(index)
    }
}

impl GetElement<Point> for Kernel {
    fn get(&self, index: &PointIndex) -> &Point {
        self.point_buffer.get(index)
    }
}
