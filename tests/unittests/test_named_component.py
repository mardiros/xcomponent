import pytest

from xcomponent import Catalog


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component(name="HelloWorld1")
    def hello_world(name="world"):
        return "<>Hello {name}</>"

    @catalog.component()
    def HelloWorld2(name="world"):
        return "<>Hello {name}</>"

    @catalog.component
    def HelloWorld3(name="world"):
        return "<>Hello {name}</>"


@pytest.mark.parametrize(
    "markup",
    [
        pytest.param("<HelloWorld1/>", id="explicit name"),
        pytest.param("<HelloWorld2/>", id="called implicit"),
        pytest.param("<HelloWorld3/>", id="implicit"),
    ],
)
def test_named_component(catalog: Catalog, markup: str):
    assert catalog.render(markup) == "Hello world"
