from typing import Any
import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component

catalog = Catalog()


@catalog.component
def AddOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a + b}</>"""


@catalog.component
def SubOp(a: int | bool | str, b: int | bool) -> str:
    return """<>{a - b}</>"""


@catalog.component
def MulOp(a: int | bool | str, b: int | bool) -> str:
    return """<>{a * b}</>"""


@catalog.component
def DivOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a / b}</>"""


@catalog.component
def AddMany(a: int | bool | str, b: int | bool | str, c: int | bool | str) -> str:
    return """<>{a + b + c}</>"""


@catalog.component
def ComponsedOp(a: int, b: int, c: int) -> str:
    return """<>{(a + b) * c}</>"""


@catalog.component
def ComponsedOp2(a: int, b: int, c: int) -> str:
    return """<>{c * (a + b) }</>"""


@catalog.component
def PriorityOp(a: int, b: int, c: int) -> str:
    return """<>{a + b*c}</>"""


@catalog.component
def PriorityOp2(a: int, b: int, c: int) -> str:
    return """<>{a*b + c}</>"""


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
    "component,expected",
    [
        pytest.param(MulOp(8, 2), "16", id="mul int"),
        pytest.param(MulOp(4, 5), "20", id="mul int"),
        pytest.param(MulOp(True, 2), "2", id="mul bool and int"),
        pytest.param(MulOp(True, False), "0", id="mul true-false"),
        pytest.param(MulOp(False, False), "0", id="mul false-false"),
        pytest.param(MulOp(True, True), "1", id="mul true-true"),
        pytest.param(MulOp("*", 5), "*****", id="mul int-str"),
        pytest.param(MulOp("+-", 3), "+-+-+-", id="mul int-str"),
        pytest.param(MulOp("+", -13), "", id="mul int-str"),
    ],
)
def test_mul(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(DivOp(8, 2), "4", id="mul int"),
        pytest.param(DivOp(2, 3), "0", id="mul int-2/3"),
        pytest.param(DivOp(True, 3), "0", id="mul bool and int"),
        pytest.param(DivOp(2, True), "2", id="mul true-true"),
    ],
)
def test_div(component: str, expected: str):
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
    "component,args,expected",
    [
        pytest.param(DivOp, (4, 0), "Division by zero", id="int"),
        pytest.param(DivOp, (1, False), "Division by zero", id="bool"),
    ],
)
def test_div_by_0(component: Component, args: Any, expected: str):
    with pytest.raises(ZeroDivisionError) as exc:
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
        pytest.param(ComponsedOp(1, 2, 3), "9", id="composed operation (a + b) * c"),
        pytest.param(ComponsedOp(2, 2, 3), "12", id="composed operation (a + b) * c"),
        pytest.param(ComponsedOp2(1, 2, 3), "9", id="composed operation c * (a + b) "),
        pytest.param(ComponsedOp2(2, 2, 3), "12", id="composed operation c * (a + b) "),
    ],
)
def test_precendence_op(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        # a + b*c
        pytest.param(PriorityOp(3, 2, 4), "11", id="composed operation"),
        # a*b + c
        pytest.param(
            PriorityOp2(3, 2, 4), "10", id="composed operation, multiply left"
        ),
    ],
)
def test_priority_op(component: str, expected: str):
    assert component == expected
