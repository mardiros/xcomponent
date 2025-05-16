import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component

catalog = Catalog()


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


@pytest.mark.parametrize("func", [FuncCall, FuncCall2, FuncCall3])
def test_call(func: Component):
    assert func("1", "2") == "2"
