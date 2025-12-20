import pytest
from xcomponent import Catalog


@pytest.fixture
def base_catalog():
    base = Catalog()

    @base.component
    def H1(title: str) -> str:
        return "<h1 class='xl'>{title}</h1>"

    @base.component
    def H2(title: str) -> str:
        return "<h2>{title}</h2>"

    return base


@pytest.fixture
def page_catalog(base_catalog: Catalog) -> Catalog:
    page = Catalog()

    @page.component(use={"base": base_catalog})
    def Page(title: str) -> str:
        return "<html><base.H1>{title}</base.h1></html>"

    return page


def test_namespace(page_catalog: Catalog):
    page = page_catalog.render("<Page title='yolo'/>")
    assert page == '<html><h1 class="xl">yolo</h1></html>'
