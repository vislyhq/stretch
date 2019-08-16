#![feature(in_band_lifetimes)]
#![feature(dec2flt)]

use std::f32;

extern crate dict_derive;
use dict_derive::{FromPyObject, IntoPyObject};

extern crate pyo3;
use pyo3::prelude::*;
use pyo3::{wrap_pyfunction, wrap_pymodule};
use pyo3::exceptions;

extern crate stretch;
use stretch::geometry::*;
use stretch::node::*;
use stretch::number::*;
use stretch::style::*;


/* ======================================================================== */
/* UTIL                                                                     */
/* ======================================================================== */


trait FromIndex<T> {
    fn from_index(index: i32) -> PyResult<T>;
}


/* ======================================================================== */
/* ENUM CONVERSION - based on style.rs                                      */
/* ======================================================================== */


impl FromIndex<AlignItems> for AlignItems {
    fn from_index(index: i32) -> PyResult<AlignItems> {
        match index {
            0 => Ok(AlignItems::FlexStart),
            1 => Ok(AlignItems::FlexEnd),
            2 => Ok(AlignItems::Center),
            3 => Ok(AlignItems::Baseline),
            4 => Ok(AlignItems::Stretch),
            n => Err(exceptions::ValueError::py_err(format!("enum AlignItems - invalid index: {}", n))),
        }
    }
}

impl FromIndex<AlignSelf> for AlignSelf {
    fn from_index(index: i32) -> PyResult<AlignSelf> {
        match index {
            0 => Ok(AlignSelf::Auto),
            1 => Ok(AlignSelf::FlexStart),
            2 => Ok(AlignSelf::FlexEnd),
            3 => Ok(AlignSelf::Center),
            4 => Ok(AlignSelf::Baseline),
            5 => Ok(AlignSelf::Stretch),
            n => Err(exceptions::ValueError::py_err(format!("enum AlignSelf - invalid index: {}", n))),
        }
    }
}

impl FromIndex<AlignContent> for AlignContent {
    fn from_index(index: i32) -> PyResult<AlignContent> {
        match index {
            0 => Ok(AlignContent::FlexStart),
            1 => Ok(AlignContent::FlexEnd),
            2 => Ok(AlignContent::Center),
            3 => Ok(AlignContent::Stretch),
            4 => Ok(AlignContent::SpaceBetween),
            5 => Ok(AlignContent::SpaceAround),
            n => Err(exceptions::ValueError::py_err(format!("enum AlignContent - invalid index: {}", n))),
        }
    }
}

impl FromIndex<Direction> for Direction {
    fn from_index(index: i32) -> PyResult<Direction> {
        match index {
            0 => Ok(Direction::Inherit),
            1 => Ok(Direction::LTR),
            2 => Ok(Direction::RTL),
            n => Err(exceptions::ValueError::py_err(format!("enum Direction - invalid index: {}", n))),
        }
    }
}

impl FromIndex<Display> for Display {
    fn from_index(index: i32) -> PyResult<Display> {
        match index {
            0 => Ok(Display::Flex),
            1 => Ok(Display::None),
            n => Err(exceptions::ValueError::py_err(format!("enum Display - invalid index: {}", n))),
        }
    }
}

impl FromIndex<FlexDirection> for FlexDirection {
    fn from_index(index: i32) -> PyResult<FlexDirection> {
        match index {
            0 => Ok(FlexDirection::Row),
            1 => Ok(FlexDirection::Column),
            2 => Ok(FlexDirection::RowReverse),
            3 => Ok(FlexDirection::ColumnReverse),
            n => Err(exceptions::ValueError::py_err(format!("enum FlexDirection - invalid index: {}", n))),
        }
    }
}

impl FromIndex<JustifyContent> for JustifyContent {
    fn from_index(index: i32) -> PyResult<JustifyContent> {
        match index {
            0 => Ok(JustifyContent::FlexStart),
            1 => Ok(JustifyContent::FlexEnd),
            2 => Ok(JustifyContent::Center),
            3 => Ok(JustifyContent::SpaceBetween),
            4 => Ok(JustifyContent::SpaceAround),
            5 => Ok(JustifyContent::SpaceEvenly),
            n => Err(exceptions::ValueError::py_err(format!("enum JustifyContent - invalid index: {}", n))),
        }
    }
}

impl FromIndex<Overflow> for Overflow {
    fn from_index(index: i32) -> PyResult<Overflow> {
        match index {
            0 => Ok(Overflow::Visible),
            1 => Ok(Overflow::Hidden),
            2 => Ok(Overflow::Scroll),
            n => Err(exceptions::ValueError::py_err(format!("enum Overflow - invalid index: {}", n))),
        }
    }
}

impl FromIndex<PositionType> for PositionType {
    fn from_index(index: i32) -> PyResult<PositionType> {
        match index {
            0 => Ok(PositionType::Relative),
            1 => Ok(PositionType::Absolute),
            n => Err(exceptions::ValueError::py_err(format!("enum PositionType - invalid index: {}", n))),
        }
    }
}

