from enum import IntEnum
from typing import Mapping


class NodeType(IntEnum):
    Element = 1
    Text = 3


class XNode:
    node_type: NodeType
    tag: str | None
    text: str | None
    attrs: Mapping[str, XNode] | None
    children: list[XNode] | None

    def __init__(
        self,
        node_type: NodeType = NodeType.Element,
        tag: str | None = None,
        text: str | None = None,
        attrs: Mapping[str, XNode] | None = None,
        children: list[XNode] | None = None,
    ): ...


def parse_str(raw: str) -> list[XNode]: ...
