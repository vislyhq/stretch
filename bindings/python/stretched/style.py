from enum import IntEnum
from typing import Optional, TypeVar, Generic
from stretched import _bindings


# ========================================================================= #
# STYLE - ENUMS                                                             #
# ========================================================================= #


class AlignItems(IntEnum):
    FLEX_START: int = 0
    FLEX_END: int = 1
    CENTER: int = 2
    BASELINE: int = 3
    STRETCH: int = 4


class AlignSelf(IntEnum):
    AUTO: int = 0
    FLEX_START: int = 1
    FLEX_END: int = 2
    CENTER: int = 3
    BASELINE: int = 4
    STRETCH: int = 5


class AlignContent(IntEnum):
    FLEX_START: int = 0
    FLEX_END: int = 1
    CENTER: int = 2
    STRETCH: int = 3
    SPACE_BETWEEN: int = 4
    SPACE_AROUND: int = 5


class Direction(IntEnum):
    INHERIT: int = 0
    LTR: int = 1
    RTL: int = 2


class Display(IntEnum):
    FLEX: int = 0
    NONE: int = 1


class FlexDirection(IntEnum):
    ROW: int = 0
    COLUMN: int = 1
    ROW_REVERSE: int = 2
    COLUMN_REVERSE: int = 3


class JustifyContent(IntEnum):
    FLEX_START: int = 0
    FLEX_END: int = 1
    CENTER: int = 2
    SPACE_BETWEEN: int = 3
    SPACE_AROUND: int = 4
    SPACE_EVENLY: int = 5


class Overflow(IntEnum):
    VISIBLE: int = 0
    HIDDEN: int = 1
    SCROLL: int = 2


class PositionType(IntEnum):
    RELATIVE: int = 0
    ABSOLUTE: int = 1


class FlexWrap(IntEnum):
    NO_WRAP: int = 0
    WRAP: int = 1
    WRAP_REVERSE: int = 2


class Dimension(IntEnum):
    # TODO: order does not match definition in stretch crate.
    POINTS: int = 0
    PERCENT: int = 1
    AUTO: int = 2
    UNDEFINED: int = 3

    @staticmethod
    def new_points(value=0.0) -> 'DimensionValue':
        return DimensionValue(Dimension.POINTS, value)

    @staticmethod
    def new_percent(value=0.0) -> 'DimensionValue':
        return DimensionValue(Dimension.PERCENT, value)

    @staticmethod
    def new_auto() -> 'DimensionValue':
        return DimensionValue(Dimension.AUTO, 0.0)

    @staticmethod
    def new_undefined() -> 'DimensionValue':
        return DimensionValue(Dimension.UNDEFINED, 0.0)


# ========================================================================= #
# STYLE - DATA                                                              #
# ========================================================================= #


T = TypeVar('T')


class DimensionValue(Generic[T]):
    def __init__(self, unit: Dimension, value: T = 0):
        self.unit = unit
        self.value = value

    @property
    def stretch_value(self):
        if self.unit == Dimension.POINTS:
            return dict(dim=0, value=self.value)
        elif self.unit == Dimension.PERCENT:
            return dict(dim=1, value=self.value)
        elif self.unit == Dimension.AUTO:
            return dict(dim=2, value=self.value)
        elif self.unit == Dimension.UNDEFINED:
            return dict(dim=3, value=self.value)
        else:
            raise RuntimeError('This should never happen')

    def __str__(self):
        return '(value: unit={}, value={})'.format(self.unit, self.value)


DimensionValue.AUTO = Dimension.new_auto()
DimensionValue.UNDEFINED = Dimension.new_undefined()


class Size(Generic[T]):
    def __init__(self, width: T, height: T):
        self.width: T = width if width is not None else _NAN
        self.height: T = height if height is not None else _NAN

    def __str__(self):
        return '(size: width={}, height={})'.format(self.width, self.height)


class Rect(Generic[T]):
    def __init__(self, start: DimensionValue[T], end: DimensionValue[T], top: DimensionValue[T], bottom: DimensionValue[T]):
        self.start: DimensionValue[T] = start
        self.end: DimensionValue[T] = end
        self.top: DimensionValue[T] = top
        self.bottom: DimensionValue[T] = bottom

    def __str__(self):
        return '(rect: start={}, end={}, top={}, bottom={})'.format(self.start, self.end, self.top, self.bottom)


# ========================================================================= #
# STYLE                                                                     #
# ========================================================================= #


_NAN = float('nan')


