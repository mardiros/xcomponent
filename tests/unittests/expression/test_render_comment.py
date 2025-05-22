import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def UselessComponent() -> str:
    return """<>{/* I am the most useless component you ever see! */}</>"""


@catalog.component
def Two() -> str:
    return """<>{
        /* A boring comment that only product 2 */
        1 + 1
    }</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(UselessComponent(), "", id="comment only"),
        pytest.param(Two(), "2", id="comment with binary operation"),
    ],
)
def test_comments(component: str, expected: str):
    assert component == expected
