#[test]
fn padding_align_end_child() {
    let layout = stretch::compute(
        &stretch::style::Node {
            align_items: stretch::style::AlignItems::FlexEnd,
            justify_content: stretch::style::JustifyContent::FlexEnd,
            size: stretch::geometry::Size {
                width: stretch::style::Dimension::Points(200.0000),
                height: stretch::style::Dimension::Points(200.0000),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100.0000),
                    height: stretch::style::Dimension::Points(100.0000),
                    ..Default::default()
                },
                padding: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(20.0000),
                    end: stretch::style::Dimension::Points(20.0000),
                    top: stretch::style::Dimension::Points(20.0000),
                    bottom: stretch::style::Dimension::Points(20.0000),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap();

    assert_eq!(layout.size.width, 200.0000);
    assert_eq!(layout.size.height, 200.0000);
    assert_eq!(layout.location.x, 0.0000);
    assert_eq!(layout.location.y, 0.0000);

    assert_eq!(layout.children[0].size.width, 100.0000);
    assert_eq!(layout.children[0].size.height, 100.0000);
    assert_eq!(layout.children[0].location.x, 100.0000);
    assert_eq!(layout.children[0].location.y, 100.0000);
}
