from typing import TypedDict
from xcomponent import Catalog
from xcomponent.xcore import XNode

import pytest

catalog = Catalog()


class User(TypedDict):
    username: str

class Product(TypedDict):
    owner: User

@catalog.component()
def DummyNode(a: int) -> str:
    return """<p>{a}</p>"""


@catalog.component()
def Types(a: bool, b: bool, c: int, d: str, e: XNode) -> str:
    return """<>{a}-{b}-{c}-{d}-{e}</>"""


@catalog.component()
def DictComplexType(u: User) -> str:
    return """<>{u.username}</>"""

@catalog.component()
def NestedDictComplexType(product: Product) -> str:
    return """<>{product.owner.username}</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(
            Types(False, True, 2, "3", DummyNode(a="4")),
            "false-true-2-3-<p>4</p>",
            id="simpletypes",
        ),
        pytest.param(
            DictComplexType({"username": "bob"}),
            "bob",
            id="dict",
        ),

        pytest.param(
            NestedDictComplexType(Product(owner=User(username="alice"))),
            "alice",
            id="nested-dict",
        ),
    ],
)
def test_types(component: str, expected: str):
    assert component == expected
