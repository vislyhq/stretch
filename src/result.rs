use crate::algo::ComputeResult;
use crate::geometry::{Point, Size};
use crate::number::Number;

#[derive(Copy, Debug, Clone)]
pub struct Layout {
    pub(crate) order: u32,
    pub size: Size<f32>,
    pub location: Point<f32>,
}

impl Layout {
    pub(crate) fn new() -> Self {
        Self { order: 0, size: Size::zero(), location: Point::zero() }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Cache {
    pub(crate) node_size: Size<Number>,
    pub(crate) parent_size: Size<Number>,
    pub(crate) perform_layout: bool,

    pub(crate) result: ComputeResult,
}
