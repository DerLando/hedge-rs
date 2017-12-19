
extern crate env_logger;

use super::*;

type TestMesh = Mesh;

#[test]
fn basic_debug_printing() {
    let _ = env_logger::init();
    let edge = Edge::default();
    debug!("{:?}", edge);
    let vertex = Vertex::default();
    debug!("{:?}", vertex);
    let face = Face::default();
    debug!("{:?}", face);
    let mesh = TestMesh::new();
    debug!("{:?}", mesh);

}

#[test]
fn index_types_are_invalid_by_default() {
    let vert = EdgeIndex::default();
    let edge = EdgeIndex::default();
    assert!(!vert.is_valid());
    assert!(!edge.is_valid());
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
fn initial_mesh_has_default_elements() {
    let _ = env_logger::init();
    let mesh = Mesh::new();
    assert_eq!(mesh.edge_list.len(), 1);
    assert!(!mesh.edge_list[0].is_valid());

    assert_eq!(mesh.vertex_list.len(), 1);
    assert!(!mesh.vertex_list[0].is_valid());

    assert_eq!(mesh.face_list.len(), 1);
    assert!(!mesh.face_list[0].is_valid());
}

#[test]
fn can_iterate_over_faces() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();
    mesh.face_list.push(Face::new(EdgeIndex::new(1)));
    mesh.face_list.push(Face::new(EdgeIndex::new(4)));
    mesh.face_list.push(Face::new(EdgeIndex::new(7)));

    assert_eq!(mesh.face_list.len(), 4);

    let mut faces_iterated_over = 0;

    for index in mesh.faces() {
        let face = &mesh.face(index);
        assert!(face.is_valid());
        faces_iterated_over += 1;
    }

    assert_eq!(faces_iterated_over, 3);
}

#[test]
fn can_iterate_over_edges_of_face() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();
    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let _face = mesh.add(triangle::FromVerts(v0, v1, v2));

    assert_eq!(mesh.vertex_list.len(), 4);
    assert_eq!(mesh.edge_list.len(), 7);
    assert_eq!(mesh.face_list.len(), 2);

    let mut faces_iterated_over = 0;
    let mut edges_iterated_over = 0;

    for face_index in mesh.faces() {
        let face = &mesh.face(face_index);
        assert!(face.is_valid());
        faces_iterated_over += 1;

        for edge_index in mesh.edges(face) {
            let edge = &mesh.edge(edge_index);
            assert!(edge.is_valid());
            edges_iterated_over += 1;
        }
    }

    assert_eq!(faces_iterated_over, 1);
    assert_eq!(edges_iterated_over, 3);
}

#[test]
fn can_iterate_over_vertices_of_face() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();
    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let _face = mesh.add(triangle::FromVerts(v0, v1, v2));

    let mut faces_iterated_over = 0;
    let mut vertices_iterated_over = 0;

    for face_index in mesh.faces() {
        assert!(face_index.is_valid());
        let face = &mesh.face(face_index);
        assert!(face.is_valid());
        faces_iterated_over += 1;

        for vertex_index in mesh.vertices(face) {
            assert!(vertex_index.is_valid());
            let vertex = &mesh.vertex(vertex_index);
            assert!(vertex.is_valid());
            vertices_iterated_over += 1;
        }
    }

    assert_eq!(faces_iterated_over, 1);
    assert_eq!(vertices_iterated_over, 3);
}

#[test]
fn can_add_triangles_to_mesh() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let v3 = mesh.add(Vertex::default());

    let f1 = mesh.add(triangle::FromVerts(v0, v1, v3));
    for eindex in mesh.edges(&mesh.face(f1)) {
        let edge = &mesh.edge(eindex);
        assert!(edge.next_index.is_valid());
        assert!(edge.prev_index.is_valid());
    }
    assert_eq!(mesh.face_fn(f1).edge().face().index, f1);

    let f1e1 = mesh.face_fn(f1).edge().next().index;
    let f2e0 = mesh.edge(f1e1).twin_index;
    assert!(f2e0.is_valid());

    let f2 = mesh.add(triangle::FromOneEdge(f2e0, v2));
    for eindex in mesh.edges(&mesh.face(f1)) {
        let edge = &mesh.edge(eindex);
        assert!(edge.next_index.is_valid());
        assert!(edge.prev_index.is_valid());
    }
    assert_eq!(mesh.face_fn(f2).edge().face().index, f2);

    assert!(f1e1.is_valid());

    assert_eq!(mesh.edge(f2e0).twin_index, f1e1);
    assert_eq!(mesh.edge(f1e1).twin_index, f2e0);

    assert_eq!(mesh.edge(f2e0).vertex_index, mesh.edge_fn(f1e1).next().vertex().index);
    assert_eq!(mesh.edge(f1e1).vertex_index, mesh.edge_fn(f2e0).next().vertex().index);
}

