//!
//! An index based half-edge mesh implementation.
//!

use std::fmt;
use std::sync::atomic;

#[cfg(feature="amethyst")]
pub mod amethyst;

pub use crate::handles::*;
pub use crate::elements::*;
pub use crate::kernel::*;
pub use crate::function_sets::*;
pub use crate::iterators::*;

pub mod handles;
pub mod elements;
pub mod kernel;
pub mod proxies;
pub mod mesh;
pub mod utils;
pub mod function_sets;
pub mod iterators;

pub type Tag = u32;
pub type Position = [f32; 3];
pub type Normal = [f32; 3];

/// An interface for asserting the validity of components and indices of the mesh.
pub trait IsValid {
    fn is_valid(&self) -> bool;
}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct EdgeData {
    /// The adjacent half-edge
    pub adjacent: EdgeHandle,
    /// The Handle of the next edge in the loop
    pub next: EdgeHandle,
    /// The Handle of the previous edge in the loop
    pub prev: EdgeHandle,
    /// The Handle of the face this edge loop defines
    pub face: FaceHandle,
    /// The Handle of the Vertex for this edge.
    pub vertex: VertexHandle,
}
pub type HalfEdge = MeshElement<EdgeData>;
pub type EdgeHandle = Handle<HalfEdge>;
impl ElementData for EdgeData {}
impl ElementHandle for  EdgeHandle {}
impl HalfEdge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        let data = self.data();
        data.next.is_valid() && data.prev.is_valid()
    }
}
impl IsValid for HalfEdge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        let data = self.data();
        self.is_active() &&
            data.vertex.is_valid() &&
            data.adjacent.is_valid() &&
            data.next.is_valid() &&
            data.prev.is_valid()
    }
}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct VertexData {
    /// Index of the outgoing edge
    pub edge: EdgeHandle,
    /// Index of point this vertex belongs to
    pub point: PointHandle,
}
pub type Vertex = MeshElement<VertexData>;
pub type VertexHandle = Handle<Vertex>;
impl ElementData for VertexData {}
impl ElementHandle for VertexHandle {}
impl Vertex {
    pub fn new(edge: EdgeHandle, point: PointHandle) -> Self {
        Vertex::with_data(VertexData { edge, point })
    }

    pub fn for_edge(edge: EdgeHandle) -> Self {
        Vertex::with_data(VertexData {
            edge,
            ..VertexData::default()
        })
    }

    pub fn at_point(point: PointHandle) -> Self {
        Vertex::with_data(VertexData {
            point,
            ..VertexData::default()
        })
    }
}
impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge.is_valid()
    }
}

/// TODO: Documentation
#[derive(Debug, Clone, Default)]
pub struct FaceData {
    /// The "root" of an edge loop that defines this face.
    pub edge: EdgeHandle,
}
pub type Face = MeshElement<FaceData>;
pub type FaceHandle = Handle<Face>;
impl ElementData for FaceData {}
impl ElementHandle for FaceHandle {}
impl Face {
    pub fn new(edge: EdgeHandle) -> Self {
        Face::with_data(FaceData { edge })
    }
}
impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge.is_valid()
    }
}

