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


@x_component(namespace="lib")
def Gettext():
    return """<>{globals.gettext('The lazy dog')}</>"""


@x_component(namespace="lib")
def Dgettext():
    return """<>{globals.dgettext('mydomain', 'The lazy dog')}</>"""


@x_component(namespace="lib")
def Ngettext():
    return """<>{globals.ngettext('The lazy dog', 'The lazy dogs', 1)}</>"""


@x_component(namespace="lib")
def Dngettext():
    return """
        <>{globals.dngettext('mydomain', 'The lazy dog', 'The lazy dogs', 1)}</>
        """


@x_component(namespace="lib")
def Pgettext():
    return """<>{globals.pgettext('animal', 'The lazy dog')}</>"""


@x_component(namespace="lib")
def Dpgettext():
    return """<>{globals.dpgettext('mydomain', 'animal', 'The lazy dog')}</>"""


@x_component(namespace="lib")
def Npgettext():
    return """<>{globals.npgettext('animal', 'The lazy dog', 'The lazy dogs', 1)}</>"""


@x_component(namespace="lib")
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


@x_component(namespace="app")
def App():
    return """
    <>
        <lib.Gettext/>
    </>
    """


@x_component(namespace="app")
def AppAttribute():
    return """
    <>
        <div aria-label={globals.pgettext("aria-label", "The lazy dog")}/>
    </>
    """


@pytest.fixture
def catalogs() -> dict[str, Catalog]:
    config = Configurator(Settings())
    config.include(".")
    return config.build_catalogs()


@pytest.fixture
def catalog(catalogs: dict[str, Catalog]) -> Catalog:
    return catalogs["lib"]


@pytest.fixture
def ns_catalog(catalogs: dict[str, Catalog]) -> Catalog:
    return catalogs["app"]


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


@pytest.mark.parametrize(
    "msg",
    [
        pytest.param("<App/>", id="gettext"),
    ],
)
def test_ns_localize(
    ns_catalog: Catalog, msg: str, globals: dict[str, Callable[..., str]]
):
    assert ns_catalog.render(msg, globals=globals) == "Le chien fénéant"


@pytest.mark.parametrize(
    "msg",
    [
        pytest.param("<AppAttribute/>", id="pgettext"),
    ],
)
def test_localize_attrs(
    ns_catalog: Catalog, msg: str, globals: dict[str, Callable[..., str]]
):
    assert (
        ns_catalog.render(msg, globals=globals)
        == '<div aria-label="Le chien fénéant"/>'
    )
