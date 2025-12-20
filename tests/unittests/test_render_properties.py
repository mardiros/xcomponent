from dataclasses import dataclass

import pytest
from xcomponent import Catalog


@dataclass
class Model:
    name: str


@dataclass
class Dummy:
    model: Model

    @property
    def name(self) -> str:
        return self.model.name


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component
    def DummyComponent(
        dummy: Dummy,
    ) -> str:
        return """
            <div>
                { dummy.name }
            </div>
        """

    return DummyComponent


def test_render_property(catalog: Catalog):
    soup_rendered = catalog.render(
        "<DummyComponent dummy={dummy}/>", dummy=Dummy(model=Model(name="foobar"))
    )
    assert soup_rendered == "<div>foobar</div>"
