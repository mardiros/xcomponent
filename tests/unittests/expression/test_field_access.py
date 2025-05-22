from xcomponent import Catalog, XNode

catalog = Catalog()


class DummyObject:
    @property
    def type(self) -> str:
        return "Dummy"

    @property
    def title(self) -> str:
        return "a dummy title"


@catalog.component
def Article():
    return """
    <div>{dummy.type}</div>
    """


@catalog.component
def HtmlHead(title: str = "") -> str:
    """
    Component to render the root html head tag.
    """
    return """
        <head>
            <title>{title}</title>
        </head>
    """


@catalog.component
def Page(head: XNode):
    return """
    <html>{head}<body><Article /></body></html>
    """


# def test_render_property():
#     assert catalog.render("<Article/>", {"dummy": DummyObject()}) == "<div>Dummy</div>"


def test_render_nested_property():
    assert catalog.render(
        "<Page head={<HtmlHead title={dummy.title}/>}/>",
        globals={"dummy": DummyObject()},
    ) == (
        "<html><head><title>a dummy title</title></head>"
        "<body><div>Dummy</div></body></html>"
    )
