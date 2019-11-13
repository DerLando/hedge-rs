//! Iterators for simple or common mesh traversal patterns.

use log::*;
use super::*;


pub struct VertexCirculator<'mesh> {
    tag: Tag,
    vert: VertexProxy<'mesh>,
    last_edge: Option<HalfEdgeProxy<'mesh>>,
    central_point: PointHandle,
}

impl<'mesh> VertexCirculator<'mesh> {
    pub fn new(tag: Tag, vert: VertexProxy<'mesh>) -> Self {
        VertexCirculator {
            tag,
            vert,
            last_edge: None,
            central_point: vert.data()
                .map(|d| d.point)
                .unwrap_or_else(Default::default)
        }
    }
}

impl<'mesh> Iterator for VertexCirculator<'mesh> {
    type Item = HalfEdgeProxy<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_edge = if let Some(last_edge) = self.last_edge {
            let next_edge = last_edge.prev().adjacent();
            next_edge.element().and_then(|e| {
                if e.tag() == self.tag {
                    debug!("Encountered previously tagged edge.");
                    None
                } else {
                    e.set_tag(self.tag);
                    Some(next_edge)
                }
            }).and_then(|next_edge| {
                if next_edge.is_boundary() {
                    warn!("Vertex circulator terminated due to boundary edge.");
                    None
                } else if let Some(phnd) = next_edge.vertex().data().map(|d| d.point) {
                    if phnd == self.central_point {
                        Some(next_edge)
                    } else {
                        debug!("Ending iteration because vertex attributes do not match.");
                        None
                    }
                } else {
                    None
                }
            })
        } else {
            let edge = self.vert.edge();
            edge.element().and_then(|e| {
                e.set_tag(self.tag);
                Some(edge)
            })
        };
        self.last_edge
    }
}

pub struct FaceEdges<'mesh> {
    tag: Tag,
    root_edge: HalfEdgeProxy<'mesh>,
    last_edge: Option<HalfEdgeProxy<'mesh>>,
}

impl<'mesh> FaceEdges<'mesh> {
    pub fn new(tag: Tag, face: FaceProxy<'mesh>) -> Self {
        FaceEdges {
            tag,
            root_edge: face.root_edge(),
            last_edge: None
        }
    }
}

impl<'mesh> Iterator for FaceEdges<'mesh> {
    type Item = HalfEdgeProxy<'mesh>;

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
                    if next_edge.handle == self.root_edge.handle {
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
    pub fn new(tag: Tag, face: FaceProxy<'mesh>) -> Self {
        let inner_iter = FaceEdges {
            tag,
            root_edge: face.root_edge(),
            last_edge: None
        };
        FaceVertices { inner_iter }
    }
}

impl<'mesh> Iterator for FaceVertices<'mesh> {
    type Item = VertexProxy<'mesh>;

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
        let mut mesh = Mesh::default();

        let p0 = mesh.add(Point::from_position(-1.0, 0.0, 0.0));
        let p1 = mesh.add(Point::from_position(1.0, 0.0, 0.0));
        let p2 = mesh.add(Point::from_position(0.0, 1.0, 0.0));

        let f0 = mesh.add_face([p0, p1, p2].as_ref());
        assert!(f0.is_valid());

        // let v0 = mesh.add(Vertex::at_point(p0));
        // let v1 = mesh.add(Vertex::at_point(p1));
        // let v2 = mesh.add(Vertex::at_point(p2));

        // let e0 = utils::build_full_edge(&mut mesh, v0, v1);
        // let e1 = utils::build_full_edge_from(&mut mesh, e0, v2);
        // let e2 = utils::close_edge_loop(&mut mesh, e1, e0);

        // let f0 = mesh.add(Face::default());
        // utils::assign_face_to_loop(&mesh, e0, f0);

        let edges: Vec<HalfEdgeProxy> = mesh.face(f0).edges().collect();
        assert_eq!(edges.len(), 3);

        let mut iter_count = 0;
        for edge in mesh.face(f0).edges() {
            assert!(iter_count < 3);
            match iter_count {
                0 => assert_eq!(edge.vertex().data().map(|d| d.point), Some(p0)),
                1 => assert_eq!(edge.vertex().data().map(|d| d.point), Some(p1)),
                2 => assert_eq!(edge.vertex().data().map(|d| d.point), Some(p2)),
                _ => unreachable!(),
            }
            iter_count += 1;
        }
        assert_eq!(iter_count, 3);
    }

    #[test]
    fn can_iterate_over_vertices_of_face() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        let p0 = mesh.add(Point::from_position(-1.0, 0.0, 0.0));
        let p1 = mesh.add(Point::from_position(1.0, 0.0, 0.0));
        let p2 = mesh.add(Point::from_position(0.0, 1.0, 0.0));

        let f0 = mesh.add_face([p0, p1, p2].as_ref());
        assert!(f0.is_valid());

