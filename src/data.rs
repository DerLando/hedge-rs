

pub type Tag = u32;
pub type Offset = u32;
pub type Generation = u32;
pub type Position = [f32; 3];
pub type Normal = [f32; 3];

/// Whether or not a cell is current or 'removed'
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum ElementStatus {
    ACTIVE,
    INACTIVE,
}
