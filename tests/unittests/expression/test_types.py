from dataclasses import dataclass
from xcomponent import Catalog
from xcomponent.xcore import XNode

import pytest

catalog = Catalog()


@dataclass
class User:
    username: str


@catalog.component()
def DummyNode(a: int) -> str:
    return """<p>{a}</p>"""


@catalog.component()
def Types(a: bool, b: bool, c: int, d: str, e: XNode) -> str:
    return """<>{a}-{b}-{c}-{d}-{e}</>"""


@catalog.component()
def ComplexType(u: User) -> str:
    return """<>{u.username}</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(
            Types(False, True, 2, "3", DummyNode(a="4")),
            "false-true-2-3-<p>4</p>",
            id="simpletypes",
        ),
        pytest.param(
            ComplexType(User(username="bob")),
            "bob",
            id="complex-type",
        ),
    ],
)
def test_types(component: str, expected: str):
    assert component == expected
