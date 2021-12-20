from __future__ import annotations
from dataclasses import dataclass
from typing import Optional
from ast import literal_eval
from copy import deepcopy
import functools

import itertools


@dataclass
class BinaryNode:
    data: Optional[int]
    left: Optional[BinaryNode] = None
    right: Optional[BinaryNode] = None
    parent: Optional[BinaryNode] = None

    def depth(self) -> int:
        if not self.parent:
            return 0
        else:
            return 1 + self.parent.depth()

    def add_child(self, other) -> None:
        if self.left and self.right:
            raise ValueError("Already full")
        other.parent = self
        if self.left:
            self.right = other
        else:
            self.left = other

    def __str__(self):
        return str(self.show())

    def show(self):
        if self.data is not None:
            return self.data
        else:
            return [self.left.show()] + [self.right.show()]

    def add(self, other) -> BinaryNode:
        N = type(self)(None)
        N.add_child(self)
        N.add_child(other)
        return N

    def split(self) -> None:
        if self.data and self.data >= 10:
            (q, r) = divmod(self.data, 2)
            self.add_child(type(self)(q))
            self.add_child(type(self)(q + r))
            self.data = None

    def root(self) -> BinaryNode:
        if not self.parent:
            return self
        return self.parent.root()

    def magnitude(self) -> int:
        if self.data is not None:
            return self.data
        return 3 * self.left.magnitude() + 2 * self.right.magnitude()


def get_data() -> list[BinaryNode]:
    with open("input/18.txt") as f:
        return [read_row(literal_eval(r)) for r in f.readlines()]


def read_row(vals: list) -> BinaryNode:
    assert len(vals) == 2
    node = BinaryNode(None)
    for v in vals:
        if isinstance(v, int):
            child = BinaryNode(v)
        else:
            child = read_row(v)
        node.add_child(child)
    return node


def explode_left(node: BinaryNode) -> None:
    data = node.left.data
    while True:  # ascending
        p = node.parent
        if not p:
            return
        if p.left is node:
            node = p
        else:
            node = p
            break
    node = node.left
    while node.data is None:
        node = node.right
    node.data += data
    # node.split()


def explode_right(node: BinaryNode) -> None:
    data = node.right.data
    while True:  # ascending
        p = node.parent
        if not p:
            return
        if p.right is node:
            node = p
        else:
            node = p
            break
    node = node.right
    while node.data is None:
        node = node.left
    node.data += data
    # node.split()


def explode(node: BinaryNode) -> None:
    assert node.depth() >= 4
    explode_left(node)
    explode_right(node)
    node.data = 0
    del node.left
    node.left = None
    del node.right
    node.right = None


def reduction(node: BinaryNode):
    while True:
        if explode_step(node):
            continue
        if split_step(node):
            continue
        else:
            break


def combine(a: BinaryNode, b: BinaryNode) -> BinaryNode:
    a = deepcopy(a)
    b = deepcopy(b)
    c = a.add(b)
    reduction(c)
    # print(c)
    # print()
    return c


def explode_step(node: BinaryNode) -> bool:
    changed = False
    if not node:
        return False

    if (
        node.depth() >= 4 and node.data is None
    ):  # and node.data and  node.data.left is not None:
        explode(node)
        return True

    return explode_step(node.left) or explode_step(node.right)


def split_step(node: BinaryNode) -> bool:
    changed = False
    if not node:
        return False
    if node.data is not None and node.data >= 10:
        node.split()
        changed = True
        return changed

    return split_step(node.left) or split_step(node.right)


def reduction_step(node: BinaryNode) -> bool:
    changed = False
    if not node:
        return False

    if reduction_step(node.left):
        return True

    if node.data is not None and node.data >= 10:
        node.split()
        changed = True
        return changed

    if (
        node.depth() >= 4 and node.data is None
    ):  # and node.data and  node.data.left is not None:
        explode(node)
        return True

    if reduction_step(node.right):
        return True

    return False


def part_1():
    data = get_data()
    res = functools.reduce(combine, data)
    print(res)
    print(res.magnitude() )

def part_2():
    data = get_data()

    pairs = ( (a,b) for (a,b) in itertools.product(data, repeat=2) if a is not b )
    print(max(combine(a,b).magnitude() for (a,b) in pairs     ))


def main():
    part_1()
    part_2()
    ##data = get_data()
    ##list(map(print, data))
    ## print  (data[0].add(data[1]))

    ##  K = BinaryNode(15)
    ##  K.split()
    # val = read_row([[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]])
    # print(val)
    # k = reduction(val)
    # print(k)
    # print(val)

    ##res = functools.reduce(combine, data)
    ##print(res)


if __name__ == "__main__":
    main()
