import pytest
from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def HelloWorld(person: dict[str, str]):
    return """
        <>Hello { person.nick or "World" }</>
    """


@pytest.mark.parametrize(
    "component",
    [
        catalog.render("<HelloWorld person={person} />", person={"nick": ""}),
        HelloWorld({"nick": ""}),
    ],
)
def test_render_nested_dict(component: str):
    assert component == "Hello World"
