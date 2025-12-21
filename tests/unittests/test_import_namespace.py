from dataclasses import dataclass
from typing import Any

import pytest
from xcomponent import Catalog, XNode


@dataclass
class App:
    name: str


@pytest.fixture
def base_catalog():
    base = Catalog()

    @base.component
    def Header(title: str) -> str:
        return "<h1 class='xl'>{title}</h1>"

    @base.component
    def Content(children: XNode) -> str:
        return "<div>{children}</div>"

    return base


@pytest.fixture
def layout_catalog(base_catalog: Catalog):
    layout = Catalog()

    @layout.component
    def Header(title: str) -> str:
        return """
            <head>
                <title>{title}</title>
            </head>
            """

    @layout.component(use={"base": base_catalog})
    def Html(children: XNode, title: str) -> str:
        return """
            <html>
                <Header title={title}/>
                <body>
                    <base.Header title={title} />
                    <base.Content>{children}</base.Content>
                </body>
            </html>
            """

    @layout.component(use={"base": base_catalog})
    def Layout(children: XNode, title: str, side_bar: XNode) -> str:
        return """
            <Html title={title}>
                <aside>{side_bar}</aside>
                <div>{children}</div>
            </Html>
        """

    return layout


@pytest.fixture
def app_catalog() -> Catalog:
    catalog = Catalog()

    @catalog.component
    def Sidebar() -> str:
        return """<menu><li><a href="#">Parameters</a></li></menu>"""

    return catalog


@pytest.fixture
def page_catalog(
    base_catalog: Catalog, layout_catalog: Catalog, app_catalog: Catalog
) -> Catalog:
    page = Catalog()

    @page.component(use={"base": base_catalog})
    def Page1(children: XNode, title: str) -> str:
        return "<html><base.Header title={title} /></html>"

    @page.component(use={"base": base_catalog})
    def Page2(children: XNode, title: str) -> str:
        return (
            "<html><base.Header title={title} />"
            "<base.Content>{children}</base.Content></html>"
        )

    @page.component(use={"layout": layout_catalog})
    def Page3(children: XNode, title: str) -> str:
        return "<layout.Html title={title}>{children}</layout.Html>"

    @page.component(use={"layout": layout_catalog, "app": app_catalog})
    def Page4(children: XNode, title: str) -> str:
        return """
            <layout.Layout
                title={title}
                side_bar={<app.Sidebar/>}
                >
                {children}
            </layout.Layout>
        """

    @page.component(use={"layout": layout_catalog, "app": app_catalog})
    def Page5(children: XNode, title: str, apps: list[App]) -> str:
        return """
            <layout.Layout
                title={title}
                side_bar={<app.Sidebar/>}
                >
                {
                    for app in globals.apps {
                        <p>{app.name}</p>
                    }
                }
            </layout.Layout>
        """

    @page.component
    def P(children: XNode) -> str:
        return "<p>{children}</p>"

    @page.component(use={"layout": layout_catalog, "app": app_catalog})
    def Page6(children: XNode, title: str, apps: list[App]) -> str:
        return """
            <layout.Layout
                title={title}
                side_bar={<app.Sidebar/>}
                >
                {
                    for app in globals.apps {
                        <P>{app.name}</P>
                    }
                }
            </layout.Layout>
        """

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
        pytest.param(
            "<Page3 title='yolo'>You only</Page3>",
            "<html><head><title>yolo</title></head>"
            '<body><h1 class="xl">yolo</h1><div>You only</div></body></html>',
            id="nested",
        ),
        pytest.param(
            "<Page4 title='yolo'>You only</Page4>",
            "<html><head><title>yolo</title></head>"
            '<body><h1 class="xl">yolo</h1><div><aside><menu>'
            '<li><a href="#">Parameters</a></li></menu></aside>'
            "<div>You only</div></div></body></html>",
            id="nested-expression",
        ),
    ],
)
def test_namespace(page_catalog: Catalog, doc: str, expected: str):
    page = page_catalog.render(doc)
    assert page == expected


@pytest.mark.parametrize(
    "doc,params,expected",
    [
        pytest.param(
            "<Page5 title='yolo'/>",
            {"globals": {"apps": [App(name="foo"), App(name="bar")]}},
            '<html><head><title>yolo</title></head><body><h1 class="xl">yolo</h1>'
            '<div><aside><menu><li><a href="#">Parameters</a></li></menu></aside>'
            "<div><p>foo</p><p>bar</p></div></div></body></html>",
            id="override-local-name",
        ),
        pytest.param(
            "<Page6 title='yolo'/>",
            {"globals": {"apps": [App(name="foo"), App(name="bar")]}},
            '<html><head><title>yolo</title></head><body><h1 class="xl">yolo</h1>'
            '<div><aside><menu><li><a href="#">Parameters</a></li></menu></aside>'
            "<div><p>foo</p><p>bar</p></div></div></body></html>",
            id="override-local-name-with-component",
        ),
    ],
)
def test_namespace_params(page_catalog: Catalog, doc: str, params: Any, expected: str):
    page = page_catalog.render(doc, **params)
    assert page == expected
