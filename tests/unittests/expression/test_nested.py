from typing import Any
import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component
from xcomponent.xcore import XNode

catalog = Catalog()


@catalog.component()
def AddOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a + b}</>"""


@catalog.component()
def NestedOperation(aa: str, bb: str) -> str:
    return """<AddOp a={aa} b={bb} />"""


@catalog.component()
def NestedExpression(aa: str, bb: str) -> str:
    return """<>{<AddOp a={aa} b={bb} />}</>"""


catalog.function(max)


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(NestedOperation("1", "2"), "12", id="operation"),
        pytest.param(NestedExpression("1", "2"), "12", id="expression"),
    ],
)
def test_nested(component: str, expected: str):
    assert component == expected
