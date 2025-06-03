import re
from collections.abc import Iterator, Mapping

import pytest
from bs4 import BeautifulSoup, PageElement  # type: ignore

from xcomponent import Catalog


@pytest.fixture()
def catalog() -> Catalog:
    return Catalog()


@pytest.fixture()
def globals() -> Mapping[str, str]:
    return {
        "RADIO_DIV_CLASS": "RADIO_DIV_CLASS",
        "RADIO_INPUT_CLASS": "RADIO_INPUT_CLASS",
        "RADIO_LABEL_CLASS": "RADIO_LABEL_CLASS",
    }


@pytest.fixture()
def soup_rendered(
    catalog: Catalog, template_string: str, globals: Mapping[str, str]
) -> Iterator[PageElement]:
    rendered = catalog.render(template_string.strip(), globals=globals)
    soup = BeautifulSoup(rendered, "html.parser")
    try:
        yield next(soup.children)  # type: ignore
    except Exception as exc:
        # Display the error in the assertion is a compromise
        yield exc  # type: ignore


@pytest.fixture()
def soup_expected(expected_string: str) -> PageElement:
    expected_string = re.sub(r">\s+<", "><", expected_string).strip()
    expected_soup = BeautifulSoup(expected_string, features="html.parser")
    return next(expected_soup.children)  # type: ignore
