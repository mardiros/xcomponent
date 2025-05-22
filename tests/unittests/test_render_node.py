import pytest
from xcomponent import Catalog, XNode

catalog = Catalog()


@catalog.component
def H1(title: str) -> str:
    return """<h1>{title}</h1>"""


@catalog.component
def H2(title: str) -> str:
    return """<h2>I - {title}</h2>"""


@catalog.component
def Section() -> str:
    return """<div><H1 title="hello"/><H2 title="world"/></div>"""


@catalog.component
def HtmlHead(title: str) -> str:
    return """
        <>
            <title>{title}</title>
            <meta charset="UTF-8"/>
        </>
    """


@catalog.component
def Details(summary: str, children: XNode, opened: bool = False):
    return """
        <details open={opened}>
            <summary>{summary}</summary>
            {children}
        </details>
    """


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


def test_render_h1():
    assert catalog.render('<H1 title="Hello, world!" />') == "<h1>Hello, world!</h1>"
    assert H1("Hello") == "<h1>Hello</h1>"


def test_render_h2():
    assert (
        catalog.render('<H2 title="Hello, world!" />') == "<h2>I - Hello, world!</h2>"
    )


def test_render_children():
    assert (
        catalog.render("<Section />") == "<div><h1>hello</h1><h2>I - world</h2></div>"
    )


def test_render_children_param():
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
    "component,expected",
    [
        pytest.param(
            catalog.render(
                """
                <Details summary="Click to expand" opened>
                    <div>I am in</div>
                </Details>
                """
            ),
            "<details open><summary>Click to expand</summary>"
            "<div>I am in</div></details>",
            id="true",
        ),
        pytest.param(
            catalog.render(
                """
                <Details summary="Click to expand" opened={false}>
                    <div>I am in</div>
                </Details>
                """
            ),
            "<details><summary>Click to expand</summary><div>I am in</div></details>",
            id="false",
        ),
        pytest.param(
            catalog.render(
                """
                <Details summary="Click to expand">
                    <div>I am in</div>
                </Details>
                """
            ),
            "<details><summary>Click to expand</summary><div>I am in</div></details>",
            id="default",
        ),
    ],
)
def test_render_bool_attr(component, expected):
    assert component == expected
