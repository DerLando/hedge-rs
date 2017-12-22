
use std::fmt;
use std::marker;
use std::cmp::Ordering;
use std::cell::Cell;
use std::slice::Iter;
use std::iter::Enumerate;
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
#[derive(Default, Debug, Clone)]
pub struct Index<T> {
    pub offset: Offset,
    pub generation: Generation,
    _marker: marker::PhantomData<T>,
}

impl <T: Clone> Copy for Index<T> {}

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
}

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE
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
    pub free_cells: Vec< Index<T> >,
    pub buffer: Vec<T>,
}

impl <T: MeshElement + Default> Default for ElementBuffer<T> {
    fn default() -> ElementBuffer<T> {
        ElementBuffer {
            free_cells: Vec::new(),
            buffer: vec![ T::default() ],
        }
    }
}

impl <T: MeshElement + Default> fmt::Debug for ElementBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ComponentBuffer<> {{ {} items }}", self.len())
    }
}

impl <T: MeshElement + Default> ElementBuffer<T> {

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
            if index.generation == element.props().generation.get() &&
                element.props().status.get() == ElementStatus::ACTIVE {
                result = element;
            }
        }
        return result;
    }

    pub fn get_mut(&mut self, index: &Index<T>) -> Option<&mut T> {
        let element = &mut self.buffer[index.offset];
        if element.props().generation.get() == index.generation &&
            element.props().status.get() == ElementStatus::ACTIVE {
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

/// Interface for getting a mutable element reference.
pub trait GetElementMut<E: MeshElement> {
    fn get_mut(&mut self, index: &Index<E>) -> Option<&mut E>;
}
