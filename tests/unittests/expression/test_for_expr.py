import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def Item(name: str) -> str:
    return """<li>{name}</li>"""


@catalog.component
def ForStmt(lst: list[str]) -> str:
    return """<ul>{for x in lst { <Item name={x} /> }}</ul>"""


@pytest.mark.parametrize(
    "result,expected",
    [
        pytest.param(ForStmt(["a", "b"]), "<ul><li>a</li><li>b</li></ul>"),
    ],
)
def test_for(result: str, expected: str):
    assert result == expected
