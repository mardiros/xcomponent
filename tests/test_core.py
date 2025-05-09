from xcomponent.xcore import XCatalog, XElement, XExpression, XNode, XText


def H1(text: str) -> str:
    return '<h1 class="5xl">{text}</h1>'


def test_catalog():
    catalog = XCatalog()
    catalog.register("H1", H1(""), {"text": str})
    template = catalog.get("H1")
    assert template.node.unwrap() == XElement(
        name="h1",
        attrs={"class": XNode.Text(XText("5xl"))},
        children=[
            XNode.Expression(XExpression("text")),
        ],
    )
    assert template.params == {"text": str}

    assert catalog.render("<H1 text='Hello'/>", {}) == '<h1 class="5xl">Hello</h1>'
