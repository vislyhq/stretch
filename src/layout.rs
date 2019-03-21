#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::geometry::{Point, Size};
use decorum::R32;

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) order: u32,
    pub size: Size<R32>,
    pub location: Point<R32>,
    pub children: Vec<Node>,
}
