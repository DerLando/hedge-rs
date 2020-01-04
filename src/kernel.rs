use std::fmt;

use crate::data::{Generation, Index, IGNORED_GENERATION};
use crate::elements::{ElementStatus, Face, Vertex};
use crate::handles::Handle;
use crate::traits::{
    AddElement, GetElement, IsValid, RemoveElement,
};

/// A pretty simple wrapper over a pair of 'Vec's.
pub struct ElementBuffer {
    generation: Generation,
    pub free_cells: Vec<Index>,
    pub buffer: Vec<Face>,
}

impl Default for ElementBuffer {
    fn default() -> Self {
        ElementBuffer {
            generation: 1,
            free_cells: Vec::new(),
            buffer: Vec::new(),
        }
    }
}

impl fmt::Debug for ElementBuffer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElementBuffer<> {{ {} items }}", self.len())
    }
}

impl ElementBuffer {
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

    pub fn enumerate(&self) -> impl Iterator<Item = (Handle, &Face)> {
        self.buffer
            .iter()
            .enumerate()
            .map(|(index, element)| (Handle::new(index as Index, self.generation), element))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Handle, &mut Face)> {
        self.buffer
            .iter_mut()
            .enumerate()
            .map(|(index, element)| (Handle::new(index as Index, self.generation), element))
    }

    pub fn active_cells(&self) -> impl Iterator<Item = (Handle, &Face)> {
        self.enumerate().filter(|elem| elem.1.is_active())
    }

    pub fn active_cells_mut(&mut self) -> impl Iterator<Item = (Handle, &mut Face)> {
        self.enumerate_mut().filter(|elem| elem.1.is_active())
    }

    pub fn active_elements(&self) -> impl Iterator<Item = &Face> {
        self.buffer.iter().filter(|elem| elem.is_active())
    }

    pub fn active_elements_mut(&mut self) -> impl Iterator<Item = &mut Face> {
        self.buffer.iter_mut().filter(|elem| elem.is_active())
    }

    fn ensure_active_cell(element: &Face) -> Option<&Face> {
        if element.is_active() {
            Some(element)
        } else {
            None
        }
    }

    pub fn get(&self, handle: Handle) -> Option<&Face> {
        if !handle.is_valid() {
            return None;
        }

        self.buffer
            .get(handle.index() as usize)
            .and_then(Self::ensure_active_cell)
            .and_then(|elem| {
                if handle.generation() > IGNORED_GENERATION {
                    if self.generation == handle.generation() {
                        Some(elem)
                    } else {
                        None
                    }
                } else {
                    Some(elem)
                }
            })
    }

    pub fn add(&mut self, element: Face) -> Handle {
        if let Some(index) = self.free_cells.pop() {
            let cell = &mut self.buffer[index as usize];
            *cell = element;
            cell.set_status(ElementStatus::ACTIVE);
            Handle::new(index, self.generation)
        } else {
            let handle = Handle::new(self.buffer.len() as u32, self.generation);
            self.buffer.push(element);
            if let Some(element) = self.buffer.get_mut(handle.index() as usize) {
                element.set_status(ElementStatus::ACTIVE);
            }
            handle
        }
    }

    pub fn remove(&mut self, handle: Handle) {
        if let Some(cell) = self.get(handle) {
            cell.set_status(ElementStatus::INACTIVE);
            self.free_cells.push(handle.index());
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
    pub buffer: ElementBuffer,
}

impl Kernel {
    pub fn can_be_compacted(&self) -> bool {
        self.face_buffer.free_cells.is_empty()
            || self.edge_buffer.free_cells.is_empty()
            || self.vertex_buffer.free_cells.is_empty()
    }
}

