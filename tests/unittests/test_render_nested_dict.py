import pytest
from xcomponent import Catalog, Component


@pytest.fixture(autouse=True)
def HelloWorld(catalog: Catalog):
    @catalog.component
    def HelloWorld(person: dict[str, str]):
        return """
            <>Hello { person.nick or "World" }</>
        """

    return HelloWorld


def test_render_nested_dict(catalog: Catalog):
    assert (
        catalog.render("<HelloWorld person={person} />", person={"nick": ""})
        == "Hello World"
    )


def test_render_nested_dict_func(HelloWorld: Component):
    assert HelloWorld({"nick": ""}) == "Hello World"
