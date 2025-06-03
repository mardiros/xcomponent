from typing import Literal

from bs4.element import PageElement
from xcomponent import Catalog, XNode

import pytest


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
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

    @catalog.component
    def Radio(
        globals: dict[str, str],
        label: str,
        name: str,
        value: str,
        id: str | None = None,
        checked: bool = False,
        disabled: bool = False,
        onclick: str | None = None,
        div_class: str | None = None,
        class_: str | None = None,
        label_class: str | None = None,
    ) -> str:
        return """
        <div class={div_class or globals.RADIO_DIV_CLASS}>
            <input type="radio" name={name} id={id} value={value}
                class={class_ or globals.RADIO_INPUT_CLASS}
                onclick={onclick}
                checked={checked}
                disabled={disabled} />
            <Label for={id} class={label_class or globals.RADIO_LABEL_CLASS}>
                {label}
            </Label>
        </div>
        """


@pytest.mark.parametrize(
    "template_string,expected_string",
    [
        pytest.param("<Form />", "<form></form>", id="drop-none"),
        pytest.param(
            "<Form><input/></Form>",
            "<form><input/></form>",
            id="drop-none",
        ),
        pytest.param(
            "<Form hx_target='/ajax'><input/></Form>",
            '<form hx-target="/ajax"><input/></form>',
            id="forward hx_target",
        ),
        pytest.param(
            "<Form hx-target='/ajax'><input/></Form>",
            '<form hx-target="/ajax"><input/></form>',
            id="forward hx-target",
        ),
        # we don't test multiple attributes since rust hashmap are not ordered
        pytest.param(
            "<Form><Label class='p-4'>Name:</Label></Form>",
            '<form><label class="p-4">Name:</label></form>',
            id="forward class and for",
        ),
        pytest.param(
            "<Form><Label for='name'>Name:</Label></Form>",
            '<form><label for="name">Name:</label></form>',
            id="forward class and for",
        ),
    ],
)
def test_render_form(soup_rendered: PageElement, soup_expected: PageElement):
    assert soup_rendered == soup_expected


@pytest.mark.parametrize(
    "template_string,expected_string",
    [
        pytest.param(
            """<Button hx-vals='{"a":"A"}' />""",
            """<button hx-vals='{"a":"A"}'>Submit</button>""",
            id="forward class and for",
        )
    ],
)
def test_render_button(soup_rendered: PageElement, soup_expected: PageElement):
    assert soup_rendered == soup_expected


@pytest.mark.parametrize(
    "template_string,expected_string",
    [
        pytest.param(
            """
            <Radio name="n" value="v"
                label="lbl" class="radio" label-class="lbl" div-class="d" />
            """,
            '<div class="d"><input type="radio" name="n" class="radio" value="v"/>'
            '<label class="lbl">lbl</label></div>',
            id="dont alter suffixed by class",
        )
    ],
)
def test_render_radio(soup_rendered: PageElement, soup_expected: PageElement):
    assert soup_rendered == soup_expected
