
use std::fmt;
use std::sync::atomic;

use crate::kernel::Kernel;
use crate::data::Tag;
use crate::handles::Handle;
use crate::traits::*;
use crate::elements::*;
use crate::proxy::*;

pub struct Mesh {
    kernel: Kernel,
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

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            kernel: Kernel::default(),
            tag: atomic::AtomicU32::new(1),
        }
    }

    pub fn next_tag(&self) -> Tag {
        self.tag.fetch_add(1, atomic::Ordering::SeqCst)
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceHandle) -> FaceProxy {
        FaceProxy::new(index, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_buffer.len() - 1
    }

    pub fn faces(&self) -> impl Iterator<Item=FaceProxy> {
        self.kernel.face_buffer.active_cells()
            .map(move |(offset, _)| {
                FaceProxy::new(FaceHandle::new(offset as u32), self)
            })
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: HalfEdgeHandle) -> HalfEdgeProxy {
        HalfEdgeProxy::new(index, &self)
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

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexHandle) -> VertexProxy {
        VertexProxy::new(index, &self)
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

    pub fn add_element<E>(&mut self, element: E) -> Handle<E>
        where Kernel: AddElement<E>
    {
        self.kernel.add_element(element)
    }

    pub fn remove_element<E>(&mut self, index: Handle<E>)
        where Kernel: RemoveElement<E>
    {
        self.kernel.remove_element(index)
    }

    pub fn get_element<E>(&self, index: &Handle<E>) -> Option<&E>
        where Kernel: GetElement<E>
    {
        self.kernel.get_element(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils;
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

        let mesh = Mesh::new();
        debug!("{:?}", mesh);
    }

    #[test]
    fn index_types_are_invalid_by_default() {
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
        let mut mesh = Mesh::new();

        let pindex = {
            let point = Point::default();
            assert_eq!(point.is_valid(), false);
            mesh.add_element(point)
        };

        assert_eq!(mesh.get_element(&pindex).is_some(), true);
    }

    #[test]
    fn initial_mesh_has_default_elements() {
        let _ = env_logger::try_init();
        let mesh = Mesh::new();

        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.get_element(&HalfEdgeHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);

        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.get_element(&FaceHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);

        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.get_element(&VertexHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);

        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.get_element(&PointHandle::new(0)).is_some(), false);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_vertices() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let v0 = mesh.add_element(Vertex::default());
        assert_eq!(mesh.vertex_count(), 1);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 2);
        mesh.remove_element(v0);
        assert_eq!(mesh.vertex_count(), 0);
        assert_eq!(mesh.kernel.vertex_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_edges() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let e0 = mesh.add_element(HalfEdge::default());
        assert_eq!(mesh.edge_count(), 1);
        assert_eq!(mesh.kernel.edge_buffer.len(), 2);
        mesh.remove_element(e0);
        assert_eq!(mesh.edge_count(), 0);
        assert_eq!(mesh.kernel.edge_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_faces() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let f0 = mesh.add_element(Face::default());
        assert_eq!(mesh.face_count(), 1);
        assert_eq!(mesh.kernel.face_buffer.len(), 2);
        mesh.remove_element(f0);
        assert_eq!(mesh.face_count(), 0);
        assert_eq!(mesh.kernel.face_buffer.len(), 1);
    }

    #[test]
    fn can_add_and_remove_points() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();
        let p0 = mesh.add_element(Point::default());
        assert_eq!(mesh.point_count(), 1);
        assert_eq!(mesh.kernel.point_buffer.len(), 2);
        mesh.remove_element(p0);
        assert_eq!(mesh.point_count(), 0);
        assert_eq!(mesh.kernel.point_buffer.len(), 1);
    }

    #[test]
    fn can_build_a_simple_mesh_manually() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();

        let p0 = mesh.add_element(Point::from_coords(-1.0, 0.0, 0.0));
        let p1 = mesh.add_element(Point::from_coords(1.0, 0.0, 0.0));
        let p2 = mesh.add_element(Point::from_coords(0.0, 1.0, 0.0));

        let v0 = mesh.add_element(Vertex::at_point(p0));
        let v1 = mesh.add_element(Vertex::at_point(p1));
        let v2 = mesh.add_element(Vertex::at_point(p2));

        let e0 = utils::build_full_edge(&mut mesh, v0, v1);
        let e1 = utils::build_full_edge_from(&mut mesh, e0, v2);
        let e2 = utils::close_edge_loop(&mut mesh, e1, e0);

        let f0 = mesh.add_element(Face::default());
        utils::assign_face_to_loop(&mesh, e0, f0);

        assert!(mesh.edge(e0).is_boundary());
        assert!(mesh.edge(e1).is_boundary());
        assert!(mesh.edge(e2).is_boundary());
        assert_eq!(mesh.edge(e0).face().index, f0);
        assert_eq!(mesh.edge(e1).face().index, f0);
        assert_eq!(mesh.edge(e2).face().index, f0);

        assert_eq!(mesh.edge(e0).vertex().index, v0);
        assert_eq!(mesh.edge(e1).vertex().index, v1);
        assert_eq!(mesh.edge(e2).vertex().index, v2);

        assert_eq!(mesh.edge(e0).twin().vertex().index, v1);
        assert_eq!(mesh.edge(e1).twin().vertex().index, v2);
        assert_eq!(mesh.edge(e2).twin().vertex().index, v0);
    }

    #[test]
    fn can_iterate_over_faces() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();

        mesh.add_element(Face::new(HalfEdgeHandle::new(1)));
        mesh.add_element(Face::new(HalfEdgeHandle::new(4)));
        mesh.add_element(Face::new(HalfEdgeHandle::new(7)));

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
        let mut mesh = Mesh::new();

        mesh.add_element(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        mesh.add_element(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        mesh.add_element(Vertex::new(HalfEdgeHandle::new(1), PointHandle::new(1)));
        let v = mesh.add_element(Vertex::new(HalfEdgeHandle::new(4), PointHandle::new(1)));
        mesh.remove_element(v);

        let mut vertices_iterated_over = 0;

        for vert in mesh.vertices() {
            assert!(vert.is_valid());
            assert_ne!(vert.edge().index.offset, 4);
            vertices_iterated_over += 1;
        }

        assert_eq!(vertices_iterated_over, mesh.vertex_count());
    }
}
