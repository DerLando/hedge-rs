//! Iterators for simple or common mesh traversal patterns.

use std::slice::Iter;
use std::iter::Enumerate;
use log::*;
use super::*;

pub struct ElementEnumerator<'mesh, E> {
    tag: Tag,
    iter: Enumerate<Iter<'mesh, E>>,
}

impl<'mesh, E: IsActive + Taggable + Storable> ElementEnumerator<'mesh, E> {
    pub fn new(tag: Tag, iter: Enumerate<Iter<'mesh, E>>) -> ElementEnumerator<'mesh, E> {
        debug!("New element enumerator");
        ElementEnumerator { tag, iter }
    }

    pub fn next_element(&mut self) -> Option<(Index<E>, &'mesh E)> {
        for (offset, element) in self.iter.by_ref() {
            let is_next = element.is_active() && element.tag() != self.tag;
            if is_next {
                element.set_tag(self.tag);
                let index = Index::with_generation(offset, element.generation());
                return Some((index, element));
            }
        }
        debug!("Element enumeration completed.");
        return None;
    }
}

pub type VertexEnumerator<'mesh> = ElementEnumerator<'mesh, Vertex>;
pub type FaceEnumerator<'mesh> = ElementEnumerator<'mesh, Face>;
pub type EdgeEnumerator<'mesh> = ElementEnumerator<'mesh, Edge>;
pub type PointEnumerator<'mesh> = ElementEnumerator<'mesh, Point>;

#[cfg(test)]
mod tests {
    use crate::*;

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
    fn can_iterate_edges_around_vertex() {
        unimplemented!();
    }
}
