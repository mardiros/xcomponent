import pytest

from xcomponent import Catalog


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component
    def ScriptInside() -> str:
        return """<><script>function onBtnClick(){alert('clicked');}</script>\
<div onclick="onBtnClick" /></>"""

    @catalog.component
    def StyleInside() -> str:
        return """<style>body {background-color: olive;}</style>"""


@pytest.mark.parametrize(
    "template_string,expected",
    [
        pytest.param(
            "<ScriptInside />",
            """<script>function onBtnClick(){alert('clicked');}"""
            """</script><div onclick="onBtnClick"/>""",
            id="script",
        ),
        pytest.param(
            "<StyleInside />",
            "<style>body {background-color: olive;}</style>",
            id="style",
        ),
    ],
)
def test_script_is_not_interpreted(
    catalog: Catalog, template_string: str, expected: str
):
    assert catalog.render(template_string) == expected
