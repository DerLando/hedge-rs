//! Iterators for simple or common mesh traversal patterns.

use std::slice::Iter;
use std::iter::Enumerate;
use log::*;
use super::*;

pub enum CirculatorMode {
    Face,
    Edge,
    Vertex,
}

pub struct Circulator;

pub struct FaceEdges<'mesh> {
    tag: Tag,
    root_edge: EdgeFn<'mesh>,
    last_edge: Option<EdgeFn<'mesh>>,
}

impl<'mesh> FaceEdges<'mesh> {
    pub fn new(tag: Tag, face: FaceFn<'mesh>) -> Self {
        FaceEdges {
            tag,
            root_edge: face.edge(),
            last_edge: None
        }
    }
}

impl<'mesh> Iterator for FaceEdges<'mesh> {
    type Item = EdgeFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_edge = if let Some(last_edge) = self.last_edge {
            let next_edge = last_edge.next();
            next_edge.element()
                .and_then(|edge| {
                    if edge.tag() == self.tag {
                        None
                    } else {
                        edge.set_tag(self.tag);
                        Some(next_edge)
                    }
                })
                .and_then(|next_edge| {
                    if next_edge.index == self.root_edge.index {
                        None
                    } else {
                        Some(next_edge)
                    }
                })
        } else {
            Some(self.root_edge)
        };
        self.last_edge
    }
}

pub struct FaceVertices<'mesh> {
    inner_iter: FaceEdges<'mesh>,
}

impl<'mesh> FaceVertices<'mesh> {
    pub fn new(tag: Tag, face: FaceFn<'mesh>) -> Self {
        let inner_iter = FaceEdges {
            tag,
            root_edge: face.edge(),
            last_edge: None
        };
        FaceVertices { inner_iter }
    }
}

impl<'mesh> Iterator for FaceVertices<'mesh> {
    type Item = VertexFn<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|edge| edge.vertex())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_iterate_over_edges_of_face() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();

        let p0 = mesh.add_element(Point::new(-1.0, 0.0, 0.0));
        let p1 = mesh.add_element(Point::new(1.0, 0.0, 0.0));
        let p2 = mesh.add_element(Point::new(0.0, 1.0, 0.0));

        let v0 = mesh.add_element(Vertex::at_point(p0));
        let v1 = mesh.add_element(Vertex::at_point(p1));
        let v2 = mesh.add_element(Vertex::at_point(p2));

        let e0 = utils::build_full_edge(&mut mesh, v0, v1);
        let e1 = utils::build_full_edge_from(&mut mesh, e0, v2);
        let e2 = utils::close_edge_loop(&mut mesh, e1, e0);

        let f0 = mesh.add_element(Face::default());
        utils::assign_face_to_loop(&mesh, e0, f0);

        let mut iter_count = 0;
        for edge in mesh.face(f0).edges() {
            assert!(iter_count < 3);
            if edge.index == e0 {
                iter_count += 1;
            } else if edge.index == e1 {
                iter_count += 1;
            } else if edge.index == e2 {
                iter_count += 1;
            } else {
                unreachable!();
            }
        }
        assert_eq!(iter_count, 3);
    }

    #[test]
    fn can_iterate_over_vertices_of_face() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::new();

        let p0 = mesh.add_element(Point::new(-1.0, 0.0, 0.0));
        let p1 = mesh.add_element(Point::new(1.0, 0.0, 0.0));
        let p2 = mesh.add_element(Point::new(0.0, 1.0, 0.0));

        let v0 = mesh.add_element(Vertex::at_point(p0));
        let v1 = mesh.add_element(Vertex::at_point(p1));
        let v2 = mesh.add_element(Vertex::at_point(p2));

        let e0 = utils::build_full_edge(&mut mesh, v0, v1);
        let e1 = utils::build_full_edge_from(&mut mesh, e0, v2);
        let e2 = utils::close_edge_loop(&mut mesh, e1, e0);

        let f0 = mesh.add_element(Face::default());
        utils::assign_face_to_loop(&mesh, e0, f0);

        let mut iter_count = 0;
        for vert in mesh.face(f0).vertices() {
            assert!(iter_count < 3);
            if vert.index == v0 {
                iter_count += 1;
            } else if vert.index == v1 {
                iter_count += 1;
            } else if vert.index == v2 {
                iter_count += 1;
            } else {
                unreachable!();
            }
        }
        assert_eq!(iter_count, 3);
    }

    #[test]
    fn can_iterate_edges_around_vertex() {
        unimplemented!();
    }

    #[test]
    fn can_iterate_over_connected_edges() {
        unimplemented!();
    }
}