impl FromIndex<FlexWrap> for FlexWrap {
    fn from_index(index: i32) -> PyResult<FlexWrap> {
        match index {
            0 => Ok(FlexWrap::NoWrap),
            1 => Ok(FlexWrap::Wrap),
            2 => Ok(FlexWrap::WrapReverse),
            n => Err(exceptions::ValueError::py_err(format!("enum FlexWrap - invalid index: {}", n))),
        }
    }
}


/* ======================================================================== */
/* HELPER STRUCTS                                                           */
/* ======================================================================== */


#[derive(FromPyObject, IntoPyObject)]
struct StretchDimensionValue {
    dim: i32,
    value: f32,
}

impl StretchDimensionValue {
    fn into(self) -> PyResult<Dimension> {
        match self.dim {
            // TODO: order does not match definition of Dimension.
            0 => Ok(Dimension::Points(self.value)),
            1 => Ok(Dimension::Percent(self.value)),
            2 => Ok(Dimension::Auto),
            3 => Ok(Dimension::Undefined),
            n => Err(exceptions::ValueError::py_err(format!("Invalid value: {}", n))),
        }
    }
}


/* ======================================================================== */
/* DUPLICATE STRUCTS - based on geometry.rs                                 */
/* ======================================================================== */


// TODO: expose class via [pyclass] alongside dict_derive
#[derive(FromPyObject, IntoPyObject)]
pub struct StretchStyleRect {
    start: StretchDimensionValue,
    end: StretchDimensionValue,
    top: StretchDimensionValue,
    bottom: StretchDimensionValue,
}

// TODO: expose class via [pyclass] alongside dict_derive
#[derive(FromPyObject, IntoPyObject)]
pub struct StretchSize {
    width: f32,
    height: f32,
}

// TODO: expose class via [pyclass] alongside dict_derive
#[derive(FromPyObject, IntoPyObject)]
pub struct StretchStyleSize {
    width: StretchDimensionValue,
    height: StretchDimensionValue,
}


/* ======================================================================== */
/* FUNCS & CLASSES                                                          */
/* TODO: c like interface could be replaced with structs with pyo3          */
/* ======================================================================== */


// TODO: make values optional, transfer functionality away from python
#[pyfunction]
unsafe fn stretch_style_create(
    display: i32,
    position_type: i32,
    direction: i32,
    flex_direction: i32,
    flex_wrap: i32,
    overflow: i32,
    align_items: i32,
    align_self: i32,
    align_content: i32,
    justify_content: i32,

    position: StretchStyleRect,
    margin: StretchStyleRect,
    padding: StretchStyleRect,
    border: StretchStyleRect,

    flex_grow: f32,
    flex_shrink: f32,

    flex_basis: StretchDimensionValue,

    size: StretchStyleSize,
    min_size: StretchStyleSize,
    max_size: StretchStyleSize,

    aspect_ratio: f32,
) -> PyResult<i64> {
    let ptr = Box::into_raw(Box::new(Style {
        display:         Display::from_index(display)?,
        position_type:   PositionType::from_index(position_type)?,
        direction:       Direction::from_index(direction)?,
        flex_direction:  FlexDirection::from_index(flex_direction)?,
        flex_wrap:       FlexWrap::from_index(flex_wrap)?,
        overflow:        Overflow::from_index(overflow)?,
        align_items:     AlignItems::from_index(align_items)?,
        align_self:      AlignSelf::from_index(align_self)?,
        align_content:   AlignContent::from_index(align_content)?,
        justify_content: JustifyContent::from_index(justify_content)?,

        position: Rect {
            start:  position.start.into()?,
            end:    position.end.into()?,
            top:    position.top.into()?,
            bottom: position.bottom.into()?,
        },

        margin: Rect {
            start:  margin.start.into()?,
            end:    margin.end.into()?,
            top:    margin.top.into()?,
            bottom: margin.bottom.into()?,
        },

        padding: Rect {
            start:  padding.start.into()?,
            end:    padding.end.into()?,
            top:    padding.top.into()?,
            bottom: padding.bottom.into()?,
        },

        border: Rect {
            start:  border.start.into()?,
            end:    border.end.into()?,
            top:    border.top.into()?,
            bottom: border.bottom.into()?,
        },

        flex_grow,
        flex_shrink,

        flex_basis: flex_basis.into()?,

        size:     Size { width: size.width.into()?, height: size.height.into()? },
        min_size: Size { width: min_size.width.into()?, height: min_size.height.into()? },
        max_size: Size { width: max_size.width.into()?, height: max_size.height.into()? },

        aspect_ratio: if f32::is_nan(aspect_ratio) { Number::Undefined } else { Number::Defined(aspect_ratio) },
    }));
    Ok(ptr as i64)
}

#[pyfunction]
unsafe fn stretch_style_free(style: i64) {
    let _style = Box::from_raw(style as *mut Style);
}

#[pyfunction]
unsafe fn stretch_init() -> i64 {
    let stretch = stretch::node::Stretch::new();
    Box::into_raw(Box::new(stretch)) as i64
}

