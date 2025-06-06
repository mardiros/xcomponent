from xcomponent import Catalog

import pytest


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component
    def Dummy() -> str:
        return '<>{"dummy-str".replace("-", "_")}</>'

    return Dummy


@pytest.mark.parametrize(
    "template_string,expected",
    [
        pytest.param("<Dummy />", "dummy_str", id="replace"),
    ],
)
def test_render_nested_property(catalog: Catalog, template_string: str, expected: str):
    assert catalog.render(template_string) == expected
