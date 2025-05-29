from typing import Literal
from xcomponent import Catalog, XNode

import pytest

catalog = Catalog()


@catalog.component
def Form(
    method: Literal["get", "post"] | None = None,
    action: str | None = None,
    hx_target: str | None = None,
    children: XNode | None = None,
) -> str:
    return """
        <form
            hx-target={hx_target}
            action={action}
            method={method}
            >
            { children }
        </form>
    """


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(catalog.render("<Form />"), "<form></form>", id="drop-none"),
        pytest.param(Form(), "<form></form>", id="drop-none"),
        pytest.param(
            catalog.render("<Form><input/></Form>"), "<form><input/></form>", id="drop-none"
        ),
    ],
)
def test_render_form(component: str, expected: str):
    assert component == expected
