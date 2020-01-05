use std::fmt;
use std::sync::atomic;

use crate::data::{Tag, Handle};
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
}

#[cfg(test)]
mod tests {
    use crate::{
        traits::IsValid,
        data::{Handle, Point},
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
