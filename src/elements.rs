
use std::cell::{Cell, RefCell, Ref, RefMut};
use crate::handles::{
    HalfEdgeHandle, FaceHandle,
    VertexHandle, PointHandle
};
use crate::data::{
    Generation, Tag, Position, ElementStatus,
    HalfEdgeData, FaceData, VertexData, PointData,
};
use crate::traits::{
    ElementData, Element,
    Storable, Taggable, IsValid
};

/// Trait for accessing Mesh element properties.
#[derive(Debug, Clone)]
pub struct MeshElement<D> {
    tag: Cell<Tag>,
    generation: Cell<Generation>,
    status: Cell<ElementStatus>,
    data: RefCell<D>,
}

impl<D: ElementData> Default for MeshElement<D> {
    fn default() -> Self {
        MeshElement {
            tag: Cell::new(0),
            generation: Cell::new(1),
            status: Cell::new(ElementStatus::INACTIVE),
            data: Default::default(),
        }
    }
}

impl<D: ElementData> Taggable for MeshElement<D> {
    fn tag(&self) -> Tag {
        self.tag.get()
    }

    fn set_tag(&self, tag: Tag) {
        self.tag.set(tag);
    }
}

impl<D: ElementData> Storable for MeshElement<D> {
    fn generation(&self) -> Generation {
        self.generation.get()
    }

    fn set_generation(&self, generation: Generation) {
        self.generation.set(generation);
    }

    fn status(&self) -> ElementStatus {
        self.status.get()
    }

    fn set_status(&self, status: ElementStatus) {
        self.status.set(status);
    }
}

////////////////////////////////////////////////////////////////

pub type HalfEdge = MeshElement<HalfEdgeData>;

impl HalfEdge {
    /// Returns true when this edge has a previous and next edge.
    pub fn is_connected(&self) -> bool {
        let data = self.data();
        data.next.is_valid() && data.prev.is_valid()
    }
}

impl Element for HalfEdge {
    type Data = HalfEdgeData;
    type Handle = HalfEdgeHandle;

    fn with_data(data: Self::Data) -> Self {
        HalfEdge {
            data: RefCell::new(data),
            ..Default::default()
        }
    }

    fn data(&self) -> Ref<Self::Data> {
        self.data.borrow()
    }

    fn data_mut(&self) -> RefMut<Self::Data> {
        self.data.borrow_mut()
    }
}

impl IsValid for HalfEdge {
    /// An Edge is valid when it has a valid twin index, a valid vertex index
    /// and `is_connected`
    fn is_valid(&self) -> bool {
        let data = self.data();
        self.is_active() &&
            data.vertex.is_valid() &&
            data.adjacent.is_valid() &&
            data.next.is_valid() &&
            data.prev.is_valid()
    }
}

////////////////////////////////////////////////////////////////

pub type Vertex = MeshElement<VertexData>;
impl Vertex {
    pub fn new(edge: HalfEdgeHandle, point: PointHandle) -> Self {
        Vertex::with_data(VertexData { edge, point })
    }

    pub fn for_edge(edge: HalfEdgeHandle) -> Self {
        Vertex::with_data(VertexData {
            edge,
            ..VertexData::default()
        })
    }

    pub fn at_point(point: PointHandle) -> Self {
        Vertex::with_data(VertexData {
            point,
            ..VertexData::default()
        })
    }
}

impl IsValid for Vertex {
    /// A vertex is considered "valid" as long as it has a valid edge index.
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().edge.is_valid()
    }
}

impl Element for Vertex {
    type Data = VertexData;
    type Handle = VertexHandle;

    fn with_data(data: Self::Data) -> Self {
        Vertex {
            data: RefCell::new(data),
            ..Default::default()
        }
    }

    fn data(&self) -> Ref<Self::Data> {
        self.data.borrow()
    }

    fn data_mut(&self) -> RefMut<Self::Data> {
        self.data.borrow_mut()
    }
}

////////////////////////////////////////////////////////////////

pub type Face = MeshElement<FaceData>;
impl Face {
    pub fn new(root_edge: HalfEdgeHandle) -> Self {
        Face::with_data(FaceData { root_edge })
    }
}

impl IsValid for Face {
    /// A face is considered "valid" as long as it has an edge index
    /// other than `INVALID_COMPONENT_INDEX`
    fn is_valid(&self) -> bool {
        self.is_active() && self.data().root_edge.is_valid()
    }
}

impl Element for Face {
    type Data = FaceData;
    type Handle = FaceHandle;

    fn with_data(data: Self::Data) -> Self {
        Face {
            data: RefCell::new(data),
            ..Default::default()
        }
    }

    fn data(&self) -> Ref<Self::Data> {
        self.data.borrow()
    }

    fn data_mut(&self) -> RefMut<Self::Data> {
        self.data.borrow_mut()
    }
}

////////////////////////////////////////////////////////////////

pub type Point = MeshElement<PointData>;
impl Point {
    pub fn new(position: Position) -> Self {
        Point::with_data(PointData::new(position))
    }

    pub fn from_coords(x: f32, y: f32, z: f32) -> Self {
        Point::with_data(PointData::new([x, y, z]))
    }

    pub fn from_slice(offset: usize, values: &[f32]) -> Self {
        assert!(values.len() >= (offset + 3));
        Point::with_data(PointData::new([
            values[offset],
            values[offset+1],
            values[offset+2],
        ]))
    }
}

impl IsValid for Point {
    fn is_valid(&self) -> bool {
        self.is_active()
    }
}

impl Element for Point {
    type Data = PointData;
    type Handle = PointHandle;

    fn with_data(data: Self::Data) -> Self {
        Point {
            data: RefCell::new(data),
            ..Default::default()
        }
    }

    fn data(&self) -> Ref<Self::Data> {
        self.data.borrow()
    }

    fn data_mut(&self) -> RefMut<Self::Data> {
        self.data.borrow_mut()
    }
}
