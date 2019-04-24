
use std::fmt;
use std::slice::Iter;
use std::iter::Enumerate;

use super::{
    MeshElement, IsValid, IsActive, Storable, Index, ElementStatus, ElementData,
    Face, Edge, Vertex, Point, AddElement, RemoveElement, GetElement,
    EdgeData, FaceData, VertexData, PointData
};

/// A pretty simple wrapper over a pair of 'Vec's.
pub struct ElementBuffer<D: ElementData + Default> {
    pub free_cells: Vec<Index<MeshElement<D>>>,
    pub buffer: Vec<MeshElement<D>>,
}

impl<D: ElementData + Default> Default for ElementBuffer<D> {
    fn default() -> Self {
        ElementBuffer {
            free_cells: Vec::new(),
            buffer: vec![Default::default()],
        }
    }
}

impl<D: ElementData + Default> fmt::Debug for ElementBuffer<D> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElementBuffer<> {{ {} items }}", self.len())
    }
}

impl<D: ElementData + Default> ElementBuffer<D> {
    /// Returns the number of currently active cells.
    /// The actual number of items allocated by the buffer might
    /// be different.
    pub fn len(&self) -> usize {
        self.buffer.len() - self.free_cells.len()
    }

    pub fn enumerate(&self) -> Enumerate<Iter<MeshElement<D>>> {
        let mut it = self.buffer.iter().enumerate();
        let _ = it.next(); // Always skip the first element since we know it's invalid
        return it;
    }

    fn ensure_active_cell(element: &MeshElement<D>) -> Option<&MeshElement<D>> {
        if element.is_active() {
            Some(element)
        } else {
            None
        }
    }

    fn ensure_matching_generation<'mesh>(
        element: &'mesh MeshElement<D>,
        index: &Index<MeshElement<D>>
    ) -> Option<&'mesh MeshElement<D>> {
        if index.generation > 0 {
            if element.generation() == index.generation {
                Some(element)
            } else {
                None
            }
        } else {
            Some(element)
        }
    }

    pub fn get(
        &self,
        index: &Index<MeshElement<D>>
    ) -> Option<&MeshElement<D>> {
        if index.is_valid() {
            self.buffer.get(index.offset)
                .and_then(ElementBuffer::ensure_active_cell)
                .and_then(|e| ElementBuffer::ensure_matching_generation(e, index))
        } else {
            None
        }
    }

    pub fn add(&mut self, element: MeshElement<D>) -> Index<MeshElement<D>> {
        if let Some(index) = self.free_cells.pop() {
            let cell = &mut self.buffer[index.offset];
            *cell = element;
            cell.status.set(ElementStatus::ACTIVE);
            cell.generation.set(index.generation);
            return index;
        } else {
            let index = Index::with_generation(self.buffer.len(), element.generation.get());
            self.buffer.push(element);
            if let Some(element) = self.buffer.get_mut(index.offset) {
                element.status.set(ElementStatus::ACTIVE);
            }
            return index;
        }
    }

    pub fn remove(&mut self, index: Index<MeshElement<D>>) {
        if let Some(cell) = self.get(&index) {
            let removed_index ={
                let prev_gen = cell.generation.get();
                cell.generation.set(prev_gen + 1);
                cell.status.set(ElementStatus::INACTIVE);
                Index::with_generation(index.offset, cell.generation.get())
            };
            self.free_cells.push(removed_index);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    pub edge_buffer: ElementBuffer<EdgeData>,
    pub face_buffer: ElementBuffer<FaceData>,
    pub vertex_buffer: ElementBuffer<VertexData>,
    pub point_buffer: ElementBuffer<PointData>,
}

impl Kernel {
    /// Sorts contents of each buffer moving inactive elements to the back.
    pub fn defrag(&mut self) {
        unimplemented!()
    }

    /// Drops all inactive elements and shrinks buffers.
    pub fn collect(&mut self) {
        unimplemented!()
    }
}

impl GetElement<Point> for Kernel {
    fn get_element(&self, index: &Index<Point>) -> Option<&Point> {
        self.point_buffer.get(index)
    }
}

impl GetElement<Vertex> for Kernel {
    fn get_element(&self, index: &Index<Vertex>) -> Option<&Vertex> {
        self.vertex_buffer.get(index)
    }
}

impl GetElement<Edge> for Kernel {
    fn get_element(&self, index: &Index<Edge>) -> Option<&Edge> {
        self.edge_buffer.get(index)
    }
}

impl GetElement<Face> for Kernel {
    fn get_element(&self, index: &Index<Face>) -> Option<&Face> {
        self.face_buffer.get(index)
    }
}

impl AddElement<Point> for Kernel {
    fn add_element(&mut self, element: Point) -> Index<Point> {
        self.point_buffer.add(element)
    }
}

impl AddElement<Vertex> for Kernel {
    fn add_element(&mut self, element: Vertex) -> Index<Vertex> {
        self.vertex_buffer.add(element)
    }
}

impl AddElement<Edge> for Kernel {
    fn add_element(&mut self, element: Edge) -> Index<Edge> {
        self.edge_buffer.add(element)
    }
}

impl AddElement<Face> for Kernel {
    fn add_element(&mut self, element: Face) -> Index<Face> {
        self.face_buffer.add(element)
    }
}

impl RemoveElement<Point> for Kernel {
    fn remove_element(&mut self, index: Index<Point>) {
        self.point_buffer.remove(index)
    }
}

impl RemoveElement<Vertex> for Kernel {
    fn remove_element(&mut self, index: Index<Vertex>) {
        self.vertex_buffer.remove(index)
    }
}

impl RemoveElement<Edge> for Kernel {
    fn remove_element(&mut self, index: Index<Edge>) {
        self.edge_buffer.remove(index)
    }
}

impl RemoveElement<Face> for Kernel {
    fn remove_element(&mut self, index: Index<Face>) {
        self.face_buffer.remove(index)
    }
}
