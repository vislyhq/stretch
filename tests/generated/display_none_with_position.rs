#[test]
fn display_none_with_position() {
    let layout = stretch::compute(
        &stretch::style::Node {
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(100.0000),
                height: stretch::style::Dimension::Points(100.0000),
                ..Default::default()
            },
            children: vec![
                stretch::style::Node { flex_grow: 1.0000, ..Default::default() },
                stretch::style::Node {
                    display: stretch::style::Display::None,
                    flex_grow: 1.0000,
                    position: stretch::geometry::Rect {
                        top: stretch::style::Dimension::Points(10.0000),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 100.0000);
    assert_eq!(layout.size.height, 100.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 0.0000);
    assert_eq!(layout.children[0].location.y, 0.0000);

    assert_eq!(layout.children[1].size.width, 0.0000);
    assert_eq!(layout.children[1].size.height, 0.0000);
    assert_eq!(layout.children[1].location.x, 0.0000);
    assert_eq!(layout.children[1].location.y, 0.0000);
}
