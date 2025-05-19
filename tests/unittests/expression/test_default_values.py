import pytest
from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def HelloWorld(name: str = "world") -> str:
    return """<p>Hello {name}</p>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(
            "<HelloWorld />",
            "<p>Hello world</p>",
            id="default_args",
        ),
    ],
)
def test_types(component: str, expected: str):
    assert catalog.render(component) == expected
