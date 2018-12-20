use crate::geometry::{Point, Size};

#[repr(C)]
#[derive(Debug)]
pub struct LayoutNode {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Box<Vec<LayoutNode>>,
}

pub type Node = LayoutNode;
