import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def IfStmt(a: bool, b: str) -> str:
    return """<p>{if a { b }}</p>"""


@catalog.component
def IfElseStmt(a: bool, b: str, c: str) -> str:
    return """<p>{ if a { b } else { c } }</p>"""


@catalog.component
def IfNotStmt(a: bool, b: str) -> str:
    return """<p>{ if not a { b } }</p>"""


@catalog.component
def IfNotElseStmt(a: bool, b: str, c: str) -> str:
    return """<p>{ if not a { b } else { c } }</p>"""


@catalog.component
def IfListStmt(a: list[str], b: str) -> str:
    return """<p>{if a { for x in a { a } } else { b } }</p>"""


@pytest.mark.parametrize(
    "result,expected",
    [
        pytest.param(IfStmt(True, "Yes"), "<p>Yes</p>"),
        pytest.param(IfStmt(False, "Yes"), "<p></p>"),
        pytest.param(IfElseStmt(True, "Yes", "No"), "<p>Yes</p>"),
        pytest.param(IfElseStmt(False, "Yes", "No"), "<p>No</p>"),
        pytest.param(IfNotStmt(False, "Yes"), "<p>Yes</p>"),
        pytest.param(IfNotStmt(True, "Yes"), "<p></p>"),
        pytest.param(IfNotElseStmt(True, "No", "Yes"), "<p>Yes</p>"),
        pytest.param(IfNotElseStmt(False, "No", "Yes"), "<p>No</p>"),
        pytest.param(IfListStmt(["a"], "b"), "<p>a</p>"),
        pytest.param(IfListStmt([], "b"), "<p>b</p>"),
    ],
)
def test_if(result: str, expected: str):
    assert result == expected
