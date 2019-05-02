
use std::fmt;
use std::slice::Iter;
use std::iter::Enumerate;
use std::cmp::Ordering;
use log::*;

use super::{
    MeshElement, IsValid, IsActive, Storable, Index, ElementStatus, ElementData,
    Face, Edge, Vertex, Point, AddElement, RemoveElement, GetElement,
    EdgeData, FaceData, VertexData, PointData, FaceIndex, VertexIndex,
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

    pub fn has_inactive_cells(&self) -> bool {
        !self.free_cells.is_empty()
    }

    fn sort(&mut self) {
        self.buffer[1..].sort_by(|a, b| {
            use crate::ElementStatus::*;
            match (a.status(), b.status()) {
                (ACTIVE, INACTIVE) => Ordering::Less,
                (INACTIVE, ACTIVE) => Ordering::Greater,
                (_, _) => Ordering::Equal,
            }
        });
    }

    pub fn enumerate(&self) -> Enumerate<Iter<MeshElement<D>>> {
        let mut it = self.buffer.iter().enumerate();
        let _ = it.next(); // Always skip the first element since we know it's invalid
        return it;
    }

    pub fn active_cells(
        &self
    ) -> impl Iterator<Item=(usize, &MeshElement<D>)> {
        self.buffer.iter().enumerate()
            .filter(|elem| elem.1.is_active())
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

    fn truncate_inactive(&mut self) {
        let total = self.buffer.len();
        let inactive = self.free_cells.len();
        let active = total - inactive;
        self.free_cells.clear();
        self.buffer.truncate(active);
    }

    fn next_swap_pair(&self) -> Option<(usize, usize)> {
        let inactive_offset = self.enumerate()
            .find(|e| !e.1.is_active())
            .map(|e| e.0);
        let active_offset = self.enumerate()
            .rev().find(|e| e.1.is_active())
            .map(|e| e.0);
        if active_offset.is_none() || inactive_offset.is_none() {
            // If we can't find both an active and inactive cell
            // offset then we have nothing to do.
            None
        } else {
            let inactive_offset = inactive_offset.unwrap();
            let active_offset = active_offset.unwrap();
            if active_offset > inactive_offset {
                // by the time this is true we should have sorted/swapped
                // all elements so that the inactive inactive elements
                // make up the tail of the buffer.
                None
            } else {
                Some((inactive_offset, active_offset))
            }
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
    fn defrag_faces(&mut self) {
        if self.face_buffer.has_inactive_cells() {
            self.face_buffer.sort();
            self.face_buffer
                .active_cells()
                .map(|(offset, face)| {
                    (FaceIndex::with_generation(offset, face.generation.get()), face)
                })
                .filter(|(index, face)| {
                    let root_edge_index = face.data.borrow().edge_index;
                    if let Some(root_edge) = self.edge_buffer.get(&root_edge_index) {
                        let root_face_index = root_edge.data.borrow().face_index;
                        *index != root_face_index
                    } else {
                        warn!("The root edge of the face at {:?} points to invalid edge.",
                              root_edge_index);
                        false
                    }
                })
                .for_each(|(face_index, face)| {
                    let root_edge_index = face.data.borrow().edge_index;
                    let mut edge_index = root_edge_index;
                    loop {
                        let edge = &self.edge_buffer.buffer[edge_index.offset];

                        let mut data = edge.data.borrow_mut();
                        // prevent an infinite loop for broken meshes
                        if data.face_index == face_index {
                            break;
                        }
                        data.face_index = face_index;

                        edge_index = data.next_index;
                        if edge_index == root_edge_index {
                            break;
                        }
                    }
                });
            self.face_buffer.truncate_inactive();
        }
    }

    fn defrag_verts(&mut self) {
        if self.vertex_buffer.has_inactive_cells() {
            self.vertex_buffer.sort();
            self.vertex_buffer
                .active_cells()
                .map(|(offset, vertex)| {
                    (VertexIndex::with_generation(offset, vertex.generation.get()), vertex)
                })
                .filter(|(vert_index, vertex)| {
                    let vert_edge_index = vertex.data.borrow().edge_index;
                    if let Some(edge) = self.edge_buffer.get(&vert_edge_index) {
                        *vert_index != edge.data.borrow().vertex_index
                    } else {
                        warn!("Vertex at {:?} has an invalid edge index.", vert_index);
                        false
                    }
                })
                .for_each(|(vertex_index, vertex)| {
                    let e0 = {
                        let eindex = vertex.data.borrow().edge_index;
                        &self.edge_buffer.buffer[eindex.offset]
                    };
                    e0.data.borrow_mut().vertex_index = vertex_index;
                });
            self.vertex_buffer.truncate_inactive();
        }
    }

    fn defrag_edges(&mut self) {
        if self.edge_buffer.has_inactive_cells() {
            // The edge array can't be sorted as easily
            // as faces and vertices because an edge
            // refers to other elements in the same buffer.
            // Our aproach here needs to be incremental and
            // swap the first active cell from the end of the
            // buffer with first inactive cell from the front
            // of the buffer.
            loop {
                if let Some(offsets) = {self.edge_buffer.next_swap_pair()} {
                    let inactive_offset = offsets.0;
                    let active_offset = offsets.1;

                    self.edge_buffer.buffer.swap(inactive_offset, active_offset);
                    let swapped = &self.edge_buffer.buffer[inactive_offset];
                    let swapped_data = swapped.data();
                    let swapped_index = Index::with_generation(inactive_offset, swapped.generation.get());

                    if let Some(next_edge) = self.edge_buffer.get(&swapped_data.next_index) {
                        next_edge.data_mut().prev_index = swapped_index;
                    }
                    if let Some(prev_edge) = self.edge_buffer.get(&swapped_data.prev_index) {
                        prev_edge.data_mut().next_index = swapped_index;
                    }
                    if let Some(twin_edge) = self.edge_buffer.get(&swapped_data.twin_index) {
                        twin_edge.data_mut().twin_index = swapped_index;
                    }

                    // For faces and vertices we only want to update the
                    // associated edge index when it matched the original
                    // buffer location.
                    // I'm doing this in case the associated root edge
                    // index for these elements is meaningful or important.

                    if let Some(face) = self.face_buffer.get(&swapped_data.face_index) {
                        let mut face_data = face.data_mut();
                        if face_data.edge_index.offset == active_offset {
                            face_data.edge_index = swapped_index;
                        }
                    }
                    if let Some(vertex) = self.vertex_buffer.get(&swapped_data.vertex_index) {
                        let mut vertex_data = vertex.data_mut();
                        if vertex_data.edge_index.offset == active_offset {
                            vertex_data.edge_index = swapped_index;
                        }
                    }
                } else {
                    break;
                }
            }
            self.edge_buffer.truncate_inactive();
        }
    }

    fn defrag_points(&mut self) {
        if self.point_buffer.has_inactive_cells() {
            // The point structure is potentially
            // referenced from multiple vertices and
            // points do not hold any reference to
            // the vertices associated with them.
            // Because of this we have to search for
            // vertices with a reference to the point
            // at its original location.
            // This also means we can't use the more
            // convienient sort approach.
            loop {
                if let Some(offsets) = self.point_buffer.next_swap_pair() {
                    let inactive_offset = offsets.0;
                    let active_offset = offsets.1;

                    self.point_buffer.buffer.swap(inactive_offset, active_offset);
                    let swapped = &self.point_buffer.buffer[inactive_offset];
                    let swapped_index = Index::with_generation(inactive_offset, swapped.generation.get());

                    self.vertex_buffer.buffer[1..].iter()
                        .filter(|v| v.is_active() && v.data().point_index.offset == active_offset)
                        .for_each(|v| {
                            v.data_mut().point_index = swapped_index;
                        });
                } else {
                    break;
                }
            }
            self.vertex_buffer.truncate_inactive();
        }
    }

    /// Sorts buffers and drops all inactive elements.
    pub fn collect(&mut self) {
        if self.inactive_element_count() > 0 {
            self.defrag_faces();
            self.defrag_verts();
            self.defrag_points();
            self.defrag_edges();
        }
    }

    pub fn inactive_element_count(&self) -> usize {
        self.face_buffer.free_cells.len() +
            self.edge_buffer.free_cells.len() +
            self.vertex_buffer.free_cells.len() +
            self.point_buffer.free_cells.len()
    }

    pub fn active_element_count(&self) -> usize {
        self.face_buffer.len() +
            self.edge_buffer.len() +
            self.vertex_buffer.len() +
            self.point_buffer.len()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EdgeIndex;

    fn new_edge(kernel: &mut Kernel) -> EdgeIndex {
        let e0 = kernel.add_element(Edge::default());
        let e1 = kernel.add_element(Edge::default());
        match (kernel.get_element(&e0),
               kernel.get_element(&e1)) {
            (Some(edge0), Some(edge1)) => {
                edge0.data.borrow_mut().twin_index = e1;
                edge1.data.borrow_mut().twin_index = e0;
            },
            _ => panic!("Invalid edge indexes specified: {:?}, {:?}", e0, e1),
        }
        e0
    }

    fn connect_edges(
        kernel: &mut Kernel,
        prev_index: EdgeIndex,
        next_index: EdgeIndex
    ) {
        match (kernel.get_element(&prev_index),
               kernel.get_element(&next_index)) {
            (Some(prev), Some(next)) => {
                prev.data.borrow_mut().next_index = next_index;
                next.data.borrow_mut().prev_index = prev_index;
            },
            _ => panic!("Invalid edge indexes specified: {:?}, {:?}",
                        prev_index, next_index),
        }
    }

    fn make_face(kernel: &mut Kernel, root_edge: EdgeIndex) -> FaceIndex {
        let face_index = kernel.add_element(
            Face::with_data(FaceData {
                edge_index: root_edge
            })
        );
        let mut edge_index = root_edge;
        loop {
            let edge = &kernel.edge_buffer.buffer[edge_index.offset];
            let mut data = edge.data.borrow_mut();
            if data.face_index == face_index {
                break;
            }
            data.face_index = face_index;
            if data.next_index == root_edge {
                break;
            }
            edge_index = data.next_index;
        }
        face_index
    }

    fn make_triangle(kernel: &mut Kernel) -> FaceIndex {
        let e0 = new_edge(kernel);
        let e1 = new_edge(kernel);
        let e2 = new_edge(kernel);

        connect_edges(kernel, e0, e1);
        connect_edges(kernel, e1, e2);
        connect_edges(kernel, e2, e0);

        make_face(kernel, e0)
    }

    #[test]
    fn defrag_faces() {
        let _ = env_logger::try_init();
        let mut kernel = Kernel::default();

        let f0 = make_triangle(&mut kernel);
        let root_edge = kernel.face_buffer.buffer[f0.offset].data.borrow().edge_index;

        let f1 = make_face(&mut kernel, root_edge);
        let f2 = make_face(&mut kernel, root_edge);
        assert_eq!(kernel.face_buffer.len(), 4);
        assert_eq!(f2.offset, 3);
        assert_eq!(f2.generation, 1);

        kernel.remove_element(f0);
        kernel.remove_element(f1);

        assert!(kernel.face_buffer.has_inactive_cells());
        assert_eq!(kernel.face_buffer.len(), 2);
        assert_eq!(kernel.face_buffer.free_cells.len(), 2);

        let root_face_index = kernel.edge_buffer
            .buffer[root_edge.offset]
            .data.borrow().face_index;
        assert_eq!(root_face_index, f2);

        kernel.defrag_faces();
        assert_eq!(kernel.face_buffer.len(), 2);
        assert_eq!(kernel.face_buffer.free_cells.len(), 0);
        assert!(!kernel.face_buffer.has_inactive_cells());
        assert!(kernel.get_element(&f2).is_none());

        let root_face_index = kernel.edge_buffer
            .buffer[root_edge.offset]
            .data.borrow().face_index;
        assert_ne!(root_face_index, f2);
        assert!(kernel.get_element(&root_face_index).is_some());
        let face_edge_index = kernel.face_buffer
            .buffer[root_face_index.offset]
            .data.borrow().edge_index;
        assert_eq!(face_edge_index, root_edge);
    }
}
