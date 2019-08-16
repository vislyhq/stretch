import math
from typing import Optional, List, Callable
from stretched.layout import Layout
from stretched.style import Style, Size, _NAN
from stretched.style import Style, Size
from stretched import _bindings
from stretched._stretch import Stretch


# ========================================================================= #
# NODE                                                                      #
# ========================================================================= #


MeasureFunc = Callable[[Size], Size]


class Node:

    def __init__(self, style: Style, children: Optional[List['Node']] = None, measure: MeasureFunc = None):
        self._ptr: int = _bindings.stretch_node_create(Stretch.get_ptr(), style._ptr)
        self._style: Style = style
        self._children: List['Node'] = []
        self._measure: Optional[Callable] = None
        # add children
        self.children = children
        if callable(measure):
            self.measure = measure

    def __del__(self):
        _bindings.stretch_node_free(Stretch.get_ptr(), self._ptr)

    @property
    def style(self):
        return self._style

    @style.setter
    def style(self, value: Style):
        self._style = value
        _bindings.stretch_node_set_style(Stretch.get_ptr(), self._ptr, value._ptr)

    @property
    def children(self):
        # make a shallow copy
        return self._children[:]

    @children.setter
    def children(self, value: Optional[List['Node']]):
        num_children = len(self._children)
        # Remove
        for i in range(num_children):
            self.remove_child_index(num_children-1-i)
        # Insert
        if value is not None:
            for child in value:
                self.add_child(child)

    def __len__(self):
        return len(self._children)

    @property
    def measure(self):
        return self._measure

    @staticmethod
    def _node_measure_callback(node: 'Node', width: float, height: float) -> dict:
        # TODO this function is not actually needed
        size = node.measure(
            Size(width=None if math.isnan(width) else width, height=None if math.isnan(height) else height))
        return dict(width=size.width, height=size.height)

    @measure.setter
    def measure(self, value: MeasureFunc):
        assert callable(value)
        self._measure = value
        _bindings.stretch_node_set_measure(Stretch.get_ptr(), self._ptr, self, Node._node_measure_callback)

    def add_child(self, child: 'Node'):
        self._children.append(child)
        _bindings.stretch_node_add_child(Stretch.get_ptr(), self._ptr, child._ptr)

    def replace_child_at_index(self, index: int, child: 'Node') -> 'Node':
        assert index >= 0
        _bindings.stretch_node_replace_child_at_index(Stretch.get_ptr(), self._ptr, index, child._ptr)
        oldChild = self._children[index]
        self._children[index] = child
        return oldChild

    def remove_child(self, child: 'Node') -> 'Node':
        self._children.remove(child)
        _bindings.stretch_node_remove_child(Stretch.get_ptr(), self._ptr, child._ptr)
        return child

    def remove_child_index(self, index: int) -> 'Node':
        assert index >= 0
        removed = self._children.pop(index)
        _bindings.stretch_node_remove_child_at_index(Stretch.get_ptr(), self._ptr, index)
        return removed

    @property
    def dirty(self) -> bool:
        return _bindings.stretch_node_dirty(Stretch.get_ptr(), self._ptr)

    def mark_dirty(self):
        _bindings.stretch_node_mark_dirty(Stretch.get_ptr(), self._ptr)

    def compute_layout(self, size: Size[Optional[float]]) -> Layout:
        float_layout = _bindings.stretch_node_compute_layout(
            Stretch.get_ptr(),
            self._ptr,
            size.width if size.width is not None else _NAN,
            size.height if size.height is not None else _NAN,
        )
        _, layout = Layout.from_float_list(float_layout)
        return layout

    def __str__(self):
        return '(node: _ptr={}, measure={}, children={})'.format(self._ptr, self._children, self._measure)


# ========================================================================= #
# END                                                                       #
# ========================================================================= #
