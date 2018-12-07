#[macro_use]
extern crate criterion;

#[macro_use]
extern crate yoga;

extern crate stretch;

use criterion::Criterion;
use criterion::Fun;
use yoga::prelude::*;
use yoga::Node;
use yoga::StyleUnit::{Auto, UndefinedValue};

fn yoga(n: u64) {
    let mut node = Node::new();

    let mut child = Node::new();
    let mut other_child = Node::new();

    node.insert_child(&mut child, 0);
    node.insert_child(&mut other_child, 1);

    style!(node,
        Margin(10 pt),
        MarginLeft(Auto),
        PaddingHorizontal(4 pt),
        Left(16 %),
        Bottom(UndefinedValue)
    );

    let child_styles = make_styles!(
        Width((n as f32) pt),
        Height(32 pt),
        FlexGrow(1.0),
        Margin(Auto)
    );

    child.apply_styles(&child_styles);
    other_child.apply_styles(&child_styles);

    node.calculate_layout(512.0, 512.0, yoga::Direction::LTR);
}

fn stretch(n: u64) {
    stretch::compute(&stretch::style::Node {
        margin: stretch::style::Edges {
            top: stretch::style::Dimension::Points(10.0),
            start: stretch::style::Dimension::Auto,
            bottom: stretch::style::Dimension::Points(10.0),
            end: stretch::style::Dimension::Points(10.0),
            ..Default::default()
        },
        padding: stretch::style::Edges {
            start: stretch::style::Dimension::Points(4.0),
            end: stretch::style::Dimension::Points(4.0),
            ..Default::default()
        },
        start: stretch::style::Dimension::Percent(16.0),
        children: vec![stretch::style::Node {
            width: stretch::style::Dimension::Points(n as f32),
            height: stretch::style::Dimension::Points(32.0),
            flex_grow: 1.0,
            margin: Default::default(),
            ..Default::default()
        }, stretch::style::Node {
            width: stretch::style::Dimension::Points(n as f32),
            height: stretch::style::Dimension::Points(32.0),
            flex_grow: 1.0,
            margin: Default::default(),
            ..Default::default()
        }],
        ..Default::default()
    });
}

fn layouts(c: &mut Criterion) {
    let yoga_fn = Fun::new("Yoga", |b, i| b.iter(|| yoga(*i)));
    let stretch_fn = Fun::new("Stretch", |b, i| b.iter(|| stretch(*i)));

    let functions = vec![yoga_fn, stretch_fn];
    c.bench_functions("Layout", functions, 20);
}

criterion_group!(benches, layouts);
criterion_main!(benches);
