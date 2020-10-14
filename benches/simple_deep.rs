use criterion::{criterion_group, criterion_main, Criterion};

fn build_deep_hierarchy(stretch: &mut stretch::node::Stretch) -> stretch::node::Node {
    let parent_node = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50.),
                    height: stretch::style::Dimension::Points(50.),
                },
                ..Default::default()
            },
            &[],
        )
        .unwrap();
    let mut child = stretch.new_node(stretch::style::Style::default(), &[]).unwrap();
    stretch.set_children(parent_node, &[child]).unwrap();

    for _i in 0..14 {
        let new_child = stretch.new_node(stretch::style::Style::default(), &[]).unwrap();
        stretch.set_children(child, &[new_child]).unwrap();
        child = new_child;
    }
    parent_node
}

fn stretch_benchmarks(c: &mut Criterion) {
    c.bench_function("simple deep hierarchy - build", |b| {
        b.iter(|| {
            let mut stretch = stretch::node::Stretch::new();
            build_deep_hierarchy(&mut stretch);
        })
    });

    c.bench_function("simple deep hierarchy - single", |b| {
        b.iter(|| {
            let mut stretch = stretch::node::Stretch::new();
            let root = build_deep_hierarchy(&mut stretch);
            stretch.compute_layout(root, stretch::geometry::Size::undefined()).unwrap()
        })
    });

    c.bench_function("simple deep hierarchy - relayout", |b| {
        let mut stretch = stretch::node::Stretch::new();
        let root = build_deep_hierarchy(&mut stretch);

        b.iter(|| {
            stretch.mark_dirty(root);
            stretch.compute_layout(root, stretch::geometry::Size::undefined()).unwrap()
        })
    });
}

criterion_group!(benches, stretch_benchmarks);
criterion_main!(benches);
