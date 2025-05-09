from xcomponent.xcomponent import NodeType, XNode, parse_str


def test_parse_str_simple():
    res = parse_str("<h1>Hello World</h1>")
    assert res == XNode(
        tag="h1", children=[XNode(node_type=NodeType.Text, text="Hello World")]
    )


def test_parse_str_with_attrs():
    res = parse_str('<h1 class="heading">Hello World</h1>')
    assert res == XNode(
        tag="h1",
        attrs={"class": XNode(node_type=NodeType.Text, text="heading")},
        children=[XNode(node_type=NodeType.Text, text="Hello World")],
    )


def test_repr_with_attrs():
    res = parse_str('<h1 class="heading">Hello World</h1>')
    assert repr(res) == '<h1 class="heading">Hello World</h1>'
