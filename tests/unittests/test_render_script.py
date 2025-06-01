from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def Head():
    return """
    <head>
        <script src="/static/htmx.2.0.1.min.js"></script>
    </head>
    """


def test_render_script():
    assert (
        catalog.render("<Head />")
        == '<head><script src="/static/htmx.2.0.1.min.js"></script></head>'
    )
