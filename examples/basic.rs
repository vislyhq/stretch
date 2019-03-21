use stretch::geometry::Size;
use stretch::style::*;

fn main() {
    let mut db = stretch::Database::new();
    let root = db.new_node(Node {
        size: Size { width: Dimension::Points(100.0.into()), height: Dimension::Points(100.0.into()) },
        justify_content: JustifyContent::Center,
        ..Default::default()
    });

    let half_child = db.new_node(Node {
        size: Size { width: Dimension::Percent(0.5.into()), height: Dimension::Auto },
        ..Default::default()
    });

    db.set_children(root, vec![half_child]);

    let layout = stretch::compute(&mut db, root, Size::undefined()).unwrap();

    println!("{:#?}", layout);
}
