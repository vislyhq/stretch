#include "libstretch.h"
#include <assert.h>
#include <stdio.h>

int main() {
    StyleNode *child1 = create_style_node();
    child1->size.width.tag = DIMENSION_POINTS;
    child1->size.width.POINTS._0 = 123.0f;
    child1->size.height.tag = DIMENSION_POINTS;
    child1->size.height.POINTS._0 = 321.0f;

    StyleNode *child2 = create_style_node();
    child2->size.width.tag = DIMENSION_POINTS;
    child2->size.width.POINTS._0 = 234.0f;
    child2->size.height.tag = DIMENSION_POINTS;
    child2->size.height.POINTS._0 = 432.0f;

    StyleNode *node = create_style_node();

    add_style_node(node, child1);
    add_style_node(node, child2);

    LayoutNode *layout = compute_layout_node(node);

    cleanup_style_node(node);

    assert(layout->size.height == 432.0);
    assert(layout->size.width == 357.0);
    assert(layout->children.length == 2);

    cleanup_layout_node(layout);
}
