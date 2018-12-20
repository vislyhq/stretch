#include "libstretch.h"
#include <assert.h>

int main() {
    StyleNode *child1 = stretch_new_style();
    child1->size.width.unit = UNIT_POINTS;
    child1->size.width.value = 123.0f;
    child1->size.height.unit = UNIT_POINTS;
    child1->size.height.value = 321.0f;

    StyleNode *child2 = stretch_new_style();
    child2->size.width.unit = UNIT_POINTS;
    child2->size.width.value = 234.0f;
    child2->size.height.unit = UNIT_POINTS;
    child2->size.height.value = 432.0f;

    StyleNode *node = stretch_new_style();

    stretch_add_child(node, child1);
    stretch_add_child(node, child2);

    LayoutNode *layout = stretch_compute_layout(node);

    stretch_cleanup_style(node);

    assert(layout->size.height == 432.0);
    assert(layout->size.width == 357.0);

    stretch_cleanup_layout(layout);
}
