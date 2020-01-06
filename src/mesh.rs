use std::fmt;
use std::sync::atomic;

use crate::data::{Tag, Handle, Point, Position};
use crate::kernel::Kernel;

pub struct Mesh {
    pub kernel: Kernel,
    tag: atomic::AtomicU32,
}

impl fmt::Debug for Mesh {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Mesh {{ {} points, {} faces }}",
            self.kernel.point_count(),
            self.kernel.face_count(),
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

    pub fn calculate_normals(&self) {
        unimplemented!()
    }

    pub fn add_points(&mut self, points: &[Point]) {
        self.kernel.add_points(points);
        for point in points.into_iter() {
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        traits::IsValid,
        data::{Handle, Point, Position},
        Mesh,
    };

    #[test]
    fn handles_are_invalid_by_default() {
        let hnd = Handle::default();
        assert_eq!(hnd.is_valid(), false);
    }

    #[test]
    fn can_add_and_remove_faces() {
        unimplemented!()
    }

    #[test]
    fn can_add_and_remove_points() {
        let _ = env_logger::try_init();
        let mut mesh = Mesh::default();

        mesh.add_points([
            Point::from(Position::new(0.0, 0.0, 0.0)),
            Point::from(Position::new(1.0, 0.0, 0.0)),
            Point::from(Position::new(1.0, 1.0, 0.0)),
            Point::from(Position::new(0.0, 1.0, 0.0)),
        ].as_ref());

        assert_eq!(mesh.kernel.point_count(), 4);

        unimplemented!()
    }

    #[test]
    fn can_build_a_simple_mesh() {
        unimplemented!()
    }

    #[test]
    fn can_iterate_over_faces() {
        unimplemented!()
    }

    #[test]
    fn can_iterate_over_vertices() {
        unimplemented!()
    }

    #[test]
    fn can_build_triangle_list() {
        unimplemented!()
    }
}
