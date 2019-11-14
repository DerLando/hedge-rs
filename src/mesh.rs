use std::fmt;
use std::sync::atomic;

use crate::data::Tag;
use crate::elements::{Face, Point, Vertex};
use crate::handles::{FaceHandle, HalfEdgeHandle, PointHandle, VertexHandle};
use crate::kernel::Kernel;
use crate::proxy::*;
use crate::traits::*;

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
    pub fn unit_cube() -> Self {
        let mut mesh = Mesh::default();
        let p = [
            mesh.add(Point::from_position(0.0, 0.0, 0.0)),
            mesh.add(Point::from_position(1.0, 0.0, 0.0)),
            mesh.add(Point::from_position(1.0, 0.0, 1.0)),
            mesh.add(Point::from_position(0.0, 0.0, 1.0)),
            mesh.add(Point::from_position(0.0, 1.0, 0.0)),
            mesh.add(Point::from_position(1.0, 1.0, 0.0)),
            mesh.add(Point::from_position(1.0, 1.0, 1.0)),
            mesh.add(Point::from_position(0.0, 1.0, 1.0)),
        ];

        let front = mesh.add_face([p[0], p[1], p[5], p[4]].as_ref());
        let back = mesh.add_face([p[2], p[3], p[7], p[6]].as_ref());

        let bottom = mesh.add_face(
            [
                mesh.face(front).root_edge().adjacent().handle,
                mesh.face(back).root_edge().adjacent().handle,
            ]
            .as_ref(),
        );

        let top = mesh.add_face(
            [
                mesh.face(front).root_edge().next().next().adjacent().handle,
                mesh.face(back).root_edge().next().next().adjacent().handle,
            ]
            .as_ref(),
        );

        let _right = mesh.add_face(
            [
                mesh.face(front).root_edge().next().adjacent().handle,
                mesh.face(bottom).root_edge().prev().adjacent().handle,
                mesh.face(back).root_edge().prev().adjacent().handle,
                mesh.face(top).root_edge().next().adjacent().handle,
            ]
            .as_ref(),
        );

        let _left = mesh.add_face(
            [
                mesh.face(front).root_edge().prev().adjacent().handle,
                mesh.face(bottom).root_edge().next().adjacent().handle,
                mesh.face(back).root_edge().next().adjacent().handle,
                mesh.face(top).root_edge().prev().adjacent().handle,
            ]
            .as_ref(),
        );

        mesh
    }

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

    pub fn faces(&self) -> impl Iterator<Item = FaceProxy> {
        self.kernel
            .face_buffer
            .active_cells()
            .map(move |(handle, _)| FaceProxy::new(FaceHandle::new(handle as u32), self))
    }

    /// Returns an `EdgeProxy` for the given handle.
    pub fn edge(&self, handle: HalfEdgeHandle) -> HalfEdgeProxy {
        HalfEdgeProxy::new(handle, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    pub fn edges(&self) -> impl Iterator<Item = HalfEdgeProxy> {
        self.kernel
            .edge_buffer
            .active_cells()
            .map(move |(offset, _)| HalfEdgeProxy::new(HalfEdgeHandle::new(offset as u32), self))
    }

    /// Returns a `VertexProxy` for the given handle.
    pub fn vertex(&self, handle: VertexHandle) -> VertexProxy {
        VertexProxy::new(handle, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    pub fn vertices(&self) -> impl Iterator<Item = VertexProxy> {
        self.kernel
            .vertex_buffer
            .active_cells()
            .map(move |(offset, _)| VertexProxy::new(VertexHandle::new(offset as u32), self))
    }

    pub fn point(&self, handle: PointHandle) -> PointProxy {
        PointProxy::new(handle, &self)
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }

    pub fn add<E: Element>(&mut self, element: E) -> E::Handle
    where
        Kernel: AddElement<E>,
    {
        self.kernel.add(element)
    }

    pub fn remove<H: ElementHandle>(&mut self, handle: H)
    where
        Kernel: RemoveElement<H>,
    {
        self.kernel.remove(handle)
    }

    pub fn get<H: ElementHandle>(&self, handle: H) -> Option<&<H as ElementHandle>::Element>
    where
        Kernel: GetElement<H>,
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
        (v0, v1): (VertexHandle, VertexHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "-     MakeEdge<(VertexHandle({}), VertexHandle({}))>",
            v0.index(),
            v1.index()
        );
        let (e0, e1) = self.kernel.new_edge();
        if let Some(e) = self.get(e0) {
            e.data_mut().vertex = v0;
        }
        if let Some(e) = self.get(e1) {
            e.data_mut().vertex = v1;
        }
        if let Some(v) = self.get(v0) {
            v.data_mut().edge = e0;
        }
        if let Some(v) = self.get(v1) {
            v.data_mut().edge = e1;
        }

        (e0, e1)
    }
}

impl<'a> MakeEdge<(VertexHandle, VertexHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (v0, v1, face): (VertexHandle, VertexHandle, FaceHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "-    MakeEdge<(VertexHandle({}), VertexHandle({}), FaceHandle({}))>",
            v0.index(),
            v1.index(),
            face.index()
        );
        let edge_pair = self.make_edge((v0, v1));
        if let Some(e) = self.get(edge_pair.0) {
            e.data_mut().face = face;
        }
        edge_pair
    }
}

