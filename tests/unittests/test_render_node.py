from xcomponent import Catalog, Component, XNode

import pytest
from bs4 import PageElement  # type: ignore


@pytest.fixture(autouse=True)
def H1(catalog: Catalog):
    @catalog.component
    def H1(title: str) -> str:
        return """<h1>{title}</h1>"""

    return H1


@pytest.fixture(autouse=True)
def H2(catalog: Catalog):
    @catalog.component
    def H2(title: str) -> str:
        return """<h2>I - {title}</h2>"""

    return H2


@pytest.fixture(autouse=True)
def Section(catalog: Catalog):
    @catalog.component
    def Section() -> str:
        return """<div><H1 title="hello"/><H2 title="world"/></div>"""

    return Section


@pytest.fixture(autouse=True)
def HtmlHead(catalog: Catalog):
    @catalog.component
    def HtmlHead(title: str) -> str:
        return """
            <>
                <title>{title}</title>
                <meta charset="UTF-8"/>
            </>
        """

    return HtmlHead


@pytest.fixture(autouse=True)
def Details(catalog: Catalog):
    @catalog.component
    def Details(summary: str, children: XNode, opened: bool = False):
        return """
            <details open={opened}>
                <summary>{summary}</summary>
                {children}
            </details>
        """

    return Details


@pytest.fixture(autouse=True)
def Layout(catalog: Catalog):
    @catalog.component
    def Layout(head: XNode, children: XNode) -> str:
        return """
            <>
                <!DOCTYPE html>
                <html>
                    <head>
                        {head}
                    </head>
                    <body>
                        {children}
                    </body>
                </html>
            </>
        """

    return Layout


def test_render_h1(catalog: Catalog):
    assert catalog.render('<H1 title="Hello, world!" />') == "<h1>Hello, world!</h1>"


def test_render_h1_function(H1: Component):
    assert H1("Hello") == "<h1>Hello</h1>"


def test_render_h2(catalog: Catalog):
    assert (
        catalog.render('<H2 title="Hello, world!" />') == "<h2>I - Hello, world!</h2>"
    )


def test_render_children(catalog: Catalog):
    assert (
        catalog.render("<Section />") == "<div><h1>hello</h1><h2>I - world</h2></div>"
    )


def test_render_children_param(catalog: Catalog, Layout: Component):
    # ensure we cam remder the HtmlHead before continuing
    assert catalog.render('<HtmlHead title="happy world" />') == (
        '<title>happy world</title><meta charset="UTF-8"/>'
    )

    result = (
        "<!DOCTYPE html><html><head>"
        '<title>happy world</title><meta charset="UTF-8"/></head>'
        "<body><h1>Hello, world!</h1></body></html>"
    )

    assert (
        catalog.render("""
            <Layout head={<HtmlHead title="happy world" />}>
                <H1 title="Hello, world!" />
            </Layout>
        """)
        == result
    )

    assert (
        Layout(
            head='<HtmlHead title="happy world" />',
            children='<H1 title="Hello, world!" />',
        )
        == result
    )


@pytest.mark.parametrize(
    "template_string,expected_string",
    [
        pytest.param(
            """
            <Details summary="Click to expand" opened>
                <div>I am in</div>
            </Details>
            """,
            "<details open><summary>Click to expand</summary>"
            "<div>I am in</div></details>",
            id="true",
        ),
        pytest.param(
            """
            <Details summary="Click to expand" opened={false}>
                <div>I am in</div>
            </Details>
            """,
            "<details><summary>Click to expand</summary><div>I am in</div></details>",
            id="false",
        ),
        pytest.param(
            """
            <Details summary="Click to expand">
                <div>I am in</div>
            </Details>
            """,
            "<details><summary>Click to expand</summary><div>I am in</div></details>",
            id="default",
        ),
    ],
)
def test_render_bool_attr(soup_rendered: PageElement, soup_expected: PageElement):
    assert soup_rendered == soup_expected
