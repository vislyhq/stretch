#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum {
    ALIGN_CONTENT_FLEX_START,
    ALIGN_CONTENT_FLEX_END,
    ALIGN_CONTENT_CENTER,
    ALIGN_CONTENT_STRETCH,
    ALIGN_CONTENT_SPACE_BETWEEN,
    ALIGN_CONTENT_SPACE_AROUND,
} AlignContent;

typedef enum {
    ALIGN_ITEMS_FLEX_START,
    ALIGN_ITEMS_FLEX_END,
    ALIGN_ITEMS_CENTER,
    ALIGN_ITEMS_BASELINE,
    ALIGN_ITEMS_STRETCH,
} AlignItems;

typedef enum {
    ALIGN_SELF_AUTO,
    ALIGN_SELF_FLEX_START,
    ALIGN_SELF_FLEX_END,
    ALIGN_SELF_CENTER,
    ALIGN_SELF_BASELINE,
    ALIGN_SELF_STRETCH,
} AlignSelf;

typedef enum {
    DIRECTION_INHERIT,
    DIRECTION_L_T_R,
    DIRECTION_R_T_L,
} Direction;

typedef enum {
    DISPLAY_FLEX,
    DISPLAY_NONE,
} Display;

typedef enum {
    FLEX_DIRECTION_ROW,
    FLEX_DIRECTION_COLUMN,
    FLEX_DIRECTION_ROW_REVERSE,
    FLEX_DIRECTION_COLUMN_REVERSE,
} FlexDirection;

typedef enum {
    FLEX_WRAP_NO_WRAP,
    FLEX_WRAP_WRAP,
    FLEX_WRAP_WRAP_REVERSE,
} FlexWrap;

typedef enum {
    JUSTIFY_CONTENT_FLEX_START,
    JUSTIFY_CONTENT_FLEX_END,
    JUSTIFY_CONTENT_CENTER,
    JUSTIFY_CONTENT_SPACE_BETWEEN,
    JUSTIFY_CONTENT_SPACE_AROUND,
    JUSTIFY_CONTENT_SPACE_EVENLY,
} JustifyContent;

typedef enum {
    OVERFLOW_VISIBLE,
    OVERFLOW_HIDDEN,
    OVERFLOW_SCROLL,
} Overflow;

typedef enum {
    POSITION_TYPE_RELATIVE,
    POSITION_TYPE_ABSOLUTE,
} PositionType;

typedef enum {
    UNIT_UNDEFINED,
    UNIT_AUTO,
    UNIT_POINTS,
    UNIT_PERCENT,
} Unit;

typedef struct {
    Unit unit;
    float value;
} Dimension;

typedef struct {
    Dimension start;
    Dimension end;
    Dimension top;
    Dimension bottom;
} Rect_Dimension;

typedef struct {
    Dimension width;
    Dimension height;
} Size_Dimension;

typedef enum {
    DEFINITION_DEFINED,
    DEFINITION_UNDEFINED,
} DEFINITION;

typedef struct {
    DEFINITION definition;
    float value;
} Number;

typedef struct {
    Display display;
    PositionType position_type;
    Direction direction;
    FlexDirection flex_direction;
    FlexWrap flex_wrap;
    Overflow overflow;
    AlignItems align_items;
    AlignSelf align_self;
    AlignContent align_content;
    JustifyContent justify_content;
    Rect_Dimension position;
    Rect_Dimension margin;
    Rect_Dimension padding;
    Rect_Dimension border;
    float flex_grow;
    float flex_shrink;
    Dimension flex_basis;
    Size_Dimension size;
    Size_Dimension min_size;
    Size_Dimension max_size;
    Number aspect_ratio;
    void *children;
} StyleNode;

typedef struct {
    float width;
    float height;
} Size_f32;

typedef struct {
    float x;
    float y;
} Point_f32;

typedef struct {
    uint32_t order;
    Size_f32 size;
    Point_f32 location;
    void *children;
} LayoutNode;

void stretch_add_child(StyleNode *style, StyleNode *child);

void stretch_cleanup_layout(LayoutNode *node);

void stretch_cleanup_style(StyleNode *node);

LayoutNode *stretch_compute_layout(StyleNode *root);

StyleNode *stretch_new_style(void);