impl<'a> MakeEdge<(PointHandle, PointHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (p0, p1): (PointHandle, PointHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "-   MakeEdge<(PointHandle({}), PointHandle({}))>",
            p0.index(),
            p1.index()
        );
        let v0 = self.add(Vertex::at_point(p0));
        let v1 = self.add(Vertex::at_point(p1));

        self.make_edge((v0, v1))
    }
}

impl<'a> MakeEdge<(PointHandle, PointHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (p0, p1, face): (PointHandle, PointHandle, FaceHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "-  MakeEdge<(PointHandle({}), PointHandle({}), FaceHandle({}))>",
            p0.index(),
            p1.index(),
            face.index()
        );
        let (e0, e1) = self.make_edge((p0, p1));
        if let Some(e) = self.get(e0) {
            e.data_mut().face = face;
        }
        (e0, e1)
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, PointHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, p1): (HalfEdgeHandle, PointHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "- MakeEdge<(HalfEdgeHandle({}), PointHandle({}))>",
            e0.index(),
            p1.index()
        );
        let p0 = self.edge(e0).adjacent().vertex().point().handle;
        let v0 = self.add(Vertex::at_point(p0));
        let v1 = self.add(Vertex::at_point(p1));
        let edge_pair = self.make_edge((v0, v1));
        let base_edge = self.edge(e0);
        let next_edge = self.edge(edge_pair.0);
        base_edge.connect_to(&next_edge);
        edge_pair
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, PointHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, p1, face): (HalfEdgeHandle, PointHandle, FaceHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "- MakeEdge<(HalfEdgeHandle({}), PointHandle({}), FaceHandle({}))>",
            e0.index(),
            p1.index(),
            face.index()
        );
        let edge_pair = self.make_edge((e0, p1));
        if let Some(e) = self.get(edge_pair.0) {
            e.data_mut().face = face;
        }
        edge_pair
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, HalfEdgeHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, e2): (HalfEdgeHandle, HalfEdgeHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "- MakeEdge<(HalfEdgeHandle({}), HalfEdgeHandle({}))>",
            e0.index(),
            e2.index()
        );
        let p0 = self.edge(e0).adjacent().vertex().point().handle;
        let p1 = self.edge(e2).vertex().point().handle;
        let edge_pair = self.make_edge((p0, p1));
        let base_edge = self.edge(e0);
        let next_edge = self.edge(edge_pair.0);
        let last_edge = self.edge(e2);
        base_edge.connect_to(&next_edge);
        next_edge.connect_to(&last_edge);
        edge_pair
    }
}

impl<'a> MakeEdge<(HalfEdgeHandle, HalfEdgeHandle, FaceHandle)> for Mesh {
    fn make_edge(
        &mut self,
        (e0, e2, face): (HalfEdgeHandle, HalfEdgeHandle, FaceHandle),
    ) -> (HalfEdgeHandle, HalfEdgeHandle) {
        log::trace!(
            "- MakeEdge<(HalfEdgeHandle({}), HalfEdgeHandle({}), FaceHandle({}))>",
            e0.index(),
            e2.index(),
            face.index()
        );
        let edge_pair = self.make_edge((e0, e2));
        if let Some(e) = self.get(edge_pair.0) {
            e.data_mut().face = face;
        }
        edge_pair
    }
}

impl<'a> AddFace<&'a [PointHandle]> for Mesh {
    fn add_face(&mut self, points: &'a [PointHandle]) -> FaceHandle {
        log::trace!("- AddFace<&'a [PointHandle]>");
        assert!(points.len() >= 3);
        let f0 = self.add(Face::default());
        let root_point = points[0];
        let current_point = points[1];
        let (root_edge, _) = self.make_edge((root_point, current_point, f0));
        self.add_face((root_edge, points.split_at(2).1, f0))
    }
}

