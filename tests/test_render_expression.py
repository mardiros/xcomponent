from typing import Any
import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component
from xcomponent.xcore import XNode

catalog = Catalog()


@catalog.component()
def DummyNode(a: int) -> str:
    return """<p>{a}</p>"""


@catalog.component()
def Types(a: bool, b: bool, c: int, d: str, e: XNode) -> str:
    return """<>{a}-{b}-{c}-{d}-{e}</>"""


@catalog.component()
def AddOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a + b}</>"""


@catalog.component()
def SubOp(a: int | bool | str, b: int | bool) -> str:
    return """<>{a - b}</>"""


@catalog.component()
def MulOp(a: int | bool | str, b: int | bool) -> str:
    return """<>{a * b}</>"""


@catalog.component()
def DivOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a / b}</>"""


@catalog.component()
def AddMany(a: int | bool | str, b: int | bool | str, c: int | bool | str) -> str:
    return """<>{a + b + c}</>"""


@catalog.component()
def NestedOperation(aa: str, bb: str) -> str:
    return """<AddOp a={aa} b={bb} />"""


@catalog.component()
def NestedExpression(aa: str, bb: str) -> str:
    return """<>{<AddOp a={aa} b={bb} />}</>"""


@catalog.component()
def FuncCall(a: int, b: int) -> str:
    return """<>{max(a, b)}</>"""


@catalog.component()
def FuncCall2(a: int, b: int) -> str:
    return """<>{my_max(a, b)}</>"""


@catalog.component()
def FuncCall3(a: int, b: int) -> str:
    return """<>{my_max2(a, b)}</>"""


catalog.function(max)


@catalog.function
def my_max(i: int, j: int):
    return max(i, j)


@catalog.function("my_max2")
def my_dummy_max(i: int, j: int):
    return max(i, j)


# --------------------------------------------------------------------------- #
# -                              tests                                      - #
# --------------------------------------------------------------------------- #


def test_types():
    assert Types(False, True, 2, "3", DummyNode(a="4")) == "false-true-2-3-<p>4</p>"


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(AddOp(4, 2), "6", id="add int"),
        pytest.param(AddOp(13, 5), "18", id="add int-2"),
        pytest.param(AddOp(True, 2), "3", id="add bool and int"),
        pytest.param(AddOp(True, False), "1", id="add true-false"),
        pytest.param(AddOp(False, False), "0", id="add false-false"),
        pytest.param(AddOp(True, True), "2", id="add true-true"),
        pytest.param(AddOp("1", "2"), "12", id="concat str"),
    ],
)
def test_add(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(SubOp(8, 2), "6", id="sub int"),
        pytest.param(SubOp(23, 5), "18", id="sub int-2"),
        pytest.param(SubOp(True, 2), "-1", id="sub bool and int"),
        pytest.param(SubOp(True, False), "1", id="sub true-false"),
        pytest.param(SubOp(False, False), "0", id="sub false-false"),
        pytest.param(SubOp(True, True), "0", id="sub true-true"),
    ],
)
def test_sub(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,args,expected",
    [
        pytest.param(AddOp, (4, "2"), "Invalid types for addition", id="add int-str"),
        pytest.param(SubOp, ("1", "2"), "Invalid types for subtraction", id="sub str"),
    ],
)
def test_type_error(component: Component, args: Any, expected: str):
    with pytest.raises(TypeError) as exc:
        component(*args)

    assert str(exc.value) == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(AddMany(1, 2, 3), "6", id="add int"),
    ],
)
def test_multiple_op(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(NestedOperation("1", "2"), "12", id="operation"),
        pytest.param(NestedExpression("1", "2"), "12", id="expression"),
    ],
)
def test_nested(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize("func", [FuncCall, FuncCall2, FuncCall3])
def test_call(func: Component):
    assert func("1", "2") == "2"
