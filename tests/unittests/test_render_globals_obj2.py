from dataclasses import dataclass
from typing import Any
from typing import Mapping
from xcomponent import Catalog, Component
import pytest


@dataclass
class Page:
    title: str
    summary: str


@pytest.fixture(autouse=True)
def Home(catalog: Catalog):
    @catalog.component
    def Excerpt(page: Page) -> str:
        return """
        <div>
            <h2>{page.title}</h2>
            <div>
                {page.summary}
            </div>
        </div>
        """

    @catalog.component
    def Home(globals: Mapping[str, Any]) -> str:
        return """
            <div>
            {
            for page in globals.pages {
                <Excerpt page={page} />
            }
            }
            </div>
        """

    return Home


def test_render_attrs_from_globals(Home: Component):
    rendered = Home(
        globals={
            "pages": [
                Page(title="foo", summary="This is foo"),
                Page(title="bar", summary="This is bar"),
            ]
        }
    )

    assert rendered == (
        "<div><div><h2>foo</h2><div>This is foo</div></div>"
        "<div><h2>bar</h2><div>This is bar</div></div></div>"
    )