impl<'a> AddFace<(HalfEdgeHandle, &'a [PointHandle])> for Mesh {
    fn add_face(&mut self, (root_edge, points): (HalfEdgeHandle, &'a [PointHandle])) -> FaceHandle {
        log::trace!(
            "- AddFace<(HalfEdgeHandle({}), &'a [PointHandle])>",
            root_edge.index()
        );
        assert!(!points.is_empty());
        let f0 = self.add(Face::default());
        self.add_face((root_edge, points, f0))
    }
}

impl<'a> AddFace<(HalfEdgeHandle, &'a [PointHandle], FaceHandle)> for Mesh {
    fn add_face(
        &mut self,
        (root_edge, points, f0): (HalfEdgeHandle, &'a [PointHandle], FaceHandle),
    ) -> FaceHandle {
        log::trace!(
            "- AddFace<(HalfEdgeHandle({}), &'a [PointHandle], FaceHandle({}))>",
            root_edge.index(),
            f0.index()
        );
        assert!(!points.is_empty());
        let mut previous_edge = root_edge;
        for current_point in points {
            let edge_pair = self.make_edge((previous_edge, *current_point, f0));
            previous_edge = edge_pair.0;
        }
        let _ = self.make_edge((previous_edge, root_edge, f0));
        if let Some(face) = self.get(f0) {
            face.data_mut().root_edge = root_edge;
        }
        f0
    }
}

impl<'a> AddFace<(HalfEdgeHandle, HalfEdgeHandle)> for Mesh {
    fn add_face(&mut self, (e0, e2): (HalfEdgeHandle, HalfEdgeHandle)) -> FaceHandle {
        log::trace!(
            "- AddFace<(HalfEdgeHandle({}), HalfEdgeHandle({}))>",
            e0.index(),
            e2.index()
        );
        // let face = self.add(Face::default());
        // let _edge_pair = self.make_edge((e0, e2, face));
        // if let Some(face) = self.get(face) {
        //     face.data_mut().root_edge = e0;
        // }
        // self.edge(e2).connect_to(&self.edge(e0));
        // face
        self.add_face([e0, e2].as_ref())
    }
}

impl<'a> AddFace<&[HalfEdgeHandle]> for Mesh {
    fn add_face(&mut self, edges: &[HalfEdgeHandle]) -> FaceHandle {
        log::trace!("- AddFace<&[HalfEdgeHandle]>");
        assert!(edges.len() >= 2);
        let face = self.add(Face::default());
        let root_edge = edges[0];
        if let Some(mut data) = self.edge(root_edge).data_mut() {
            data.face = face;
        }
        let mut last_edge = root_edge;
        for next_edge in edges.iter().skip(1) {
            let last_point = self.edge(last_edge).adjacent().vertex().point().handle;
            let next_point = self.edge(*next_edge).adjacent().vertex().point().handle;
            if last_point == next_point {
                self.edge(last_edge).connect_to(&self.edge(*next_edge));
            } else {
                let _ = self.make_edge((last_edge, *next_edge, face));
            }

            if let Some(edge) = self.get(*next_edge) {
                edge.data_mut().face = face;
            }
            last_edge = *next_edge;
        }
        let root_point = self.edge(root_edge).vertex().point().handle;
        let last_point = self.edge(last_edge).adjacent().vertex().point().handle;
        if root_point == last_point {
            self.edge(last_edge).connect_to(&self.edge(root_edge));
        } else {
            let _ = self.make_edge((last_edge, root_edge, face));
        }

        if let Some(mut data) = self.face(face).data_mut() {
            data.root_edge = root_edge;
        }
        face
    }
}

impl<'a> AddFace<(&[HalfEdgeHandle], &[PointHandle])> for Mesh {
    fn add_face(&mut self, (edges, points): (&[HalfEdgeHandle], &[PointHandle])) -> FaceHandle {
        log::trace!("- AddFace<&[HalfEdgeHandle]>");
        assert!(!edges.is_empty());
        if edges.len() == 1 {
            assert!(!points.is_empty());
            self.add_face((edges[0], points))
        } else if edges.len() == 2 && points.is_empty() {
            self.add_face(edges)
        } else {
            let face = self.add(Face::default());

            let root_edge = edges[0];
            if let Some(mut data) = self.edge(root_edge).data_mut() {
                data.face = face;
            }

            let mut last_edge = root_edge;
            for next_edge in edges.iter().skip(1).map(|h| self.edge(*h)) {
                self.edge(last_edge).connect_to(&next_edge);
                if let Some(mut data) = next_edge.data_mut() {
                    data.face = face;
                }
                last_edge = next_edge.handle;
            }

            for current_point in points {
                let edge_pair = self.make_edge((last_edge, *current_point, face));
                last_edge = edge_pair.0;
            }

            let _ = self.make_edge((last_edge, root_edge, face));
            if let Some(face) = self.get(face) {
                face.data_mut().root_edge = root_edge;
            }

            face
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::elements::{Face, HalfEdge, Point, Vertex};
    use crate::handles::PointHandle;
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
        edge.vertex()
            .data()
            .map(|d| d.point)
            .unwrap_or_else(Default::default)
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
