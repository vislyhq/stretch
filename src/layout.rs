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

impl Node {
    pub(crate) unsafe fn to_layout_node(node: *const Node) -> Box<LayoutNode> {
        let children = Array {
            pointer: (*node).children.as_ptr() as *const c_void,
            length: (*node).children.len(),
            capacity: (*node).children.capacity(),
        };

        let layout = LayoutNode {
            order: (*node).order,
            size: (*node).size,
            location: (*node).location,
            children: children,
        };

        Box::new(layout)
    }
}
