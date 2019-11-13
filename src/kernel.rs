use log::*;
use std::cmp::Ordering;
use std::fmt;
use std::iter::Enumerate;
use std::slice::Iter;

use crate::data::{ElementStatus, Index};
use crate::elements::{Face, HalfEdge, Point, Vertex};
use crate::handles::{FaceHandle, HalfEdgeHandle, PointHandle, VertexHandle};
use crate::traits::{
    AddElement, Element, ElementHandle, GetElement, IsValid, RemoveElement, Storable,
};

/// A pretty simple wrapper over a pair of 'Vec's.
pub struct ElementBuffer<E: Element> {
    pub free_cells: Vec<E::Handle>,
    pub buffer: Vec<E>,
}

impl<E: Element> Default for ElementBuffer<E> {
    fn default() -> Self {
        ElementBuffer {
            free_cells: Vec::new(),
            buffer: vec![Default::default()],
        }
    }
}

impl<E: Element> fmt::Debug for ElementBuffer<E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ElementBuffer<> {{ {} items }}", self.len())
    }
}

impl<E: Element> ElementBuffer<E> {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

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

    pub fn enumerate(&self) -> Enumerate<Iter<E>> {
        let mut it = self.buffer.iter().enumerate();
        let _ = it.next(); // Always skip the first element since we know it's invalid
        it
    }

    pub fn active_cells(&self) -> impl Iterator<Item = (usize, &E)> {
        self.buffer
            .iter()
            .enumerate()
            .filter(|elem| elem.1.is_active())
    }

    pub fn active_elements(&self) -> impl Iterator<Item = &E> {
        self.buffer.iter().filter(|elem| elem.is_active())
    }

    fn ensure_active_cell(element: &E) -> Option<&E> {
        if element.is_active() {
            Some(element)
        } else {
            None
        }
    }

    fn ensure_matching_generation(element: &E, handle: E::Handle) -> Option<&E> {
        if handle.generation() > 0 {
            if element.generation() == handle.generation() {
                Some(element)
            } else {
                None
            }
        } else {
            Some(element)
        }
    }

    pub fn get(&self, handle: E::Handle) -> Option<&E> {
        if handle.is_valid() {
            self.buffer
                .get(handle.index() as usize)
                .and_then(ElementBuffer::ensure_active_cell)
                .and_then(|e| ElementBuffer::ensure_matching_generation(e, handle))
        } else {
            None
        }
    }

    pub fn add(&mut self, element: E) -> E::Handle {
        if let Some(handle) = self.free_cells.pop() {
            let cell = &mut self.buffer[handle.index() as usize];
            *cell = element;
            cell.set_status(ElementStatus::ACTIVE);
            cell.set_generation(handle.generation());
            handle
        } else {
            let handle = E::Handle::with_generation(self.buffer.len() as u32, element.generation());
            self.buffer.push(element);
            if let Some(element) = self.buffer.get_mut(handle.index() as usize) {
                element.set_status(ElementStatus::ACTIVE);
            }
            handle
        }
    }

    pub fn remove(&mut self, handle: E::Handle) {
        if let Some(cell) = self.get(handle) {
            let removed_handle = {
                let next_gen = cell.generation() + 1;
                if next_gen == u32::max_value() {
                    cell.set_generation(1);
                } else {
                    cell.set_generation(next_gen);
                }
                cell.set_status(ElementStatus::INACTIVE);
                E::Handle::with_generation(handle.index(), cell.generation())
            };
            self.free_cells.push(removed_handle);
        }
    }

    // TODO: ... um if you call this on an unsorted buffer ..
    fn truncate_inactive(&mut self) {
        let total = self.buffer.len();
        let inactive = self.free_cells.len();
        let active = total - inactive;
        self.free_cells.clear();
        self.buffer.truncate(active);
    }

