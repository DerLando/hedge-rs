use nalgebra as na;

/// Handles with this generation value will only have their index considered.
pub const IGNORED_GENERATION: Generation = 0;

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: Index = std::u32::MAX;

pub type Index = u32;
pub type Generation = u32;
pub type Position = na::Point3<f32>;
pub type Normal = na::Vector3<f32>;
pub type Color = na::Vector4<f32>;

impl Default for Generation {
    fn default() -> Self {
        IGNORED_GENERATION
    }
}

impl Default for Index {
    fn default() -> Self {
        INVALID_COMPONENT_INDEX
    }
}

impl Default for Position {
    fn default() -> Self {
        na::Point3::new(0.0, 0.0, 0.0)
    }
}

impl Default for Normal {
    fn default() -> Self {
        na::Vector3::new(0.0, 0.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        na::Vector4::new(1.0, 1.0, 1.0, 1.0)
    }
}
