"""Typing for the rust code."""

from enum import Enum
from typing import Any
from collections.abc import Mapping, Callable

class NodeType(Enum):
    Element = "Element"
    Comment = "Comment"
    Text = "Text"
    Expression = "Expression"
    Fragment = "Fragment"

class XFragment:
    children: list[XNode]
    __match_args__ = ("children",)

    def __init__(self, children: list[XNode]) -> None: ...

class XElement:
    name: str
    attrs: dict[str, XNode]
    children: list[XNode]

    __match_args__ = ("name", "attrs", "children")

    def __init__(
        self, name: str, attrs: dict[str, XNode], children: list[XNode]
    ) -> None: ...

class XText:
    text: str

    __match_args__ = ("text",)

    def __init__(self, text: str) -> None: ...

class XComment:
    comment: str

    __match_args__ = ("comment",)

    def __init__(self, comment: str) -> None: ...

class XExpression:
    expression: str

    __match_args__ = ("expression",)

    def __init__(self, expression: str) -> None: ...

class XNode:
    """Represent a node in the markup."""
    @staticmethod
    def Fragment(fragment: XFragment, /) -> XNode: ...
    @staticmethod
    def Element(element: XElement, /) -> XNode: ...
    @staticmethod
    def Comment(text: XComment) -> XNode: ...
    @staticmethod
    def Text(text: XText) -> XNode: ...
    @staticmethod
    def Expression(expression: XExpression) -> XNode: ...
    def __init__(self) -> None: ...
    def kind(self) -> NodeType: ...
    def __repr__(self) -> str: ...
    def __eq__(self, other: object) -> bool: ...
    def unwrap(self) -> XFragment | XElement | XComment | XText | XExpression: ...

class XTemplate:
    node: XNode
    params: Mapping[str, type | Any]
    defaults: Mapping[str, Any]

class XCatalog:
    """Catalog of templates en functions."""
    def __init__(self) -> None: ...
    def add_component(
        self,
        name: str,
        template: str,
        params: Mapping[str, type | Any],
        defaults: Mapping[str, Any],
    ) -> None: ...
    def add_function(self, name: str, fn: Callable[..., Any]) -> None: ...
    def get(self, name: str) -> XTemplate: ...
    def render_node(self, node: XNode, params: RenderContext) -> str: ...
    def render(self, template: str, **params: dict[str, Any]) -> str: ...

class RenderContext:
    def __init__(self) -> None: ...
    def push(self, params: Mapping[str, Any]) -> None: ...
    def pop(self) -> None: ...
