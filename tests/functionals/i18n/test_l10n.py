from pathlib import Path
from typing import Callable

import pytest
from fastlife import Configurator, Settings, x_component
from fastlife.service.translations import Localizer
from xcomponent import Catalog


@pytest.fixture
def globals():
    lczr = Localizer()
    with (Path(__file__).parent / "fr.mo").open("rb") as buf:
        lczr.register("mydomain", buf)
    return lczr.as_dict()


@x_component()
def Gettext():
    return """<>{globals.gettext('The lazy dog')}</>"""


@x_component()
def Dgettext():
    return """<>{globals.dgettext('mydomain', 'The lazy dog')}</>"""


@x_component()
def Ngettext():
    return """<>{globals.ngettext('The lazy dog', 'The lazy dogs', 1)}</>"""


@x_component()
def Dngettext():
    return """
        <>{globals.dngettext('mydomain', 'The lazy dog', 'The lazy dogs', 1)}</>
        """


@x_component()
def Pgettext():
    return """<>{globals.pgettext('animal', 'The lazy dog')}</>"""


@x_component()
def Dpgettext():
    return """<>{globals.dpgettext('mydomain', 'animal', 'The lazy dog')}</>"""


@x_component()
def Npgettext():
    return """<>{globals.npgettext('animal', 'The lazy dog', 'The lazy dogs', 1)}</>"""


@x_component()
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


@pytest.fixture
def catalog():
    config = Configurator(Settings())
    config.include(".")
    return config.build_catalog()


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
def test_localize(catalog: Catalog, msg: str, globals: dict[str, Callable[..., str]]):
    assert catalog.render(msg, globals=globals) == "Le chien fénéant"
