import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def IfStmt(a: bool, b: str) -> str:
    return """<p>{if a { b }}</p>"""


@catalog.component
def IfElseStmt(a: bool, b: str, c: str) -> str:
    return """<p>{ if a { b } else { c } }</p>"""


@pytest.mark.parametrize(
    "result,expected",
    [
        pytest.param(IfStmt(True, "Yes"), "<p>Yes</p>"),
        pytest.param(IfStmt(False, "Yes"), "<p></p>"),
        pytest.param(IfElseStmt(True, "Yes", "No"), "<p>Yes</p>"),
        pytest.param(IfElseStmt(False, "Yes", "No"), "<p>No</p>"),
    ],
)
def test_if(result: str, expected: str):
    assert result == expected
