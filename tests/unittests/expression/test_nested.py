import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def AddOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a + b}</>"""


@catalog.component
def NestedOperation(aa: str, bb: str) -> str:
    return """<AddOp a={aa} b={bb} />"""


@catalog.component
def NestedExpression(aa: str, bb: str) -> str:
    return """<>{<AddOp a={aa} b={bb} />}</>"""


@catalog.component
def NestedFunction(aa: int, bb: int) -> str:
    return """<>{<AddOp a={max(aa, 3)} b={bb} />}</>"""


@catalog.component
def NestedIf(aa: int, bb: int) -> str:
    return """<>{<AddOp a={if aa > 3 { 6 } else { 0 }} b={bb} />}</>"""


catalog.function(max)


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(NestedOperation("1", "2"), "12", id="operation"),
        pytest.param(NestedExpression("1", "2"), "12", id="expression"),
        pytest.param(NestedFunction(6, 6), "12", id="expression"),
        pytest.param(NestedIf(5, 6), "12", id="expression"),
    ],
)
def test_nested(component: str, expected: str):
    assert component == expected
