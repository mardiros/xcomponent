from xcomponent.xcore import XCatalog, XElement, XExpression, XNode, XText


def H1(text: str) -> str:
    return '<h1 class="5xl">{text}</h1>'


def Modulo(x: int, y: int) -> str:
    return "<>{mod(x, y)}</>"


def test_component():
    catalog = XCatalog()
    catalog.add_component("H1", H1(""), {"text": str}, {})
    template = catalog.get("H1")
    assert template.node.unwrap() == XElement(
        name="h1",
        attrs={"class": XNode.Text(XText("5xl"))},
        children=[
            XNode.Expression(XExpression("text")),
        ],
    )
    assert template.params == {"text": str}

    assert catalog.render("<H1 text='Hello'/>") == '<h1 class="5xl">Hello</h1>'


def test_function():
    catalog = XCatalog()

    def modulo(x: int, y: int) -> int:
        return x % y

    catalog.add_function("mod", modulo)
    catalog.add_component("Modulo", Modulo(1, 1), {"x": int, "y": int}, {})
    assert catalog.render("<Modulo x={7} y={3} />") == "1"
