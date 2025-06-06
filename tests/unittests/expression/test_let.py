import pytest

from xcomponent import Catalog


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component
    def DummyInt() -> str:
        return """<>{let a = 42}{a}</>"""

    @catalog.component
    def YesNo(a: int) -> str:
        return """<>{let b = if (a >= 1) {"Yes"} else {"No"}}{b}</>"""


@pytest.mark.parametrize(
    "template_string,expected",
    [
        pytest.param("<DummyInt />", "42", id="simple let"),
        pytest.param("<YesNo a={0} />", "No", id="let if"),
        pytest.param("<YesNo a={2} />", "Yes", id="let else"),
    ],
)
def test_nested(catalog: Catalog, template_string: str, expected: str):
    assert catalog.render(template_string) == expected
