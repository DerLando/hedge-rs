use ultraviolet as uv;

/// Handles with this generation value will only have their index considered.
pub const IGNORED_GENERATION: Generation = 0;

/// Our default value for uninitialized or unconnected components in the mesh.
pub const INVALID_COMPONENT_INDEX: Index = std::u32::MAX;

pub type Generation = u32;
pub type Index = u32;

pub type Position = uv::Vec3;
pub type Normal = uv::Vec3;
pub type Color = uv::Vec4;
pub type TexCoord = uv::Vec2;

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
        uv::Vec3::new(0.0, 0.0, 0.0)
    }
}

impl Default for Normal {
    fn default() -> Self {
        uv::Vec3::new(0.0, 0.0, 1.0)
    }
}

impl Default for Color {
    fn default() -> Self {
        uv::Vec4::new(1.0, 1.0, 1.0, 1.0)
    }
}