#[test]
fn can_walk_and_get_mutable_ref() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());

    let f1 = mesh.add(triangle::FromVerts(v0, v1, v2));

    {
        let vertex = {
            let index = mesh.face_fn(f1).edge().vertex().index;
            &mut mesh.vertex_mut(index)
        };
        debug!("{:?}", vertex);
        assert_eq!(vertex.edge_index.offset, 6);
        vertex.edge_index = EdgeIndex::default();
    }

    assert!(!mesh.face_fn(f1).edge().vertex().edge().index.is_valid());
}

fn print_mesh(mesh: &Mesh) {
    debug!("{:?}", mesh);
    for findex in mesh.faces() {
        let face = &mesh.face(findex);
        let mut edge_count = 0;
        debug!("\t{:?} - {:?}", findex, face);
        for eindex in mesh.edges(face) {
            edge_count += 1;
            let edge = &mesh.edge(eindex);
            debug!("\t\t{:?} - {:?}", eindex, edge);
            assert!(edge_count < 4);
        }
    }
}

#[test]
fn can_build_a_simple_mesh() {
    let _ = env_logger::init();
    debug!("===========================");
    debug!("=====Creating new mesh=====");
    let mut mesh = TestMesh::new();
    debug!("");

    debug!("===========================");
    debug!("====Creating 4 vertices====");
    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let v3 = mesh.add(Vertex::default());
    debug!("");

    debug!("===========================");
    debug!("====Creating triangle 1====");
    let f1 = mesh.add(triangle::FromVerts(v0, v1, v2));
    debug!("");
    debug!("=================================================");
    print_mesh(&mesh);
    debug!("=================================================");
    debug!("");

    debug!("===========================");
    debug!("====Creating triangle 2====");
    let f2 = {
        let f2e0 = mesh.face_fn(f1).edge().twin().index;
        mesh.add(triangle::FromOneEdge(f2e0, v3))
    };
    debug!("");
    debug!("=================================================");
    print_mesh(&mesh);
    debug!("=================================================");
    debug!("");

    debug!("===========================");
    debug!("====Creating triangle 3====");
    let f3 = {
        let f1e1 = mesh.face_fn(f1).edge().next().twin().index;
        let f2e2 = mesh.face_fn(f2).edge().prev().twin().index;
        mesh.add(triangle::FromTwoEdges(f1e1, f2e2, triangle::EdgeOrder::AB))
    };
    debug!("");
    debug!("=================================================");
    print_mesh(&mesh);
    debug!("=================================================");
    debug!("");

    debug!("===========================");
    debug!("====Creating triangle 4====");
    let f4 = {
        let f1e2 = mesh.face_fn(f1).edge().prev().twin().index;
        let f3e2 = mesh.face_fn(f3).edge().prev().twin().index;
        let f2e1 = mesh.face_fn(f2).edge().next().twin().index;
        mesh.add(triangle::FromThreeEdges(f1e2, f3e2, f2e1))
    };

    debug!("");
    debug!("=================================================");
    print_mesh(&mesh);
    debug!("=================================================");
    debug!("");

    assert_eq!(mesh.face_fn(f1).edge().twin().face().index       , f2);
    assert_eq!(mesh.face_fn(f1).edge().next().twin().face().index, f3);
    assert_eq!(mesh.face_fn(f1).edge().prev().twin().face().index, f4);

    assert_eq!(mesh.face_fn(f2).edge().next().twin().face().index, f4);
    assert_eq!(mesh.face_fn(f2).edge().prev().twin().face().index, f3);

    assert_eq!(mesh.face_fn(f3).edge().next().twin().face().index, f2);
    assert_eq!(mesh.face_fn(f3).edge().prev().twin().face().index, f4);

    assert_eq!(mesh.face_fn(f4).edge().next().twin().face().index, f3);
    assert_eq!(mesh.face_fn(f4).edge().prev().twin().face().index, f2);

    assert_eq!(mesh.face_fn(f1).edge().prev().vertex().index, mesh.face_fn(f3).edge().vertex().index);
    assert_eq!(mesh.face_fn(f1).edge().vertex().index       , mesh.face_fn(f4).edge().vertex().index);
    assert_eq!(mesh.face_fn(f1).edge().next().vertex().index, mesh.face_fn(f2).edge().vertex().index);

    assert_eq!(mesh.face_fn(f2).edge().vertex().index, mesh.face_fn(f3).edge().next().vertex().index);
    assert_eq!(mesh.face_fn(f3).edge().vertex().index, mesh.face_fn(f4).edge().next().vertex().index);

    assert_eq!(mesh.face_fn(f2).edge().prev().vertex().index, v3);
    assert_eq!(mesh.face_fn(f3).edge().prev().vertex().index, v3);
    assert_eq!(mesh.face_fn(f4).edge().prev().vertex().index, v3);
}