class Style:
    def __init__(
            self,
            display: Display                = Display.FLEX,
            position_type: PositionType     = PositionType.RELATIVE,
            direction: Direction            = Direction.INHERIT,
            flex_direction: FlexDirection   = FlexDirection.ROW,
            flex_wrap: FlexWrap             = FlexWrap.NO_WRAP,
            overflow: Overflow              = Overflow.HIDDEN,
            align_items: AlignItems         = AlignItems.STRETCH,
            align_self: AlignSelf           = AlignSelf.AUTO,
            align_content: AlignContent     = AlignContent.FLEX_START,
            justify_content: JustifyContent = JustifyContent.FLEX_START,
            position: Rect[DimensionValue]       = Rect(start=DimensionValue.UNDEFINED, end=DimensionValue.UNDEFINED, top=DimensionValue.UNDEFINED, bottom=DimensionValue.UNDEFINED),
            margin: Rect[DimensionValue]         = Rect(start=DimensionValue.UNDEFINED, end=DimensionValue.UNDEFINED, top=DimensionValue.UNDEFINED, bottom=DimensionValue.UNDEFINED),
            padding: Rect[DimensionValue]        = Rect(start=DimensionValue.UNDEFINED, end=DimensionValue.UNDEFINED, top=DimensionValue.UNDEFINED, bottom=DimensionValue.UNDEFINED),
            border: Rect[DimensionValue]         = Rect(start=DimensionValue.UNDEFINED, end=DimensionValue.UNDEFINED, top=DimensionValue.UNDEFINED, bottom=DimensionValue.UNDEFINED),
            flex_grow: float                = 0.0,
            flex_shrink: float              = 1.0,
            flex_basis: DimensionValue           = DimensionValue.AUTO,
            size: Size[DimensionValue]           = Size(width=DimensionValue.AUTO, height=DimensionValue.AUTO),
            min_size: Size[DimensionValue]       = Size(width=DimensionValue.AUTO, height=DimensionValue.AUTO),
            max_size: Size[DimensionValue]       = Size(width=DimensionValue.AUTO, height=DimensionValue.AUTO),
            aspect_ratio: Optional[float]   = None
    ):
        self.display: Display               = display
        self.positionType: PositionType     = position_type
        self.direction: Direction           = direction
        self.flexDirection: FlexDirection   = flex_direction
        self.flexWrap: FlexWrap             = flex_wrap
        self.overflow: Overflow             = overflow
        self.alignItems: AlignItems         = align_items
        self.alignSelf: AlignSelf           = align_self
        self.alignContent: AlignContent     = align_content
        self.justifyContent: JustifyContent = justify_content
        self.position: Rect                 = position
        self.margin: Rect                   = margin
        self.padding: Rect                  = padding
        self.border: Rect                   = border
        self.flexGrow: float                = flex_grow
        self.flexShrink: float              = flex_shrink
        self.flexBasis: DimensionValue      = flex_basis
        self.size: Size                     = size
        self.minSize: Size                  = min_size
        self.maxSize: Size                  = max_size
        self.aspectRatio: Optional[float]   = aspect_ratio

        self._ptr: int = _bindings.stretch_style_create(
            display =         display.value,
            position_type =   position_type.value,
            direction =       direction.value,
            flex_direction =  flex_direction.value,
            flex_wrap =       flex_wrap.value,
            overflow =        overflow.value,
            align_items =     align_items.value,
            align_self =      align_self.value,
            align_content =   align_content.value,
            justify_content = justify_content.value,
            position =        dict(start=position.start.stretch_value, end=position.end.stretch_value, top=position.top.stretch_value, bottom=position.bottom.stretch_value),
            margin =          dict(start=margin.start.stretch_value, end=margin.end.stretch_value, top=margin.top.stretch_value, bottom=margin.bottom.stretch_value),
            padding =         dict(start=padding.start.stretch_value, end=padding.end.stretch_value, top=padding.top.stretch_value, bottom=padding.bottom.stretch_value),
            border =          dict(start=border.start.stretch_value, end=border.end.stretch_value, top=border.top.stretch_value, bottom=border.bottom.stretch_value),
            flex_grow =       flex_grow,
            flex_shrink =     flex_shrink,
            flex_basis =      flex_basis.stretch_value,
            size =            dict(width=size.width.stretch_value, height=size.height.stretch_value),
            min_size =        dict(width=min_size.width.stretch_value, height=min_size.height.stretch_value),
            max_size =        dict(width=max_size.width.stretch_value, height=max_size.height.stretch_value),
            aspect_ratio =    aspect_ratio if aspect_ratio is not None else _NAN,
        )

    def __del__(self):
        _bindings.stretch_style_free(self._ptr)

    def __str__(self):
        return '\n'.join([
            '(style:'
            'display={}'.format(self.display),
            'positionType={}'.format(self.positionType),
            'direction={}'.format(self.direction),
            'flexDirection={}'.format(self.flexDirection),
            'flexWrap={}'.format(self.flexWrap),
            'overflow={}'.format(self.overflow),
            'alignItems={}'.format(self.alignItems),
            'alignSelf={}'.format(self.alignSelf),
            'alignContent={}'.format(self.alignContent),
            'justifyContent={}'.format(self.justifyContent),
            'position={}'.format(self.position),
            'margin={}'.format(self.margin),
            'padding={}'.format(self.padding),
            'border={}'.format(self.border),
            'flexGrow={}'.format(self.flexGrow),
            'flexShrink={}'.format(self.flexShrink),
            'flexBasis={}'.format(self.flexBasis),
            'size={}'.format(self.size),
            'minSize={}'.format(self.minSize),
            'maxSize={}'.format(self.maxSize),
            'aspectRatio={}'.format(self.aspectRatio),
            ')'
        ])


# ========================================================================= #
# END                                                                       #
# ========================================================================= #