        // let v0 = mesh.add(Vertex::at_point(p0));
        // let v1 = mesh.add(Vertex::at_point(p1));
        // let v2 = mesh.add(Vertex::at_point(p2));

        // let e0 = utils::build_full_edge(&mut mesh, v0, v1);
        // let e1 = utils::build_full_edge_from(&mut mesh, e0, v2);
        // let _e2 = utils::close_edge_loop(&mut mesh, e1, e0);

        // let f0 = mesh.add(Face::default());
        // utils::assign_face_to_loop(&mesh, e0, f0);

        let mut iter_count = 0;
        for vert in mesh.face(f0).vertices() {
            assert!(vert.is_valid());
            assert!(iter_count < 3);
            match iter_count {
                0 => assert_eq!(vert.data().map(|d| d.point), Some(p0)),
                1 => assert_eq!(vert.data().map(|d| d.point), Some(p1)),
                2 => assert_eq!(vert.data().map(|d| d.point), Some(p2)),
                _ => unreachable!(),
            }
            iter_count += 1;
        }
        assert_eq!(iter_count, 3);
    }

    fn build_fan(points: [PointHandle; 5], mesh: &mut Mesh) -> VertexHandle {
        let f0 = mesh.add_face([points[0], points[1], points[4]].as_ref());
        assert!(f0.is_valid());

        let v2 = mesh
            .face(f0)
            .root_edge()
            .next()
            .next()
            .vertex()
            .handle;

        // let v0 = mesh.add(Vertex::at_point(points[0]));
        // let v1 = mesh.add(Vertex::at_point(points[1]));
        // let v2 = mesh.add(Vertex::at_point(points[4]));

        // let e0 = utils::build_full_edge(mesh, v0, v1);
        // let e1 = utils::build_full_edge_from(mesh, e0, v2);
        // let e2 = utils::close_edge_loop(mesh, e1, e0);

        // let f0 = mesh.add(Face::default());
        // utils::assign_face_to_loop(mesh, e0, f0);

        /////////////////////////////////

        let f1 = mesh.add_face([points[1], points[2], points[4]].as_ref());
        assert!(f1.is_valid());

        // let v3 = mesh.add(Vertex::at_point(points[1]));
        // let _v4 = mesh.add(Vertex::at_point(points[2]));
        // let v5 = mesh.add(Vertex::at_point(points[4]));

        // let e3 = mesh.edge(e1).adjacent().handle;
        // utils::assoc_vert_edge(mesh, v5, e3);
        // let e4 = utils::build_full_edge_from(mesh, e3, v3);
        // let e5 = utils::close_edge_loop(mesh, e4, e3);

        // let f1 = mesh.add(Face::default());
        // utils::assign_face_to_loop(mesh, e3, f1);

        /////////////////////////////////

        let f2 = mesh.add_face([points[2], points[3], points[4]].as_ref());
        assert!(f2.is_valid());

        // let v6 = mesh.add(Vertex::at_point(points[2]));
        // let _v7 = mesh.add(Vertex::at_point(points[3]));
        // let v8 = mesh.add(Vertex::at_point(points[4]));

        // let e6 = mesh.edge(e5).adjacent().handle;
        // utils::assoc_vert_edge(mesh, v8, e6);
        // let e7 = utils::build_full_edge_from(mesh, e6, v6);
        // let e8 = utils::close_edge_loop(mesh, e7, e6);

        // let f2 = mesh.add(Face::default());
        // utils::assign_face_to_loop(mesh, e6, f2);

        /////////////////////////////////

        let f3 = mesh.add_face([points[3], points[0], points[4]].as_ref());
        assert!(f3.is_valid());

        // let _v9  = mesh.add(Vertex::at_point(points[3]));
        // let _v10 = mesh.add(Vertex::at_point(points[0]));
        // let v11 = mesh.add(Vertex::at_point(points[4]));

        // let e9 = mesh.edge(e8).adjacent().handle;
        // utils::assoc_vert_edge(mesh, v11, e9);
        // let e11 = mesh.edge(e2).adjacent().handle;
        // utils::assoc_vert_edge(mesh, v0, e11);
        // let _e10 = utils::close_edge_loop(mesh, e9, e11);

        // let f3 = mesh.add(Face::default());
        // utils::assign_face_to_loop(mesh, e9, f3);

        return v2;
    }

    #[test]
    fn can_iterate_around_vertex() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        let points = [
            mesh.add(Point::from_position(-1.0, 0.0, 0.0)),
            mesh.add(Point::from_position(0.0, -1.0, 0.0)),
            mesh.add(Point::from_position(1.0, 0.0, 0.0)),
            mesh.add(Point::from_position(0.0, 1.0, 0.0)),
            mesh.add(Point::from_position(0.0, 0.0, 0.0)),
        ];

        let root_vert = build_fan(points, &mut mesh);

        let mut iter_count = 0;
        for _edge in mesh.vertex(root_vert).edges() {
            assert!(iter_count < 4);
            iter_count += 1;
        }
        assert_eq!(iter_count, 4);
    }
}
