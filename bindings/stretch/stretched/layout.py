from typing import List, Tuple


# ========================================================================= #
# LAYOUT                                                                    #
# ========================================================================= #


class Layout :

    def __init__(self, x: float, y: float, width: float, height: float, children: List['Layout']):
        self.x: float = x
        self.y: float = y
        self.width: float = width
        self.height: float = height
        self.children: List[Layout] = children

    @staticmethod
    def from_float_list(floats: List[float], offset: int = 0) -> Tuple[int, 'Layout']:
        next_offset = offset + 5
        x, y, width, height, child_count = floats[offset:next_offset]

        children: List[Layout] = []
        for i in range(int(child_count)):
            (next_offset, layout) = Layout.from_float_list(floats, next_offset)
            children.append(layout)

        return next_offset, Layout(x=x, y=y, width=width, height=height, children=children)

    def __str__(self):
        return '(layout: x={}, y={}, width={}, height={}, children={})'.format(
            self.x, self.y, self.width, self.height, self.children
        )


# ========================================================================= #
# END                                                                       #
# ========================================================================= #