#[pyfunction]
unsafe fn stretch_free(stretch: i64) {
    Box::from_raw(stretch as *mut Stretch);
}

#[pyfunction]
unsafe fn stretch_node_create(stretch: i64, style: i64) -> i64 {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let style = Box::from_raw(style as *mut Style);
    let node = stretch.new_node(*style, vec![]).unwrap();

    Box::leak(style);
    Box::leak(stretch);

    Box::into_raw(Box::new(node)) as i64
}

#[pyfunction]
unsafe fn stretch_node_free(stretch: i64, node: i64) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove(*node);

    Box::leak(stretch);
}

#[pyfunction]
unsafe fn stretch_node_set_measure(
    stretch: i64,
    node: i64,
    node_self: PyObject,
    measure: PyObject // fn(i64, f32, f32) -> StretchSize
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .set_measure(
            *node,
            Some(Box::new(move |constraint| {
                // acquire lock
                let gil = Python::acquire_gil();
                let py = gil.python();
                // call function
                let args = (&node_self, constraint.width.or_else(f32::NAN), constraint.height.or_else(f32::NAN));
                let result = measure.call1(py, args).unwrap();
                // cast
                let size: StretchSize = result.extract(py).unwrap();
                // return args
                Ok(Size { width: size.width, height: size.height })
            })),
        )
        .unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[pyfunction]
unsafe fn stretch_node_set_style(stretch: i64, node: i64, style: i64) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let style = Box::from_raw(style as *mut Style);

    stretch.set_style(*node, *style).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(style);
}

#[pyfunction]
unsafe fn stretch_node_dirty(stretch: i64, node: i64) -> bool {
    let stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let dirty = stretch.dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);

    dirty
}

#[pyfunction]
unsafe fn stretch_node_mark_dirty(stretch: i64, node: i64) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.mark_dirty(*node).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[pyfunction]
unsafe fn stretch_node_add_child(stretch: i64, node:i64, child: i64) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.add_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[pyfunction]
unsafe fn stretch_node_replace_child_at_index(
    stretch: i64,
    node: i64,
    index: usize,
    child: i64,
) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    stretch.replace_child_at_index(*node, index, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);
}

#[pyfunction]
unsafe fn stretch_node_remove_child(stretch: i64, node: i64, child: i64) -> PyResult<()> {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);
    let child = Box::from_raw(child as *mut Node);

    // TODO: this fails with an unknown error...
    stretch.remove_child(*node, *child).unwrap();

    Box::leak(stretch);
    Box::leak(node);
    Box::leak(child);

    Ok(())
}

#[pyfunction]
unsafe fn stretch_node_remove_child_at_index(stretch: i64, node: i64, index: usize) {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch.remove_child_at_index(*node, index).unwrap();

    Box::leak(stretch);
    Box::leak(node);
}

#[pyfunction]
unsafe fn stretch_node_compute_layout(
    stretch: i64,
    node: i64,
    width: f32,
    height: f32,
) -> Vec<f32> {
    let mut stretch = Box::from_raw(stretch as *mut Stretch);
    let node = Box::from_raw(node as *mut Node);

    stretch
        .compute_layout(
            *node,
            Size {
                width: if f32::is_nan(width) { Number::Undefined } else { Number::Defined(width) },
                height: if f32::is_nan(height) { Number::Undefined } else { Number::Defined(height) },
            },
        )
        .unwrap();

    let mut output = vec![];
    copy_output(&stretch, *node, &mut output);

    Box::leak(stretch);
    Box::leak(node);

    output
}

fn copy_output(stretch: &Stretch, node: Node, output: &mut Vec<f32>) {
    let layout = stretch.layout(node).unwrap();
    let children = stretch.children(node).unwrap();

    output.push(layout.location.x);
    output.push(layout.location.y);
    output.push(layout.size.width);
    output.push(layout.size.height);
    output.push(children.len() as f32);

    for child in &children {
        copy_output(stretch, *child, output);
    }
}


/* ======================================================================== */
/* MODULE                                                                   */
/* ======================================================================== */


#[pymodule]
pub fn _bindings(_py: Python, m: &PyModule) -> PyResult<()> {
    /* FUNC*/
    m.add_wrapped(wrap_pyfunction!(stretch_style_create))?;
    m.add_wrapped(wrap_pyfunction!(stretch_style_free))?;
    m.add_wrapped(wrap_pyfunction!(stretch_init))?;
    m.add_wrapped(wrap_pyfunction!(stretch_free))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_create))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_free))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_set_measure))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_set_style))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_dirty))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_mark_dirty))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_add_child))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_replace_child_at_index))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_remove_child))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_remove_child_at_index))?;
    m.add_wrapped(wrap_pyfunction!(stretch_node_compute_layout))?;
    /* END */
    Ok(())
}

// for pyo3-pack, name must match module.
#[pymodule]
fn stretched(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(_bindings))?;
    Ok(())
}
