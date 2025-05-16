from typing import Any
import pytest

from xcomponent import Catalog
from xcomponent.service.catalog import Component
from xcomponent.xcore import XNode

catalog = Catalog()


@catalog.component()
def Eq(a: int | bool | str, b: int | bool | str) -> str:
    return """<>{a == b}</>"""


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
