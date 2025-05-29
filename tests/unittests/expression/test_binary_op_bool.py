import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def AndOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a and b}</>"""


@catalog.component
def OrOp(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a or b}</>"""


@catalog.component
def ParenthesisOp(a: int | bool | str, b: bool, c: int | bool | str) -> str:
    return """<>{a or (b and c)}</>"""


@catalog.component
def NestedParenthesisOp(a: bool, b: bool, c: bool, d: bool) -> str:
    return """<>{a or (b and (c or d))}</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(AndOp(8, 2), "2", id="and int"),
        pytest.param(AndOp(2, 8), "8", id="and int"),
        pytest.param(AndOp(3, 0), "0", id="and int"),
        pytest.param(AndOp(3, False), "false", id="and int bool"),
        pytest.param(AndOp(3, ""), "", id="and int str"),
        pytest.param(AndOp(0, 8), "0", id="and int"),
        pytest.param(AndOp(3, -3), "-3", id="and int"),
        pytest.param(AndOp(True, False), "false", id="bool"),
        pytest.param(AndOp(False, True), "false", id="bool"),
        pytest.param(AndOp(True, False), "false", id="bool"),
        pytest.param(AndOp(False, False), "false", id="bool"),
    ],
)
def test_and(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(OrOp(8, 2), "8", id="and int"),
        pytest.param(OrOp(2, 8), "2", id="and int"),
        pytest.param(OrOp(3, 0), "3", id="and int"),
        pytest.param(OrOp(3, False), "3", id="and int bool"),
        pytest.param(OrOp(3, ""), "3", id="and int str"),
        pytest.param(OrOp(0, 8), "8", id="and int"),
        pytest.param(OrOp(3, -3), "3", id="and int"),
        pytest.param(OrOp(True, False), "true", id="bool"),
        pytest.param(OrOp(False, True), "true", id="bool"),
        pytest.param(OrOp(True, False), "true", id="bool"),
        pytest.param(OrOp(False, False), "false", id="bool"),
    ],
)
def test_or(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(ParenthesisOp("Yes", False, "No"), "Yes", id="Yes"),
        pytest.param(ParenthesisOp("", True, "Bis"), "Bis", id="Bis"),
        pytest.param(ParenthesisOp(False, False, False), "false", id="false"),
        pytest.param(
            NestedParenthesisOp(False, True, False, True), "true", id="nested-true"
        ),
        pytest.param(
            NestedParenthesisOp(False, True, False, False), "false", id="nested-false"
        ),
    ],
)
def test_composed_or(component: str, expected: str):
    assert component == expected
