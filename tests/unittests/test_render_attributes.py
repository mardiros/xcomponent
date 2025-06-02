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


@catalog.component
def Label(
    for_: str | None = None,
    class_: str | None = None,
) -> str:
    return """
        <label
            for={for_}
            class={class_}
            >
            { children }
        </label>
    """


@catalog.component
def Button(hx_vals: str) -> str:
    return """
    <button hx-vals={hx_vals}>Submit</button>
    """


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(catalog.render("<Form />"), "<form></form>", id="drop-none"),
        pytest.param(Form(), "<form></form>", id="render-component"),
        pytest.param(
            catalog.render("<Form><input/></Form>"),
            "<form><input/></form>",
            id="drop-none",
        ),
        pytest.param(Form("post"), '<form method="post"></form>', id="add-args"),
        pytest.param(
            Form(method="post"), '<form method="post"></form>', id="add-kwargs"
        ),
        pytest.param(
            catalog.render("<Form hx_target='/ajax'><input/></Form>"),
            '<form hx-target="/ajax"><input/></form>',
            id="forward hx_target",
        ),
        pytest.param(
            catalog.render("<Form hx-target='/ajax'><input/></Form>"),
            '<form hx-target="/ajax"><input/></form>',
            id="forward hx-target",
        ),
        # we don't test multiple attributes since rust hashmap are not ordered
        pytest.param(
            catalog.render("<Form><Label class='p-4'>Name:</Label></Form>"),
            '<form><label class="p-4">Name:</label></form>',
            id="forward class and for",
        ),
        pytest.param(
            catalog.render("<Form><Label for='name'>Name:</Label></Form>"),
            '<form><label for="name">Name:</label></form>',
            id="forward class and for",
        ),
    ],
)
def test_render_form(component: str, expected: str):
    assert component == expected


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(
            catalog.render("""<Button hx-vals='{"a":"A"}' />"""),
            """<button hx-vals='{"a":"A"}'>Submit</button>""",
            id="forward class and for",
        )
    ],
)
def test_render_double_quote_in_quote(component: str, expected: str):
    assert component == expected
