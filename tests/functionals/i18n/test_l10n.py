from pathlib import Path
from typing import Callable
import pytest

from fastlife.adapters.xcomponent.catalog import catalog
from fastlife.service.translations import Localizer


@pytest.fixture
def globals():
    lczr = Localizer()
    with (Path(__file__).parent / "fr.mo").open("rb") as buf:
        lczr.register("mydomain", buf)
    return lczr.as_dict()


@catalog.component
def Gettext():
    return """<>{globals.gettext('The lazy dog')}</>"""


@catalog.component
def Dgettext():
    return """<>{globals.dgettext('mydomain', 'The lazy dog')}</>"""


@catalog.component
def Ngettext():
    return """<>{globals.ngettext('The lazy dog', 'The lazy dogs', 1)}</>"""


@catalog.component
def Dngettext():
    return """
        <>{globals.dngettext('mydomain', 'The lazy dog', 'The lazy dogs', 1)}</>
        """


@catalog.component
def Pgettext():
    return """<>{globals.pgettext('animal', 'The lazy dog')}</>"""


@catalog.component
def Dpgettext():
    return """<>{globals.dpgettext('mydomain', 'animal', 'The lazy dog')}</>"""


@catalog.component
def Npgettext():
    return """<>{globals.npgettext('animal', 'The lazy dog', 'The lazy dogs', 1)}</>"""


@catalog.component
def Dnpgettext():
    return """
        <>
            {
                globals.dnpgettext(
                    'mydomain',
                    'animal',
                    'The lazy dog',
                    'The lazy dogs',
                    1)
            }
        </>
        """


@pytest.mark.parametrize(
    "msg",
    [
        pytest.param("<Gettext/>", id="gettext"),
        pytest.param("<Dgettext/>", id="dgettext"),
        pytest.param("<Ngettext/>", id="ngettext"),
        pytest.param("<Dngettext/>", id="dngettext"),
        pytest.param("<Pgettext/>", id="pgettext"),
        pytest.param("<Dpgettext/>", id="dpgettext"),
        pytest.param("<Npgettext/>", id="npgettext"),
        pytest.param("<Dnpgettext/>", id="dnpgettext"),
    ],
)
def test_localize(msg: str, globals: dict[str, Callable[..., str]]):
    assert catalog.render(msg, globals=globals) == "Le chien fénéant"
