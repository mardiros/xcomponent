from xcomponent import Catalog, XNode

catalog = Catalog()


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
def HtmlHead(title: str, description: str = "") -> str:
    return """
        <>
            <title>{title}</title>
            {if description {<meta name="description" content={description}/>}}
            <meta charset="UTF-8"/>
        </>
    """


@catalog.component()
def HelloWebPage(title: str) -> str:
    return """
      <Layout head={<HtmlHead title={title} />} title={title}>
          <h1>Hello, world!"</h1>
      </Layout>
    """


def test_catalog_render():
    assert catalog.render("<HelloWebPage title='my title'/>") == (
        "<!DOCTYPE html><html><head><title>my title</title>"
        '<meta charset="UTF-8"/>'
        '</head><body><h1>Hello, world!"</h1></body></html>'
    )


def test_render_component():
    assert HelloWebPage(title="my title") == (
        "<!DOCTYPE html><html><head><title>my title</title>"
        '<meta charset="UTF-8"/>'
        '</head><body><h1>Hello, world!"</h1></body></html>'
    )
