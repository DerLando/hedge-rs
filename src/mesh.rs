
use log;
use std::fmt;
use std::sync::atomic;

use crate::kernel::Kernel;
use crate::elements::{
    Face, Vertex
};
use crate::data::{
    Tag,
};
use crate::handles::{
    HalfEdgeHandle, FaceHandle,
    VertexHandle, PointHandle,
};
use crate::traits::*;
use crate::proxy::*;

pub struct Mesh {
    pub kernel: Kernel,
    tag: atomic::AtomicU32,
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

impl Default for Mesh {
    fn default() -> Self {
        Mesh {
            kernel: Kernel::default(),
            tag: atomic::AtomicU32::new(1),
        }
    }
}

impl Mesh {
    pub fn next_tag(&self) -> Tag {
        self.tag.fetch_add(1, atomic::Ordering::SeqCst)
    }

    /// Returns a `FaceProxy` for the given handle.
    pub fn face(&self, handle: FaceHandle) -> FaceProxy {
        FaceProxy::new(handle, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_buffer.len() - 1
    }

    pub fn faces(&self) -> impl Iterator<Item=FaceProxy> {
        self.kernel.face_buffer.active_cells()
            .map(move |(handle, _)| {
                FaceProxy::new(FaceHandle::new(handle as u32), self)
            })
    }

    /// Returns an `EdgeProxy` for the given handle.
    pub fn edge(&self, handle: HalfEdgeHandle) -> HalfEdgeProxy {
        HalfEdgeProxy::new(handle, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    pub fn edges(&self) -> impl Iterator<Item=HalfEdgeProxy> {
        self.kernel.edge_buffer.active_cells()
            .map(move |(offset, _)| {
                HalfEdgeProxy::new(HalfEdgeHandle::new(offset as u32), self)
            })
    }

    /// Returns a `VertexProxy` for the given handle.
    pub fn vertex(&self, handle: VertexHandle) -> VertexProxy {
        VertexProxy::new(handle, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    pub fn vertices(&self) -> impl Iterator<Item=VertexProxy> {
        self.kernel.vertex_buffer.active_cells()
            .map(move |(offset, _)| {
                VertexProxy::new(VertexHandle::new(offset as u32), self)
            })
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }

    pub fn add<E: Element>(&mut self, element: E) -> E::Handle
        where Kernel: AddElement<E>
    {
        self.kernel.add(element)
    }

    pub fn remove<H: ElementHandle>(&mut self, handle: H)
        where Kernel: RemoveElement<H>
    {
        self.kernel.remove(handle)
    }

    pub fn get<H: ElementHandle>(
        &self, handle: H
    ) -> Option<&<H as ElementHandle>::Element>
        where Kernel: GetElement<H>
    {
        self.kernel.get(handle)
    }

    pub fn calculate_normals(&self) {
        unimplemented!()
    }
}

impl<'a> MakeEdge<(VertexHandle, VertexHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (v0, v1): (VertexHandle, VertexHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let (e0, e1) = self.kernel.new_edge();
        if let Some(e) = self.get(e0) { e.data_mut().vertex = v0; }
        if let Some(e) = self.get(e1) { e.data_mut().vertex = v1; }
        if let Some(v) = self.get(v0) { v.data_mut().edge = e0; }
        if let Some(v) = self.get(v1) { v.data_mut().edge = e1; }

        (e0, e1)
    }
}

impl<'a> MakeEdge<(PointHandle, PointHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (p0, p1): (PointHandle, PointHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let v0 = self.add(Vertex::at_point(p0));
        let v1 = self.add(Vertex::at_point(p1));

        self.make_edge((v0, v1))
    }
}

impl<'a> MakeEdge<(PointHandle, PointHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (p0, p1, face): (PointHandle, PointHandle, FaceHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let (e0, e1) = self.make_edge((p0, p1));
        if let Some(e) = self.get(e0) { e.data_mut().face = face; }
        (e0, e1)
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, PointHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, p1): (HalfEdgeHandle, PointHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let p0 = {
            match self.edge(e0).adjacent().vertex().data() {
                Some(data) => data.point,
                None => {
                    // Returning two invalid edge handles sucks.
                    // This entire adventure is just an enormous mess sometimes.
                    // I really hope to maybe find some route back to sanity.
                    return (Default::default(), Default::default());
                }
            }
        };
        self.make_edge((p0, p1))
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, PointHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, p1, face): (HalfEdgeHandle, PointHandle, FaceHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let edge_pair = self.make_edge((e0, p1));
        if let Some(e) = self.get(edge_pair.0) { e.data_mut().face = face; }
        edge_pair
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, HalfEdgeHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, e2): (HalfEdgeHandle, HalfEdgeHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let p0 = {
            match self.edge(e0).adjacent().vertex().data() {
                Some(data) => data.point,
                None => {
                    // Returning two invalid edge handles sucks.
                    // This entire adventure is just an enormous mess sometimes.
                    // I really hope to maybe find some route back to sanity.
                    log::error!("Unable to find the first point of the new edge.");
                    return (Default::default(), Default::default());
                }
            }
        };
        let p1 = {
            match self.edge(e2).vertex().data() {
                Some(data) => data.point,
                None => {
                    log::error!("Unable to find the second point of the new edge.");
                    return (Default::default(), Default::default());
                }
            }
        };
        self.make_edge((p0, p1))
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, HalfEdgeHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, e2, face): (HalfEdgeHandle, HalfEdgeHandle, FaceHandle)
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        let edge_pair = self.make_edge((e0, e2));
        if let Some(e) = self.get(edge_pair.0) { e.data_mut().face = face; }
        edge_pair
    }
}

