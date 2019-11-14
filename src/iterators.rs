//! Iterators for simple or common mesh traversal patterns.

use super::*;

#[derive(Debug)]
pub struct VertexCirculator<'mesh> {
    tag: Tag,
    vert: VertexProxy<'mesh>,
    last_edge: Option<HalfEdgeProxy<'mesh>>,
    central_point: PointHandle,
}

impl<'mesh> VertexCirculator<'mesh> {
    pub fn new(tag: Tag, vert: VertexProxy<'mesh>) -> Self {
        assert!(vert.is_valid());
        let central_point = vert.point().handle;
        dbg!(vert.handle.index(), central_point.index());
        VertexCirculator {
            tag,
            vert,
            last_edge: None,
            central_point,
        }
    }
}

impl<'mesh> Iterator for VertexCirculator<'mesh> {
    type Item = HalfEdgeProxy<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(last_edge) = self.last_edge {
            self.last_edge = Some(last_edge.prev().adjacent());
        } else {
            self.last_edge = Some(self.vert.edge());
        }
        if let Some(element) = self.last_edge.and_then(|e| e.element()) {
            if element.tag() == self.tag {
                return None;
            } else {
                element.set_tag(self.tag);
            }
        }
        self.last_edge
    }
}

#[derive(Debug)]
pub struct VertexFaceCirculator<'mesh> {
    inner_iter: VertexCirculator<'mesh>,
}

#[derive(Debug)]
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
            last_edge: None,
        }
    }
}

impl<'mesh> Iterator for FaceEdges<'mesh> {
    type Item = HalfEdgeProxy<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.last_edge = if let Some(last_edge) = self.last_edge {
            let next_edge = last_edge.next();
            next_edge
                .element()
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

#[derive(Debug)]
pub struct FaceVertices<'mesh> {
    inner_iter: FaceEdges<'mesh>,
}

impl<'mesh> FaceVertices<'mesh> {
    pub fn new(face: FaceProxy<'mesh>) -> Self {
        FaceVertices {
            inner_iter: face.edges(),
        }
    }
}

impl<'mesh> Iterator for FaceVertices<'mesh> {
    type Item = VertexProxy<'mesh>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|edge| edge.vertex())
    }
}

#[derive(Debug)]
pub struct FaceNeighbors<'mesh> {
    inner_iter: FaceEdges<'mesh>
}

impl<'mesh> FaceNeighbors<'mesh> {
    pub fn new(face: FaceProxy<'mesh>) -> Self {
        FaceNeighbors {
            inner_iter: face.edges(),
        }
    }
}

impl<'mesh> Iterator for FaceNeighbors<'mesh> {
    type Item = FaceProxy<'mesh>;

    // TOOD: boundary edges should be skipped right?
    fn next(&mut self) -> Option<Self::Item> {
        self.inner_iter.next().map(|edge| edge.adjacent().face())
    }
}

pub struct FaceTriangles<'mesh> {
    vertices: Vec<VertexProxy<'mesh>>,
}

impl<'mesh> Iterator for FaceTriangles<'mesh> {
    type Item = (
        &'mesh VertexProxy<'mesh>,
        &'mesh VertexProxy<'mesh>,
        &'mesh VertexProxy<'mesh>
    );

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'mesh> FaceTriangles<'mesh> {
    pub fn new(face: FaceProxy<'mesh>) -> Self {
        FaceTriangles {
            vertices: face.vertices().collect(),
        }
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

    fn build_fan(points: [PointHandle; 5], mesh: &mut Mesh) -> FaceHandle {
        let f0 = mesh.add_face([points[0], points[1], points[4]].as_ref());
        assert!(f0.is_valid());
        assert_eq!(mesh.face(f0).vertices().count(), 3);

        let leading_edge = mesh.face(f0).root_edge().next().adjacent().handle;
        let f1 = mesh.add_face((leading_edge, [points[2]].as_ref()));
        assert!(f1.is_valid());
        assert_eq!(mesh.face(f1).vertices().count(), 3);

        let leading_edge = mesh.face(f1).root_edge().prev().adjacent().handle;
        let f2 = mesh.add_face((leading_edge, [points[3]].as_ref()));
        assert!(f2.is_valid());
        assert_eq!(mesh.face(f2).vertices().count(), 3);

        let leading_edge = mesh.face(f2).root_edge().prev().adjacent().handle;
        let closing_edge = mesh.face(f0).root_edge().prev().adjacent().handle;
        let f3 = mesh.add_face((leading_edge, closing_edge));
        assert!(f3.is_valid());
        assert_eq!(mesh.face(f3).vertices().count(), 3);

        f0
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

        let f0 = build_fan(points, &mut mesh);

        let root_vert = {
            let e0 = mesh.face(f0).root_edge();
            let e1 = e0.next();
            let e2 = e1.next();
            let vert = e2.vertex();
            assert_eq!(vert.data().map(|v| v.point.index()), Some(5));
            vert
        };
        dbg!(&mesh);
        assert_eq!(root_vert.edges().count(), 4);
    }
}
