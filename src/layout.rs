use std::ffi::c_void;
use crate::array::Array;
use crate::geometry::{Point, Size};

#[repr(C)]
#[derive(Debug)]
pub struct LayoutNode {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Array<c_void>,
}

#[derive(Debug)]
pub struct Node {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Node>,
}
