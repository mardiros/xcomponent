import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def Eq(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a == b}</>"""


@catalog.component
def Neq(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a != b}</>"""


@catalog.component
def Gt(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a > b}</>"""


@catalog.component
def Lt(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a < b}</>"""


@catalog.component
def Gte(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a >= b}</>"""


@catalog.component
def Lte(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a <= b}</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Eq(4, 2), "false", id="int-false"),
        pytest.param(Eq(5, 5), "true", id="int-true"),
        pytest.param(Eq(True, 2), "false", id="bool and int-false"),
        pytest.param(Eq(True, 1), "true", id="bool and int-true"),
        pytest.param(Eq(False, 0), "true", id="bool and int-true"),
        pytest.param(Eq(True, False), "false", id="true-false"),
        pytest.param(Eq(False, False), "true", id="false-false"),
        pytest.param(Eq(True, True), "true", id="add true-true"),
        pytest.param(Eq("1", "2"), "false", id="str-false"),
        pytest.param(Eq("1", "1"), "true", id="str-true"),
    ],
)
def test_eq(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Neq(4, 2), "true", id="int-true"),
        pytest.param(Neq(5, 5), "false", id="int-false"),
        pytest.param(Neq(True, 2), "true", id="bool and int-true"),
        pytest.param(Neq(True, 1), "false", id="bool and int-false"),
        pytest.param(Neq(False, 0), "false", id="bool and int-false"),
        pytest.param(Neq(True, False), "true", id="true-false is true"),
        pytest.param(Neq(False, False), "false", id="false-false is false"),
        pytest.param(Neq(True, True), "false", id="add true-true is true"),
        pytest.param(Neq("1", "2"), "true", id="str-true"),
        pytest.param(Neq("1", "1"), "false", id="str-false"),
    ],
)
def test_neq(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Gt(4, 2), "true", id="int-true"),
        pytest.param(Gt(5, 5), "false", id="int-false"),
        pytest.param(Gt(5, True), "true", id="int-bool-true"),
        pytest.param(Gt(-5, True), "false", id="int-bool-false"),
        pytest.param(Gt(True, 0), "true", id="bool and int-true"),
        pytest.param(Gt(True, 1), "false", id="bool and int-false"),
        pytest.param(Gt(False, 2), "false", id="bool and int-false"),
        pytest.param(Gt(True, False), "true", id="false-true"),
        pytest.param(Gt(False, False), "false", id="false-false"),
        pytest.param(Gt(True, True), "false", id="add true-true"),
    ],
)
def test_gt(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Lt(2, 4), "true", id="int-true"),
        pytest.param(Lt(5, 5), "false", id="int-false"),
        pytest.param(Lt(-5, True), "true", id="int-bool-true"),
        pytest.param(Lt(5, True), "false", id="int-bool-false"),
        pytest.param(Lt(False, 1), "true", id="bool and int-true"),
        pytest.param(Lt(True, 1), "false", id="bool and int-false"),
        pytest.param(Lt(False, 2), "true", id="bool and int-true"),
        pytest.param(Lt(True, False), "false", id="false-true"),
        pytest.param(Lt(False, False), "false", id="false-false"),
        pytest.param(Lt(True, True), "false", id="add true-true"),
    ],
)
def test_lt(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Gte(4, 2), "true", id="int-true"),
        pytest.param(Gte(5, 5), "true", id="int-eq-true"),
        pytest.param(Gte(5, 6), "false", id="int-false"),
        pytest.param(Gte(5, True), "true", id="int-bool-true"),
        pytest.param(Gte(-5, True), "false", id="int-bool-false"),
        pytest.param(Gte(True, 0), "true", id="bool and int-true"),
        pytest.param(Gte(True, 1), "true", id="bool and int-false"),
        pytest.param(Gte(False, 2), "false", id="bool and int-false"),
        pytest.param(Gte(True, False), "true", id="false-true"),
        pytest.param(Gte(False, False), "true", id="false-false"),
        pytest.param(Gte(True, True), "true", id="add true-true"),
        pytest.param(Gte(False, True), "false", id="false-true"),
    ],
)
def test_gte(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(Lte(2, 4), "true", id="int-true"),
        pytest.param(Lte(5, 5), "true", id="int-false"),
        pytest.param(Lte(-5, True), "true", id="int-bool-true"),
        pytest.param(Lte(5, True), "false", id="int-bool-false"),
        pytest.param(Lte(False, 1), "true", id="bool and int-true"),
        pytest.param(Lte(True, 1), "true", id="bool and int-false"),
        pytest.param(Lte(False, 2), "true", id="bool and int-true"),
        pytest.param(Lte(True, False), "false", id="true-false"),
        pytest.param(Lte(False, False), "true", id="false-false"),
        pytest.param(Lte(True, True), "true", id="add true-true"),
        pytest.param(Lte(False, True), "true", id="false-true"),
    ],
)
def test_lte(component: str, expected: str):
    assert component == expected
