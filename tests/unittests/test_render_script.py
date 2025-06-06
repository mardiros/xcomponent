import pytest
from xcomponent import Catalog


@pytest.fixture(autouse=True)
def Head(catalog: Catalog):
    @catalog.component
    def Head():
        return """
        <head>
            <script src="/static/htmx.2.0.1.min.js"></script>
        </head>
        """

    return Head


def test_render_script(catalog: Catalog):
    assert (
        catalog.render("<Head />")
        == '<head><script src="/static/htmx.2.0.1.min.js"></script></head>'
    )
