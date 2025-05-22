from dataclasses import dataclass
from xcomponent import Catalog

catalog = Catalog()


@dataclass
class Page:
    title: str
    summary: str


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
def Home() -> str:
    return """
        <div>
        {
          for page in pages {
            <Excerpt page={page} />
          }
        }
        </div>
    """


def test_render_attrs_from_globals():
    rendered = catalog.render(
        "<Home/>",
        {
            "pages": [
                Page(title="foo", summary="This is foo"),
                Page(title="bar", summary="This is bar"),
            ]
        },
    )
    assert rendered == (
        "<div><div><h2>foo</h2><div>This is foo</div></div>"
        "<div><h2>bar</h2><div>This is bar</div></div></div>"
    )
