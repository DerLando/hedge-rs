
use std::fmt;
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

pub type Tag = usize;
pub type Offset = usize;
pub type Generation = usize;

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

    fn get_edge_mut(&mut self, index: EdgeIndex) -> Option<&mut Edge>;
    fn get_face_mut(&mut self, index: FaceIndex) -> Option<&mut Face>;
    fn get_vertex_mut(&mut self, index: VertexIndex) -> Option<&mut Vertex>;
    fn get_point_mut(&mut self, index: PointIndex) -> Option<&mut Point>;

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

/// Marker trait for operators using Index types
pub trait ElementIndex {}

/// Marker trait for operators using Mesh components
/// Components are expected to have a field `component: ComponentProperties`
pub trait MeshElement: Default {
    fn props(&self) -> &ElementProperties;
    fn props_mut(&mut self) -> &mut ElementProperties;
}

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE
}

impl Default for ElementStatus {
    fn default() -> Self {
        ElementStatus::ACTIVE
    }
}

/// The 3 fields our component buffers needs to do its work
#[derive(Debug, Default, Copy, Clone)]
pub struct ElementProperties {
    pub status: ElementStatus,
    pub generation: Generation,
}

///
/// Blah blah blah
///
#[derive(Default)]
pub struct ElementBuffer<T: MeshElement + Default> {
    free_cells: Vec< Index<T> >,
    buffer: Vec<T>,
}

impl <T: MeshElement + Default> fmt::Debug for ElementBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComponentBuffer<> {{ {} items }}", self.len())
    }
}

impl <T: MeshElement + Default> ElementBuffer<T> {
    pub fn new() -> ElementBuffer<T> {
        ElementBuffer {
            free_cells: Vec::new(),
            buffer: vec![ T::default() ]
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len() - self.free_cells.len()
    }

    pub fn get(&self, index: &Index<T>) -> &T {
        let mut result = &self.buffer[0];
        if let Some(element) = self.buffer.get(index.offset) {
            if index.generation == element.props().generation &&
                element.props().status == ElementStatus::ACTIVE {
                result = element;
            }
        }
        return result;
    }

    pub fn get_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        let element = &mut self.buffer[index.offset];
        if element.props().generation == index.generation &&
            element.props().status == ElementStatus::ACTIVE {
            Some(element)
        } else {
            None
        }
    }

    pub fn add(&mut self, element: T) -> Index<T> {
        if let Some(index) = self.free_cells.pop() {
            let cell = &mut self.buffer[index.offset];
            *cell = element;
            cell.props_mut().generation = index.generation;
            return index;
        } else {
            let index = Index::with_generation(self.buffer.len(), element.props().generation);
            self.buffer.push(element);
            return index;
        }
    }

    pub fn remove(&mut self, index: Index<T>) {
        let removed = self.get_mut(&index).map(|cell| {
            let props = cell.props_mut();
            props.generation += 1;
            props.status = ElementStatus::INACTIVE;
            Index::with_generation(index.offset, index.generation + 1)
        });
        if let Some(removed) = removed {
            self.free_cells.push(removed);
        }
    }
}