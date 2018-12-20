#include "libstretch.h"
#include <assert.h>
#include <stdio.h>

int main() {
    StyleNode *child1 = create_style_node();
    child1->size.width.unit = UNIT_POINTS;
    child1->size.width.value = 123.0f;
    child1->size.height.unit = UNIT_POINTS;
    child1->size.height.value = 321.0f;

    StyleNode *child2 = create_style_node();
    child2->size.width.unit = UNIT_POINTS;
    child2->size.width.value = 234.0f;
    child2->size.height.unit = UNIT_POINTS;
    child2->size.height.value = 432.0f;

    StyleNode *node = create_style_node();

    add_style_node(node, child1);
    add_style_node(node, child2);

    LayoutNode *layout = compute_layout_node(node);

    cleanup_style_node(node);

    assert(layout->size.height == 432.0);
    assert(layout->size.width == 357.0);

    cleanup_layout_node(layout);
}
