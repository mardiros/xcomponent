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


@catalog.component
def Haha():
    return "<p>Ha ha!</p>"


@catalog.component
def IfSelfClosedComponent(a: bool) -> str:
    return """<>{if a { <Haha /> } }</>"""


@catalog.component
def IfComponent(a: bool) -> str:
    return """<>{if a { <p>Ha ha!</p> } }</>"""


@catalog.component
def Hint(text: str | None = None) -> str:
    return """
    <>
    {
        if text {
            <span>
                {text}
            </span>
        }
    }
    </>
    """


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
        pytest.param(IfNotElseStmt(0, "No", "Yes"), "<p>No</p>"),
        pytest.param(IfNotElseStmt(1, "No", "Yes"), "<p>Yes</p>"),
        pytest.param(IfNotElseStmt(42, "No", "Yes"), "<p>Yes</p>"),
        pytest.param(IfNotElseStmt("", "No", "Yes"), "<p>No</p>"),
        pytest.param(IfNotElseStmt("0", "No", "Yes"), "<p>Yes</p>"),
        pytest.param(IfNotElseStmt("that's it", "No", "Yes"), "<p>Yes</p>"),
        pytest.param(IfListStmt(["a"], "b"), "<p>a</p>"),
        pytest.param(IfListStmt([], "b"), "<p>b</p>"),
        pytest.param(IfSelfClosedComponent(True), "<p>Ha ha!</p>"),
        pytest.param(IfSelfClosedComponent(False), ""),
        pytest.param(IfComponent(True), "<p>Ha ha!</p>"),
        pytest.param(IfComponent(False), ""),
        pytest.param(Hint(), ""),
        pytest.param(Hint("I exist"), "<span>I exist</span>"),
    ],
)
def test_if(result: str, expected: str):
    assert result == expected