impl<'a> AddFace<&'a [PointHandle]> for Mesh {
    fn add_face(&mut self, points: &'a [PointHandle]) -> FaceHandle {
        assert!(points.len() >= 3);
        let f0 = self.add(Face::default());

        let root_point = points[0];
        let current_point = points[1];
        let (root_edge, _) = self.make_edge((root_point, current_point, f0));
        self.add_face((root_edge, points.split_at(2).1, f0))
    }
}

impl<'a> AddFace<(HalfEdgeHandle, &'a [PointHandle])> for Mesh {
    fn add_face(
        &mut self,
        (edge_handle, points): (HalfEdgeHandle, &'a [PointHandle])
    ) -> FaceHandle {
        assert!(points.len() >= 1);
        let f0 = self.add(Face::default());
        self.add_face((edge_handle, points, f0))
    }
}

impl<'a> AddFace<(HalfEdgeHandle, &'a [PointHandle], FaceHandle)> for Mesh {
    fn add_face(
        &mut self,
        (root_edge, points, f0): (HalfEdgeHandle, &'a [PointHandle], FaceHandle)
    ) -> FaceHandle {
        assert!(points.len() >= 1);
        let mut previous_edge = root_edge;
        for current_point in points {
            let edge_pair = self.make_edge((previous_edge, *current_point, f0));
            previous_edge = edge_pair.0;
        }
        let _ = self.make_edge((previous_edge, root_edge, f0));
        f0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handles::PointHandle;
    use crate::elements::{
        HalfEdge, Face, Point, Vertex
    };
    use log::*;

    #[test]
    fn basic_debug_printing() {
        let _ = env_logger::try_init();

        let edge = HalfEdge::default();
        debug!("{:?}", edge);

        let vertex = Vertex::default();
        debug!("{:?}", vertex);

        let face = Face::default();
        debug!("{:?}", face);

        let point = Point::default();
        debug!("{:?}", point);

        let mesh = Mesh::default();
        debug!("{:?}", mesh);
    }

    #[test]
    fn handle_types_are_invalid_by_default() {
        let vert = HalfEdgeHandle::default();
        assert!(!vert.is_valid());

        let edge = HalfEdgeHandle::default();
        assert!(!edge.is_valid());

        let point = PointHandle::default();
        assert!(!point.is_valid());

        let face = FaceHandle::default();
        assert!(!face.is_valid());
    }

    #[test]
    fn default_edge_is_invalid() {
        let edge = HalfEdge::default();
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
        let mut mesh = Mesh::default();

        let phnd = {
            let point = Point::default();
            assert_eq!(point.is_valid(), false);
            mesh.add(point)
        };

        assert_eq!(mesh.get(phnd).is_some(), true);
    }

    #[test]
    fn initial_mesh_has_default_elements() {
        let _ = env_logger::try_init();
        let mesh = Mesh::default();

        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.get(HalfEdgeHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);

        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.get(FaceHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);

        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.get(VertexHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);

        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.get(PointHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_vertices() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();
        let v0 = mesh.add(Vertex::default());
        assert_eq!(mesh.vertex_count(), 1);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 2);
        mesh.remove(v0);
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_edges() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();
        let e0 = mesh.add(HalfEdge::default());
        assert_eq!(mesh.edge_count(), 1);
        assert_eq!(mesh.kernel.edge_buffer.len(), 2);
        mesh.remove(e0);
        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);

        let p0 = mesh.add(Point::from_position(0.0, 0.0, 0.0));
        let p1 = mesh.add(Point::from_position(0.0, 1.0, 0.0));
        let (e0, e1) = mesh.make_edge((p0, p1));
        assert!(mesh.edge(e0).is_boundary());
        assert!(mesh.edge(e1).is_boundary());
    }

    #[test]
    fn can_add_and_remove_faces() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();
        let f0 = mesh.add(Face::default());
        assert_eq!(mesh.face_count(), 1);
        assert_eq!(mesh.kernel.face_buffer.len(), 2);
        mesh.remove(f0);
        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);