#[derive(Debug, Clone)]
pub struct PointData {
    pub position: Position,
}
impl PointData {
    pub fn new(position: Position) -> Self {
        PointData { position }
    }
}
impl Default for PointData {
    fn default() -> Self {
        PointData {
            position: [0.0; 3],
        }
    }
}
pub type Point = MeshElement<PointData>;
pub type PointHandle = Handle<Point>;
impl ElementData for PointData {}
impl ElementHandle for PointHandle {}
impl Point {
    pub fn new(position: Position) -> Self {
        Point::with_data(PointData::new(position))
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Self {
        Point::with_data(PointData::new([x, y, z]))
    }

    pub fn from_slice(offset: usize, values: &[f32]) -> Self {
        assert!(values.len() >= (offset + 3));
        Point::with_data(PointData::new([
            values[offset],
            values[offset+1],
            values[offset+2],
        ]))
    }
}
impl IsValid for Point {
    fn is_valid(&self) -> bool {
        self.is_active()
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Interface for adding elements to a `Mesh`.
pub trait AddElement<E> {
    fn add_element(&mut self, element: E) -> Handle<E>;
}

/// Interface for removing elements to a `Mesh`.
pub trait RemoveElement<E> {
    fn remove_element(&mut self, index: Handle<E>);
}

/// Interface for getting an element reference.
pub trait GetElement<E> {
    fn get_element(&self, index: &Handle<E>) -> Option<&E>;
}

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

    fn next_tag(&self) -> Tag {
        self.tag.fetch_add(1, atomic::Ordering::SeqCst)
    }

    /// Returns a `FaceFn` for the given index.
    pub fn face(&self, index: FaceHandle) -> FaceFn {
        FaceFn::new(index, &self)
    }

    pub fn face_count(&self) -> usize {
        self.kernel.face_buffer.len() - 1
    }

    pub fn faces(&self) -> impl Iterator<Item=FaceFn> {
        self.kernel.face_buffer.active_cells()
            .map(move |(offset, _)| {
                FaceFn::new(FaceHandle::new(offset as u32), self)
            })
    }

    /// Returns an `EdgeFn` for the given index.
    pub fn edge(&self, index: EdgeHandle) -> EdgeFn {
        EdgeFn::new(index, &self)
    }

    pub fn edge_count(&self) -> usize {
        self.kernel.edge_buffer.len() - 1
    }

    pub fn edges(&self) -> impl Iterator<Item=EdgeFn> {
        self.kernel.edge_buffer.active_cells()
            .map(move |(offset, _)| {
                EdgeFn::new(EdgeHandle::new(offset as u32), self)
            })
    }

    /// Returns a `VertexFn` for the given index.
    pub fn vertex(&self, index: VertexHandle) -> VertexFn {
        VertexFn::new(index, &self)
    }

    pub fn vertex_count(&self) -> usize {
        self.kernel.vertex_buffer.len() - 1
    }

    pub fn vertices(&self) -> impl Iterator<Item=VertexFn> {
        self.kernel.vertex_buffer.active_cells()
            .map(move |(offset, _)| {
                VertexFn::new(VertexHandle::new(offset as u32), self)
            })
    }

    pub fn point_count(&self) -> usize {
        self.kernel.point_buffer.len() - 1
    }

    pub fn add_element<E>(&mut self, element: E) -> Handle<E>
        where kernel::Kernel: AddElement<E>
    {
        self.kernel.add_element(element)
    }

    pub fn remove_element<E>(&mut self, index: Handle<E>)
        where kernel::Kernel: RemoveElement<E>
    {
        self.kernel.remove_element(index)
    }

    pub fn get_element<E>(&self, index: &Handle<E>) -> Option<&E>
        where kernel::Kernel: GetElement<E>
    {
        self.kernel.get_element(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let vert = EdgeHandle::default();
        assert!(!vert.is_valid());

        let edge = EdgeHandle::default();
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
        assert_eq!(mesh.get_element(&EdgeHandle::new(0)).is_some(), false);
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

        mesh.add_element(Face::new(EdgeHandle::new(1)));
        mesh.add_element(Face::new(EdgeHandle::new(4)));
        mesh.add_element(Face::new(EdgeHandle::new(7)));

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

        mesh.add_element(Vertex::new(EdgeHandle::new(1), PointHandle::new(1)));
        mesh.add_element(Vertex::new(EdgeHandle::new(1), PointHandle::new(1)));
        mesh.add_element(Vertex::new(EdgeHandle::new(1), PointHandle::new(1)));
        let v = mesh.add_element(Vertex::new(EdgeHandle::new(4), PointHandle::new(1)));
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
