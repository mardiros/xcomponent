from typing import Any
from xcomponent import Catalog, Component, XNode
import pytest


@pytest.fixture()
def HelloWebPage(catalog: Catalog):
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

    @catalog.component
    def HtmlHead(globals: Any) -> str:
        return """
            <>
                <title>{globals.title}</title>
                {if globals.description {<meta name="description" content={globals.description}/>}}
                <meta charset="UTF-8"/>
            </>
        """

    @catalog.component()
    def HelloWebPage(globals: Any) -> str:
        return """
        <Layout head={<HtmlHead />}>
            <h1>Hello, world!"</h1>
        </Layout>
        """

    return HelloWebPage


def test_catalog_render(HelloWebPage: Component):
    assert HelloWebPage(globals={"title": "my title", "description": ""}) == (
        "<!DOCTYPE html><html><head><title>my title</title>"
        '<meta charset="UTF-8"/>'
        '</head><body><h1>Hello, world!"</h1></body></html>'
    )
