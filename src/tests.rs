extern crate env_logger;

use super::*;
use log::*;

#[test]
fn basic_debug_printing() {
    let _ = env_logger::try_init();

    let edge = Edge::default();
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
    let vert = EdgeIndex::default();
    assert!(!vert.is_valid());

    let edge = EdgeIndex::default();
    assert!(!edge.is_valid());

    let point = PointIndex::default();
    assert!(!point.is_valid());

    let face = FaceIndex::default();
    assert!(!face.is_valid());
}

#[test]
fn default_edge_is_invalid() {
    let edge = Edge::default();
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
    assert_eq!(mesh.get_element(&EdgeIndex::new(0)).is_some(), false);
    assert_eq!(mesh.kernel.edge_buffer.len(), 1);

    assert_eq!(mesh.face_count(), 0);
    assert_eq!(mesh.get_element(&FaceIndex::new(0)).is_some(), false);
    assert_eq!(mesh.kernel.face_buffer.len(), 1);

    assert_eq!(mesh.vertex_count(), 0);
    assert_eq!(mesh.get_element(&VertexIndex::new(0)).is_some(), false);
    assert_eq!(mesh.kernel.vertex_buffer.len(), 1);

    assert_eq!(mesh.point_count(), 0);
    assert_eq!(mesh.get_element(&PointIndex::new(0)).is_some(), false);
    assert_eq!(mesh.kernel.point_buffer.len(), 1);
}

#[test]
fn can_add_vertices() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();
    let _ = mesh.add_element(Vertex::default());
    assert_eq!(mesh.vertex_count(), 1);
    assert_eq!(mesh.kernel.vertex_buffer.len(), 2);
}

#[test]
fn can_add_edges() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();
    let _ = mesh.add_element(Edge::default());
    assert_eq!(mesh.edge_count(), 1);
    assert_eq!(mesh.kernel.edge_buffer.len(), 2);
}

#[test]
fn can_add_faces() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();
    let _ = mesh.add_element(Face::default());
    assert_eq!(mesh.face_count(), 1);
    assert_eq!(mesh.kernel.face_buffer.len(), 2);
}

#[test]
fn can_add_points() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();
    let _ = mesh.add_element(Point::default());
    assert_eq!(mesh.point_count(), 1);
    assert_eq!(mesh.kernel.point_buffer.len(), 2);
}

#[test]
fn can_iterate_over_faces() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();

    mesh.add_element(Face::new(EdgeIndex::new(1)));
    mesh.add_element(Face::new(EdgeIndex::new(4)));
    mesh.add_element(Face::new(EdgeIndex::new(7)));

    unimplemented!()

    //assert_eq!(mesh.face_count(), 3);

    //let mut faces_iterated_over = 0;

    //for face in mesh.faces() {
    //    assert!(face.is_valid());
    //    faces_iterated_over += 1;
    //}
    //
    //assert_eq!(faces_iterated_over, 3);
}

#[test]
fn can_iterate_over_vertices() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();

    mesh.add_element(Vertex::new(EdgeIndex::new(1), PointIndex::new(1)));
    mesh.add_element(Vertex::new(EdgeIndex::new(1), PointIndex::new(1)));
    mesh.add_element(Vertex::new(EdgeIndex::new(1), PointIndex::new(1)));
    let v = mesh.add_element(Vertex::new(EdgeIndex::new(4), PointIndex::new(1)));
    mesh.remove_element(v);

    unimplemented!()

    //let mut vertices_iterated_over = 0;

    //for vert in mesh.vertices() {
    //    assert!(vert.is_valid());
    //    assert_ne!(vert.element.edge_index.get().offset, 4);
    //    vertices_iterated_over += 1;
    //}

    //assert_eq!(vertices_iterated_over, 3);
}

#[test]
fn can_iterate_over_edges() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();

    mesh.add_element(Edge::with_data(
        EdgeData {
            twin_index: EdgeIndex::new(1),
            next_index: EdgeIndex::new(2),
            prev_index: EdgeIndex::new(3),
            face_index: FaceIndex::new(1),
            vertex_index: VertexIndex::new(1),
        }
    ));

    mesh.add_element(Edge::with_data(
        EdgeData {
            twin_index: EdgeIndex::new(1),
            next_index: EdgeIndex::new(3),
            prev_index: EdgeIndex::new(1),
            face_index: FaceIndex::new(1),
            vertex_index: VertexIndex::new(2),
        }
    ));

    mesh.add_element(Edge::with_data(
        EdgeData {
            twin_index: EdgeIndex::new(1),
            next_index: EdgeIndex::new(1),
            prev_index: EdgeIndex::new(2),
            face_index: FaceIndex::new(1),
            vertex_index: VertexIndex::new(3),
        }
    ));

    unimplemented!()

    //let mut edges_iterated_over = 0;

    //for edge in mesh.edges() {
    //    assert!(edge.is_valid());
    //    edges_iterated_over += 1;
    //}

    //assert_eq!(edges_iterated_over, 3);
}

#[test]
fn can_iterate_over_edges_of_face() {
    let _ = env_logger::try_init();
    let mut mesh = Mesh::new();
    let v0 = mesh.add_element(Vertex::default());
    let v1 = mesh.add_element(Vertex::default());
    let v2 = mesh.add_element(Vertex::default());

    assert_eq!(mesh.vertex_count(), 3);
    assert_eq!(mesh.edge_count(), 6);
    assert_eq!(mesh.face_count(), 1);

    unimplemented!();
}

#[test]
fn can_iterate_over_vertices_of_face() {
    unimplemented!();
}

#[test]
fn can_add_triangles_to_mesh() {
    let _ = env_logger::try_init();
    unimplemented!();
}

#[test]
fn can_build_a_simple_mesh() {
    unimplemented!();
}

#[test]
fn can_iterate_edges_around_vertex() {
    unimplemented!();
}

#[test]
fn can_remove_faces() {
    unimplemented!();
}

#[test]
fn can_remove_vertices() {
    unimplemented!();
}

#[test]
fn can_remove_edge() {
    unimplemented!();
}
