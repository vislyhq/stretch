
from stretched import *


def test_testing():
    node1 = Node(Style())
    node2 = Node(Style())
    node3 = Node(Style())
    node4 = Node(Style())
    assert node1.dirty == True
    assert node2.dirty == True
    assert node3.dirty == True
    assert node4.dirty == True


def test_createNode():
    child = Node(Style())
    node = Node(Style(), [child])
    assert len(node) == 1


def test_createLeaf(capsys):
    with capsys.disabled():
        node = Node(Style(), measure=lambda size: Size(100, 100))
        layout = node.compute_layout(Size(None, None))
        assert layout.width == 100.0
        assert layout.height == 100.0


def test_setMeasure():
    node = Node(Style())
    node.measure = lambda size: Size(100, 100)
    layout = node.compute_layout(Size(None, None))
    assert layout.width == 100.0
    assert layout.height == 100.0


def test_addChild():
    node = Node(Style())
    child = Node(Style())
    node.add_child(child)
    assert len(node) == 1


def test_removeChild(capsys):
    child = Node(Style())
    node = Node(Style(), [child])
    node.remove_child(child)
    assert len(node) == 0


def test_removeChildAtIndex():
    child = Node(Style())
    node = Node(Style(), [child])
    node.remove_child_index(0)
    assert len(node) == 0


def test_replaceChildAtIndex():
    child1 = Node(Style(size=Size(DimensionValue(Dimension.POINTS, 100.0), DimensionValue(Dimension.POINTS, 100.0))))
    child2 = Node(Style(size=Size(DimensionValue(Dimension.POINTS, 200.0), DimensionValue(Dimension.POINTS, 200.0))))
    node = Node(Style(), [child1])
    node.replace_child_at_index(0, child2)
    layout = node.compute_layout(Size(None, None))
    assert layout.width == 200.0
    assert layout.height == 200.0


def test_setStyle():
    node = Node(Style())
    node.style = Style()


def test_setChildren():
    node = Node(Style())
    child = Node(Style())
    node.children = [child]
    assert len(node) == 1


def test_markDirty():
    node = Node(Style())
    node.compute_layout(Size(None, None))
    assert node.dirty == False
    node.mark_dirty()
    assert node.dirty == True
