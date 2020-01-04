use std::fmt;

use crate::data::{
    Generation, Index, Point, Face, Vertex, Handle,
    INVALID_INDEX, ComponentID,
};
use crate::traits::{
    IsValid, Storable
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

    pub fn get_mut(&mut self, index: Index) -> Option<&mut E> {
        if index == INVALID_INDEX || self.free_cells.contains(&index) {
            return None;
        }
        self.buffer.get_mut(index as usize)
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

    pub fn add_point(&mut self, point: Point) -> Handle {
        log::trace!("Adding point {:?}", point);
        let index = self.point_buffer.add(point);
        Point::make_handle(index, self.point_buffer.generation)
    }

    pub fn add_face(&mut self, face: Face) -> Handle {
        log::trace!("Adding face {:?}", face);
        let index = self.face_buffer.add(face);
        Face::make_handle(index, self.face_buffer.generation)
    }

    pub fn point(&self, handle: impl Into<Handle>) -> Option<&Point> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Point(index) = handle.id() {
            self.point_buffer.get(index)
        } else {
            log::warn!("Component ID mismatch. Expected point, got {:?}", handle);
            None
        }
    }

    pub fn point_mut(&mut self, handle: impl Into<Handle>) -> Option<&mut Point> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Point(index) = handle.id() {
            self.point_buffer.get_mut(index)
        } else {
            log::warn!("Component ID mismatch. Expected point, got {:?}", handle);
            None
        }
    }

    pub fn face(&self, handle: impl Into<Handle>) -> Option<&Face> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Face(index) = handle.id() {
            self.face_buffer.get(index)
        } else {
            log::warn!("Component ID mismatch. Expected face, got {:?}", handle);
            None
        }
    }

    pub fn face_mut(&mut self, handle: impl Into<Handle>) -> Option<&mut Face> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Face(index) = handle.id() {
            self.face_buffer.get_mut(index)
        } else {
            log::warn!("Component ID mismatch. Expected face, got {:?}", handle);
            None
        }
    }

    pub fn vertex(&self, handle: impl Into<Handle>) -> Option<&Vertex> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Vertex(vert_id) = handle.id() {
            self.face_buffer.get(vert_id.fidx).map(|face| face.vertex(vert_id.vidx))
        } else {
            log::warn!("Component ID mismatch. Expected vertex, got {:?}", handle);
            None
        }
    }

    pub fn vertex_mut(&mut self, handle: impl Into<Handle>) -> Option<&mut Vertex> {
        let handle = handle.into();
        if !handle.is_valid() {
            return None;
        }
        if let ComponentID::Vertex(vert_id) = handle.id() {
            self.face_buffer.get_mut(vert_id.fidx).map(|face| face.vertex_mut(vert_id.vidx))
        } else {
            log::warn!("Component ID mismatch. Expected vertex, got {:?}", handle);
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_kernel_operation() {
        let mut kernel = Kernel::default();

        let p0 = kernel.add_point(Point::default());
        let p1 = kernel.add_point(Point::default());
        let p2 = kernel.add_point(Point::default());

        let f0 = {
            let face = Face::from_points([
                p0.index(),
                p1.index(),
                p2.index()
            ].as_ref());
            kernel.add_face(face)
        };

        assert_eq!(kernel.face_buffer.len(), 1);
        assert_eq!(kernel.point_buffer.len(), 3);

        let face = kernel.face(f0);
        assert!(face.is_some());
        let face = face.unwrap();
        assert_eq!(face.vert_count(), 3);

        assert_eq!(face.vertex(0).point, p0.index());
        assert_eq!(face.vertex(1).point, p1.index());
        assert_eq!(face.vertex(2).point, p2.index());

        let f0v0 = kernel.vertex((0,0)).expect("Failed to get f0v0!");
        let f0v1 = kernel.vertex((0,1)).expect("Failed to get f0v1!");
        let f0v2 = kernel.vertex((0,2)).expect("Failed to get f0v2!");

        assert_eq!(face.vertex(0).point, f0v0.point);
        assert_eq!(face.vertex(1).point, f0v1.point);
        assert_eq!(face.vertex(2).point, f0v2.point);
    }
}
