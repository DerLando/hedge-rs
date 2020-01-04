use std::fmt;

use crate::data::{Generation, Index, INVALID_INDEX, Point, Face, Vertex, Handle};
use crate::traits::{
    AddElement, GetElement, IsValid, RemoveElement, Storable
};

/// A pretty simple wrapper over a pair of 'Vec's.
pub struct CompactableBuffer<E: Storable> {
    generation: Generation,
    pub free_cells: Vec<Index>,
    pub buffer: Vec<E>,
}

impl<E: Storable> Default for CompactableBuffer<E> {
    fn default() -> Self {
        CompactableBuffer {
            generation: 1,
            free_cells: Vec::new(),
            buffer: Vec::new(),
        }
    }
}

impl<E: Storable> fmt::Debug for CompactableBuffer<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElementBuffer<> {{ {} items }}", self.len())
    }
}

impl<E: Storable> CompactableBuffer<E> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of currently active cells.
    /// The actual number of items allocated by the buffer might
    /// be different.
    #[inline]
    pub fn len(&self) -> usize {
        self.buffer.len() - self.free_cells.len()
    }

    #[inline]
    pub fn has_inactive_cells(&self) -> bool {
        !self.free_cells.is_empty()
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Index, &E)> {
        self.buffer
            .iter()
            .enumerate()
            .map(|(index, element)| (index as Index, element))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Index, &mut E)> {
        self.buffer
            .iter_mut()
            .enumerate()
            .map(|(index, element)| (index as Index, element))
    }

    pub fn active_elements(&self) -> impl Iterator<Item = &E> {
        let free_cells = self.free_cells.clone(); // TODO: to avoid an outlived reference
        self.enumerate()
            .filter(move |elem| !free_cells.contains(&elem.0))
            .map(|item| item.1)
    }

    pub fn active_elements_mut(&mut self) -> impl Iterator<Item = &mut E> {
        let free_cells = self.free_cells.clone(); // TODO: to avoid an outlived reference
        self.enumerate_mut()
            .filter(move |elem| !free_cells.contains(&elem.0))
            .map(|item| item.1)
    }

    pub fn get(&self, index: Index) -> Option<&E> {
        if index == INVALID_INDEX || self.free_cells.contains(&index) {
            return None;
        }
        self.buffer.get(index as usize)
    }

    pub fn add(&mut self, element: E) -> Index {
        if let Some(index) = self.free_cells.pop() {
            let cell = &mut self.buffer[index as usize];
            *cell = element;
            index
        } else {
            let index = self.buffer.len() as Index;
            self.buffer.push(element);
            index
        }
    }

    pub fn remove(&mut self, index: Index) {
        if self.get(index).is_some() {
            self.free_cells.push(index);
        }
    }

    /// Rebuilds internal buffer without any inactive cells.
    /// Returns a vector of previous indices for each new element.
    /// This should be used to update internal adjacency information.
    pub fn compact(&mut self) -> Vec<usize> {
        self.generation += 1;
        let elem_count = self.len();
        let mut remap_buffer = Vec::with_capacity(elem_count);
        let mut new_buffer = Vec::with_capacity(elem_count);
        for (previous_index, element) in self.buffer.drain(0..).enumerate() {
            remap_buffer.push(previous_index);
            new_buffer.push(element);
        }
        self.buffer = new_buffer;
        remap_buffer
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    pub face_buffer: CompactableBuffer<Face>,
    pub point_buffer: CompactableBuffer<Point>,
}

impl Kernel {
    pub fn can_be_compacted(&self) -> bool {
        self.face_buffer.free_cells.is_empty()
            || self.point_buffer.free_cells.is_empty()
    }

    pub fn compact(&mut self) {
        let _face_remap_table = self.face_buffer.compact();
        let _point_remap_table = self.point_buffer.compact();
        unimplemented!()
    }
}