        let p0 = mesh.add(Point::from_position(0.0, 0.0, 0.0));
        let p1 = mesh.add(Point::from_position(1.0, 0.0, 0.0));
        let p2 = mesh.add(Point::from_position(1.0, 1.0, 0.0));

        let f0 = mesh.add_face([p0, p1, p2].as_ref());
        assert!(f0.is_valid());
        assert!(mesh.face(f0).root_edge().is_boundary());
        assert!(mesh.face(f0).root_edge().next().is_boundary());
        assert!(mesh.face(f0).root_edge().prev().is_boundary());
    }

    #[test]
    fn can_add_and_remove_points() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();
        let p0 = mesh.add(Point::default());
        assert_eq!(mesh.point_count(), 1);
        assert_eq!(mesh.kernel.point_buffer.len(), 2);
        mesh.remove(p0);
        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[inline]
    fn point(edge: &HalfEdgeProxy) -> PointHandle {
        edge.vertex().data().map(|d| d.point).unwrap_or_else(Default::default)
    }

    #[test]
    fn can_build_a_simple_mesh_manually() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        let p0 = mesh.add(Point::from_position(-1.0, 0.0, 0.0));
        let p1 = mesh.add(Point::from_position(1.0, 0.0, 0.0));
        let p2 = mesh.add(Point::from_position(0.0, 1.0, 0.0));

        let f0 = mesh.add_face([p0, p1, p2].as_ref());

        let edges: Vec<HalfEdgeProxy> = mesh.edges().collect();

        dbg!(&edges);
        assert_eq!(edges.len(), mesh.edge_count());
        assert_eq!(edges.len(), 6);

        assert!(edges[0].is_boundary());
        assert!(edges[2].is_boundary());
        assert!(edges[4].is_boundary());

        assert_eq!(edges[0].face().handle, f0);
        assert_eq!(edges[2].face().handle, f0);
        assert_eq!(edges[4].face().handle, f0);

        assert_eq!(point(&edges[0]), p0);
        assert_eq!(point(&edges[2]), p1);
        assert_eq!(point(&edges[4]), p2);
    }

    #[test]
    fn can_iterate_over_faces() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        mesh.add(Face::new(HalfEdgeHandle::new(1)));
        mesh.add(Face::new(HalfEdgeHandle::new(4)));
        mesh.add(Face::new(HalfEdgeHandle::new(7)));

        assert_eq!(mesh.face_count(), 3);

        let mut faces_iterated_over = 0;

        for face in mesh.faces() {
            assert!(face.is_valid());
            faces_iterated_over += 1;
        }

        assert_eq!(faces_iterated_over, mesh.face_count());
    }

    #[test]
    fn can_iterate_over_vertices() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        mesh.add(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        mesh.add(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        mesh.add(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        let v = mesh.add(Vertex::new(HalfEdgeHandle::new(4), PointHandle::new(1)));
        mesh.remove(v);

        let mut vertices_iterated_over = 0;

        for vert in mesh.vertices() {
            assert!(vert.is_valid());
            assert_ne!(vert.edge().handle.index(), 4);
            vertices_iterated_over += 1;
        }

        assert_eq!(vertices_iterated_over, mesh.vertex_count());
    }
}
