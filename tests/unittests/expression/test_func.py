import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component

catalog = Catalog()


@catalog.component
def FuncCall(a: int, b: int) -> str:
    return """<>{max(a, b)}</>"""


@catalog.component
def FuncCall2(a: int, b: int) -> str:
    return """<>{my_max(a, b)}</>"""


@catalog.component
def FuncCall3(a: int, b: int) -> str:
    return """<>{my_max2(a, b)}</>"""


@catalog.component
def FuncCall4(a: int, b: int) -> str:
    return """<>{my_max(i=a, j=b)}</>"""


catalog.function(max)


@catalog.function
def my_max(i: int, j: int):
    return max(i, j)


@catalog.function("my_max2")
def my_dummy_max(i: int, j: int):
    return max(i, j)


@catalog.function
def lower(s: str):
    return s.lower()


@catalog.function
def upper(s: str):
    return s.upper()


@catalog.component
def HelloWorld(name: str) -> str:
    return """<>{lower("HELLO " + name)}</>"""


@catalog.component
def HelloWorld2(name: str) -> str:
    return """<>{lower("Hello ") + name}</>"""


@catalog.component
def HelloWorld3(name: str) -> str:
    return """<>{"Hello " + lower(name)}</>"""


@catalog.component
def HelloWorld4(name: str) -> str:
    return """<>{upper("Hello " + lower(name))}</>"""


@pytest.mark.parametrize("func", [FuncCall, FuncCall2, FuncCall3, FuncCall4])
def test_call(func: Component):
    assert func(1, 2) == "2"


@pytest.mark.parametrize(
    "rendered,expected",
    [
        pytest.param(HelloWorld("WORLD"), "hello world", id="binary"),
        pytest.param(HelloWorld2("WORLD"), "hello WORLD", id="left"),
        pytest.param(HelloWorld3("WORLD"), "Hello world", id="right"),
        pytest.param(HelloWorld4("WORLD"), "HELLO WORLD", id="nested"),
    ],
)
def test_call_in_expression(rendered: str, expected: str):
    assert rendered == expected