#[test]
fn can_iterate_edges_around_vertex() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let v3 = mesh.add(Vertex::default());

    let f1 = mesh.add(triangle::FromVerts(v0, v1, v3));
    let twin_a = mesh.face_fn(f1).edge().next().twin().index;
    let _f2 = mesh.add(triangle::FromOneEdge(twin_a, v2));

    print_mesh(&mesh);

    let vert = {
        let vindex = mesh.face_fn(f1).edge().prev().vertex().index;
        &mesh.vertex(vindex)
    };
    let mut edges_enumerated = 0;
    for eindex in mesh.edges_around_vertex(vert) {
        debug!("{:?}", eindex);
        assert! {
            (eindex == EdgeIndex::new(3)) || (eindex == EdgeIndex::new(4)) || (eindex == EdgeIndex::new(10))
        };
        edges_enumerated += 1;
    }
    // FIXME: This is just proving that a vertex connected to boundary edges won't work.
    assert_eq!(edges_enumerated, 1);
}

#[test]
fn can_remove_faces() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let v3 = mesh.add(Vertex::default());

    let f0 = mesh.add(triangle::FromVerts(v0, v1, v3));
    let f0e1 = mesh.face_fn(f0).edge().next().twin().index;
    let f1 = mesh.add(triangle::FromOneEdge(f0e1, v2));
    let f1e0 = mesh.face(f1).edge_index;

    print_mesh(&mesh);

    assert_eq!(mesh.is_boundary_edge(f0e1), false);
    assert_eq!(mesh.is_boundary_edge(f1e0), false);

    mesh.remove(f1);

    assert!(mesh.is_boundary_edge(f0e1));
    assert!(mesh.is_boundary_edge(f1e0));

    let f1 = mesh.add(triangle::FromOneEdge(f0e1, v2));
    let f1e0 = mesh.face(f1).edge_index;

    assert_eq!(mesh.is_boundary_edge(f0e1), false);
    assert_eq!(mesh.is_boundary_edge(f1e0), false);

    mesh.remove(f0);

    assert!(mesh.is_boundary_edge(f0e1));
    assert!(mesh.is_boundary_edge(f1e0));
}

#[test]
fn can_remove_vertices() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());

    let f0 = mesh.add(triangle::FromVerts(v0, v1, v2));

    mesh.remove(v2);

    assert_eq!(mesh.vertex(v2).is_valid(), false);

    assert_eq!(mesh.face_fn(f0).edge().prev().is_valid(), false);
    assert_eq!(mesh.face_fn(f0).edge().next().twin().is_valid(), false);
}

#[test]
fn can_remove_edge() {
    let _ = env_logger::init();
    let mut mesh = TestMesh::new();

    let v0 = mesh.add(Vertex::default());
    let v1 = mesh.add(Vertex::default());
    let v2 = mesh.add(Vertex::default());
    let v3 = mesh.add(Vertex::default());

    let f0 = mesh.add(triangle::FromVerts(v0, v1, v3));

    let f0e0 = mesh.face(f0).edge_index;
    let f0e1 = mesh.face_fn(f0).edge().next().index;
    let _f0e2 = mesh.face_fn(f0).edge().prev().index;


    let f1 = {
        let ei = mesh.edge(f0e1).twin_index;
        mesh.add(triangle::FromOneEdge(ei, v2))
    };

    assert_eq!(mesh.num_edges(), 10);
    mesh.remove(f0e0);
    assert_eq!(mesh.num_edges(), 8);

    assert_eq!(mesh.face(f0).edge_index, f0e1);

    assert_eq!(mesh.face_fn(f1).edge().index, EdgeIndex::new(4));
    assert_eq!(mesh.face_fn(f1).edge().next().index, EdgeIndex::new(7));
    assert_eq!(mesh.face_fn(f1).edge().prev().index, EdgeIndex::new(2));
}
