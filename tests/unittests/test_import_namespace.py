import pytest
from xcomponent import Catalog, XNode


@pytest.fixture
def base_catalog():
    base = Catalog()

    @base.component
    def H1(title: str) -> str:
        return "<h1 class='xl'>{title}</h1>"

    @base.component
    def Content(children: XNode) -> str:
        return "<div>{children}</div>"

    return base


@pytest.fixture
def page_catalog(base_catalog: Catalog) -> Catalog:
    page = Catalog()

    @page.component(use={"base": base_catalog})
    def Page1(children: XNode, title: str) -> str:
        return "<html><base.H1 title={title} /></html>"

    @page.component(use={"base": base_catalog})
    def Page2(children: XNode, title: str) -> str:
        return (
            "<html><base.H1 title={title} />"
            "<base.Content>{children}</base.Content></html>"
        )

    return page


@pytest.mark.parametrize(
    "doc,expected",
    [
        pytest.param(
            "<Page1 title='yolo' />",
            '<html><h1 class="xl">yolo</h1></html>',
            id="attrs",
        ),
        pytest.param(
            "<Page2 title='yolo'>You only</Page2>",
            '<html><h1 class="xl">yolo</h1><div>You only</div></html>',
            id="children",
        ),
    ],
)
def test_namespace(page_catalog: Catalog, doc: str, expected: str):
    page = page_catalog.render(doc)
    assert page == expected