    fn next_swap_pair(&self) -> Option<(Index, Index)> {
        let inactive_offset = self.enumerate().find(|e| !e.1.is_active()).map(|e| e.0);
        let active_offset = self
            .enumerate()
            .rev()
            .find(|e| e.1.is_active())
            .map(|e| e.0);
        if let (Some(active_offset), Some(inactive_offset)) = (active_offset, inactive_offset) {
            if active_offset < inactive_offset {
                debug!("Buffer appears to be successfully sorted!");
                // by the time this is true we should have sorted/swapped
                // all elements so that the inactive inactive elements
                // make up the tail of the buffer.
                None
            } else {
                Some((inactive_offset as u32, active_offset as u32))
            }
        } else {
            // If we can't find both an active and inactive cell
            // offset then we have nothing to do.
            debug!("No more swap pairs!");
            None
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Storage interface for Mesh types
#[derive(Debug, Default)]
pub struct Kernel {
    pub edge_buffer: ElementBuffer<HalfEdge>,
    pub face_buffer: ElementBuffer<Face>,
    pub vertex_buffer: ElementBuffer<Vertex>,
    pub point_buffer: ElementBuffer<Point>,
}

impl Kernel {
    pub fn new_edge(&mut self) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let e0 = self.add(HalfEdge::default());
        let e1 = self.add(HalfEdge::default());
        match (self.get(e0), self.get(e1)) {
            (Some(edge0), Some(edge1)) => {
                edge0.data_mut().adjacent = e1;
                edge1.data_mut().adjacent = e0;
            }
            _ => panic!("Invalid edge handles specified: {:?}, {:?}", e0, e1),
        }
        (e0, e1)
    }

    fn defrag_faces(&mut self) {
        if self.face_buffer.has_inactive_cells() {
            self.face_buffer.sort();
            self.face_buffer
                .active_cells()
                .map(|(offset, face)| {
                    (
                        FaceHandle::with_generation(offset as u32, face.generation()),
                        face,
                    )
                })
                .filter(|(hnd, face)| {
                    let root_edge_handle = face.data().root_edge;
                    if let Some(root_edge) = self.edge_buffer.get(root_edge_handle) {
                        let root_face_handle = root_edge.data().face;
                        *hnd != root_face_handle
                    } else {
                        warn!(
                            "The root edge of the face at {:?} points to invalid edge.",
                            root_edge_handle
                        );
                        false
                    }
                })
                .for_each(|(face_handle, face)| {
                    let root_edge_handle = face.data().root_edge;
                    let mut edge_handle = root_edge_handle;
                    loop {
                        let edge = &self.edge_buffer.buffer[edge_handle.index() as usize];

                        let mut data = edge.data_mut();
                        // prevent an infinite loop for broken meshes
                        if data.face == face_handle {
                            break;
                        }
                        data.face = face_handle;

                        edge_handle = data.next;
                        if edge_handle == root_edge_handle {
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
                    (
                        VertexHandle::with_generation(offset as u32, vertex.generation()),
                        vertex,
                    )
                })
                .filter(|(vert_handle, vertex)| {
                    let vert_edge_handle = vertex.data().edge;
                    if let Some(edge) = self.edge_buffer.get(vert_edge_handle) {
                        *vert_handle != edge.data().vertex
                    } else {
                        warn!("Vertex at {:?} has an invalid edge index.", vert_handle);
                        false
                    }
                })
                .for_each(|(vertex_handle, vertex)| {
                    let e0 = {
                        let edge_handle = vertex.data().edge;
                        &self.edge_buffer.buffer[edge_handle.index() as usize]
                    };
                    e0.data_mut().vertex = vertex_handle;
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
            while let Some(indices) = self.edge_buffer.next_swap_pair() {
                let inactive_handle = indices.0;
                let active_handle = indices.1;

                self.edge_buffer
                    .buffer
                    .swap(inactive_handle as usize, active_handle as usize);
                let swapped = &self.edge_buffer.buffer[inactive_handle as usize];
                let swapped_data = swapped.data();
                let swapped_handle = <HalfEdge as Element>::Handle::with_generation(
                    inactive_handle as u32,
                    swapped.generation(),
                );

                if let Some(next_edge) = self.edge_buffer.get(swapped_data.next) {
                    next_edge.data_mut().prev = swapped_handle;
                }
                if let Some(prev_edge) = self.edge_buffer.get(swapped_data.prev) {
                    prev_edge.data_mut().next = swapped_handle;
                }
                if let Some(twin_edge) = self.edge_buffer.get(swapped_data.adjacent) {
                    twin_edge.data_mut().adjacent = swapped_handle;
                }

                // For faces and vertices we only want to update the
                // associated edge handle when it matched the original
                // buffer location.
                // I'm doing this in case the associated root edge
                // handle for these elements is meaningful or important.

                if let Some(face) = self.face_buffer.get(swapped_data.face) {
                    let mut face_data = face.data_mut();
                    if face_data.root_edge.index() == active_handle {
                        face_data.root_edge = swapped_handle;
                    }
                }
                if let Some(vertex) = self.vertex_buffer.get(swapped_data.vertex) {
                    let mut vertex_data = vertex.data_mut();
                    if vertex_data.edge.index() == active_handle {
                        vertex_data.edge = swapped_handle;
                    }
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
            while let Some(offsets) = self.point_buffer.next_swap_pair() {
                let inactive_offset = offsets.0;
                let active_offset = offsets.1;

                self.point_buffer
                    .buffer
                    .swap(inactive_offset as usize, active_offset as usize);
                let swapped = &self.point_buffer.buffer[inactive_offset as usize];
                let swapped_handle = <Point as Element>::Handle::with_generation(
                    inactive_offset as u32,
                    swapped.generation(),
                );

                self.vertex_buffer.buffer[1..]
                    .iter()
                    .filter(|v| v.is_active() && v.data().point.index() == active_offset)
                    .for_each(|v| {
                        v.data_mut().point = swapped_handle;
                    });
            }
            self.vertex_buffer.truncate_inactive();
        }
    }

    /// Sorts buffers and drops all inactive elements.
    pub fn defrag(&mut self) {
        if self.inactive_element_count() > 0 {
            self.defrag_faces();
            self.defrag_verts();
            self.defrag_points();
            self.defrag_edges();
        }
    }

    pub fn inactive_element_count(&self) -> usize {
        self.face_buffer.free_cells.len()
            + self.edge_buffer.free_cells.len()
            + self.vertex_buffer.free_cells.len()
            + self.point_buffer.free_cells.len()
    }

    pub fn active_element_count(&self) -> usize {
        self.face_buffer.len()
            + self.edge_buffer.len()
            + self.vertex_buffer.len()
            + self.point_buffer.len()
    }
}

impl GetElement<PointHandle> for Kernel {
    fn get(&self, handle: PointHandle) -> Option<&<PointHandle as ElementHandle>::Element> {
        self.point_buffer.get(handle)
    }
}

impl GetElement<VertexHandle> for Kernel {
    fn get(&self, handle: VertexHandle) -> Option<&<VertexHandle as ElementHandle>::Element> {
        self.vertex_buffer.get(handle)
    }
}

impl GetElement<HalfEdgeHandle> for Kernel {
    fn get(&self, handle: HalfEdgeHandle) -> Option<&<HalfEdgeHandle as ElementHandle>::Element> {
        self.edge_buffer.get(handle)
    }
}

impl GetElement<FaceHandle> for Kernel {
    fn get(&self, handle: FaceHandle) -> Option<&<FaceHandle as ElementHandle>::Element> {
        self.face_buffer.get(handle)
    }
}

impl AddElement<Point> for Kernel {
    fn add(&mut self, element: Point) -> <Point as Element>::Handle {
        self.point_buffer.add(element)
    }
}

impl AddElement<Vertex> for Kernel {
    fn add(&mut self, element: Vertex) -> <Vertex as Element>::Handle {
        self.vertex_buffer.add(element)
    }
}

impl AddElement<HalfEdge> for Kernel {
    fn add(&mut self, element: HalfEdge) -> <HalfEdge as Element>::Handle {
        self.edge_buffer.add(element)
    }
}

impl AddElement<Face> for Kernel {
    fn add(&mut self, element: Face) -> <Face as Element>::Handle {
        self.face_buffer.add(element)
    }
}

impl RemoveElement<PointHandle> for Kernel {
    fn remove(&mut self, handle: PointHandle) {
        self.point_buffer.remove(handle)
    }
}

impl RemoveElement<VertexHandle> for Kernel {
    fn remove(&mut self, handle: VertexHandle) {
        self.vertex_buffer.remove(handle)
    }
}

impl RemoveElement<HalfEdgeHandle> for Kernel {
    fn remove(&mut self, handle: HalfEdgeHandle) {
        self.edge_buffer.remove(handle)
    }
}

impl RemoveElement<FaceHandle> for Kernel {
    fn remove(&mut self, handle: FaceHandle) {
        self.face_buffer.remove(handle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::Element;
    use crate::data::{FaceData, HalfEdgeData, VertexData};
    use crate::handles::HalfEdgeHandle;




    fn make_twin_edge(kernel: &mut Kernel, twin_handle: HalfEdgeHandle) -> HalfEdgeHandle {
        let e0 = kernel.add(HalfEdge::with_data(HalfEdgeData {
            adjacent: twin_handle,
            ..HalfEdgeData::default()
        }));
        kernel.edge_buffer.buffer[twin_handle.index() as usize]
            .data_mut()
            .adjacent = e0;
        e0
    }

    fn get_twin(kernel: &Kernel, edge_handle: HalfEdgeHandle) -> HalfEdgeHandle {
        kernel.edge_buffer.buffer[edge_handle.index() as usize]
            .data()
            .adjacent
    }

    fn get_next(kernel: &Kernel, edge_handle: HalfEdgeHandle) -> HalfEdgeHandle {
        kernel.edge_buffer.buffer[edge_handle.index() as usize]
            .data()
            .next
    }

    #[allow(dead_code)]
    fn get_prev(kernel: &Kernel, edge_handle: HalfEdgeHandle) -> HalfEdgeHandle {
        kernel.edge_buffer.buffer[edge_handle.index() as usize]
            .data()
            .prev
    }

    fn connect_edges(
        kernel: &mut Kernel,
        prev_handle: HalfEdgeHandle,
        next_handle: HalfEdgeHandle,
    ) -> VertexHandle {
        let v0 = kernel.add(Vertex::default());
        match (kernel.get(prev_handle), kernel.get(next_handle)) {
            (Some(prev), Some(next)) => {
                prev.data_mut().next = next_handle;
                next.data_mut().prev = prev_handle;
                next.data_mut().vertex = v0;
            }
            _ => panic!(
                "Invalid edge handles specified: {:?}, {:?}",
                prev_handle, next_handle
            ),
        }
        v0
    }

    fn set_face_to_loop(kernel: &Kernel, root_edge: HalfEdgeHandle, face_handle: FaceHandle) {
        let face = kernel.face_buffer.get(face_handle).unwrap();
        face.data_mut().root_edge = root_edge;
        let mut edge_handle = root_edge;
        loop {
            let edge = &kernel.edge_buffer.buffer[edge_handle.index() as usize];
            let mut data = edge.data_mut();
            if data.face == face_handle {
                break;
            }
            data.face = face_handle;
            if data.next == root_edge {
                break;
            }
            edge_handle = data.next;
        }
    }

    fn make_face(kernel: &mut Kernel, root_edge: HalfEdgeHandle) -> FaceHandle {
        let face_handle = kernel.add(Face::with_data(FaceData {
            root_edge,
        }));
        set_face_to_loop(kernel, root_edge, face_handle);
        face_handle
    }

    fn make_triangle(kernel: &mut Kernel) -> FaceHandle {
        let (e0, _) = kernel.new_edge();
        let (e1, _) = kernel.new_edge();
        let (e2, _) = kernel.new_edge();

        let _ = connect_edges(kernel, e0, e1);
        let _ = connect_edges(kernel, e1, e2);
        let _ = connect_edges(kernel, e2, e0);

        make_face(kernel, e0)
    }

    #[test]
    fn defrag_faces() {
        let _ = env_logger::try_init();
        let mut kernel = Kernel::default();

        let f0 = make_triangle(&mut kernel);
        let root_edge = kernel.face_buffer.buffer[f0.index() as usize]
            .data()
            .root_edge;

        let f1 = make_face(&mut kernel, root_edge);
        let f2 = make_face(&mut kernel, root_edge);
        assert_eq!(kernel.face_buffer.len(), 4);
        assert_eq!(f2.index(), 3);
        assert_eq!(f2.generation(), 1);

        kernel.remove(f0);
        kernel.remove(f1);

        assert!(kernel.face_buffer.has_inactive_cells());
        assert_eq!(kernel.face_buffer.len(), 2);
        assert_eq!(kernel.face_buffer.free_cells.len(), 2);

        let root_face_handle = kernel.edge_buffer.buffer[root_edge.index() as usize]
            .data()
            .face;
        assert_eq!(root_face_handle, f2);

        kernel.defrag_faces();
        assert_eq!(kernel.face_buffer.len(), 2);
        assert_eq!(kernel.face_buffer.free_cells.len(), 0);
        assert!(!kernel.face_buffer.has_inactive_cells());
        assert!(kernel.get(f2).is_none());

        let root_face_handle = kernel.edge_buffer.buffer[root_edge.index() as usize]
            .data()
            .face;
        assert_ne!(root_face_handle, f2);
        assert!(kernel.get(root_face_handle).is_some());
        let face_edge_handle = kernel.face_buffer.buffer[root_face_handle.index() as usize]
            .data()
            .root_edge;
        assert_eq!(face_edge_handle, root_edge);
    }

    #[test]
    fn defrag_vertices() {
        let _ = env_logger::try_init();
        let mut kernel = Kernel::default();

        let (e0, _) = kernel.new_edge();
        let (e1, _) = kernel.new_edge();
        let (e2, _) = kernel.new_edge();

        let v0_0 = connect_edges(&mut kernel, e0, e1);
        let v0_1 = connect_edges(&mut kernel, e1, e2);
        let v0_2 = connect_edges(&mut kernel, e2, e0);

        let v1_0 = connect_edges(&mut kernel, e0, e1);
        let v1_1 = connect_edges(&mut kernel, e1, e2);
        let v1_2 = connect_edges(&mut kernel, e2, e0);

        let v2_0 = connect_edges(&mut kernel, e0, e1);
        let v2_1 = connect_edges(&mut kernel, e1, e2);
        let v2_2 = connect_edges(&mut kernel, e2, e0);

        assert_eq!(kernel.vertex_buffer.len(), 10);

        kernel.remove(v0_0);
        kernel.remove(v0_1);
        kernel.remove(v0_2);
        kernel.remove(v1_0);
        kernel.remove(v1_1);
        kernel.remove(v1_2);

        assert_eq!(kernel.vertex_buffer.len(), 4);
        assert_eq!(kernel.vertex_buffer.buffer.len(), 10);

        assert!(kernel.vertex_buffer.get(v2_0).is_some());
        assert!(kernel.vertex_buffer.get(v2_1).is_some());
        assert!(kernel.vertex_buffer.get(v2_2).is_some());

        kernel.defrag_verts();
        assert!(kernel.vertex_buffer.get(v2_0).is_none());
        assert!(kernel.vertex_buffer.get(v2_1).is_none());
        assert!(kernel.vertex_buffer.get(v2_2).is_none());
    }

    #[test]
    fn defrag_edges() {
        let _ = env_logger::try_init();
        let mut kernel = Kernel::default();

        let (e0, _) = kernel.new_edge();
        let (e1, _) = kernel.new_edge();
        let (e2, _) = kernel.new_edge();
        let _v0 = connect_edges(&mut kernel, e0, e1);
        let _v1 = connect_edges(&mut kernel, e1, e2);
        let _v2 = connect_edges(&mut kernel, e2, e0);

        let e3 = get_twin(&kernel, e0);
        let (e4, _) = kernel.new_edge();
        let (e5, _) = kernel.new_edge();
        let _v3 = connect_edges(&mut kernel, e3, e4);
        let _v4 = connect_edges(&mut kernel, e4, e5);
        let _v5 = connect_edges(&mut kernel, e5, e3);

        let e6 = get_twin(&kernel, e4);
        let e7 = get_twin(&kernel, e2);
        let (e8, _) = kernel.new_edge();
        let _v6 = connect_edges(&mut kernel, e6, e7);
        let _v7 = connect_edges(&mut kernel, e7, e8);
        let _v8 = connect_edges(&mut kernel, e8, e6);

        let e9 = get_twin(&kernel, e8);
        let e10 = get_twin(&kernel, e1);
        let e11 = get_twin(&kernel, e5);
        let _v9 = connect_edges(&mut kernel, e9, e10);
        let _v10 = connect_edges(&mut kernel, e10, e11);
        let _v11 = connect_edges(&mut kernel, e11, e9);

        let f0 = make_face(&mut kernel, e0);
        let _f1 = make_face(&mut kernel, e3);
        let _f2 = make_face(&mut kernel, e6);
        let _f3 = make_face(&mut kernel, e9);

        assert_eq!(kernel.active_element_count(), 32);
        assert_eq!(kernel.inactive_element_count(), 0);

        let e12 = make_twin_edge(&mut kernel, e3);
        let e13 = make_twin_edge(&mut kernel, e10);
        let e14 = make_twin_edge(&mut kernel, e7);
        let _v12 = connect_edges(&mut kernel, e12, e13);
        let _v13 = connect_edges(&mut kernel, e13, e14);
        let _v14 = connect_edges(&mut kernel, e14, e12);

        set_face_to_loop(&kernel, e12, f0);
        kernel.remove(e0);
        kernel.remove(e1);
        kernel.remove(e2);

        assert_eq!(kernel.active_element_count(), 35);
        assert_eq!(kernel.inactive_element_count(), 3);

        let face0 = &kernel.face_buffer.buffer[f0.index() as usize];
        let f0e0 = face0.data().root_edge;
        let f0e1 = get_next(&kernel, f0e0);
        let f0e2 = get_next(&kernel, f0e1);
        assert_eq!(f0e0, get_next(&kernel, f0e2));
        assert_eq!(13, f0e0.index());
        assert_eq!(14, f0e1.index());
        assert_eq!(15, f0e2.index());

        kernel.defrag_edges();
        assert_eq!(kernel.active_element_count(), 35);
        assert_eq!(kernel.inactive_element_count(), 0);

        // Because of how the edge defrag is implemented
        // we expect the offsets for the edges of f0
        // to be at the head of the edge buffer again
        // and basically reversed.
        let face0 = &kernel.face_buffer.buffer[f0.index() as usize];
        let f0e0 = face0.data().root_edge;
        let f0e1 = get_next(&kernel, f0e0);
        let f0e2 = get_next(&kernel, f0e1);
        assert_eq!(f0e0, get_next(&kernel, f0e2));
        assert_eq!(5, f0e0.index());
        assert_eq!(3, f0e1.index());
        assert_eq!(1, f0e2.index());
    }

    #[test]
    fn defrag_points() {
        let _ = env_logger::try_init();
        let mut kernel = Kernel::default();

        let p0 = kernel.add(Point::default());
        let p1 = kernel.add(Point::default());
        let p2 = kernel.add(Point::default());
        let p3 = kernel.add(Point::default());

        let v0 = kernel.add(Vertex::with_data(VertexData {
            point: p1,
            ..VertexData::default()
        }));
        let v1 = kernel.add(Vertex::with_data(VertexData {
            point: p1,
            ..VertexData::default()
        }));
        let v2 = kernel.add(Vertex::with_data(VertexData {
            point: p3,
            ..VertexData::default()
        }));
        let v3 = kernel.add(Vertex::with_data(VertexData {
            point: p3,
            ..VertexData::default()
        }));

        assert_eq!(
            kernel.vertex_buffer.buffer[v0.index() as usize]
                .data()
                .point
                .index(),
            2
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v1.index() as usize]
                .data()
                .point
                .index(),
            2
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v2.index() as usize]
                .data()
                .point
                .index(),
            4
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v3.index() as usize]
                .data()
                .point
                .index(),
            4
        );

        kernel.remove(p0);
        kernel.remove(p2);
        kernel.defrag_points();

        assert_eq!(
            kernel.vertex_buffer.buffer[v0.index() as usize]
                .data()
                .point
                .index(),
            2
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v1.index() as usize]
                .data()
                .point
                .index(),
            2
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v2.index() as usize]
                .data()
                .point
                .index(),
            1
        );
        assert_eq!(
            kernel.vertex_buffer.buffer[v3.index() as usize]
                .data()
                .point
                .index(),
            1
        );
    }
}
