from typing import Any
import pytest

from xcomponent import Catalog


@pytest.fixture()
def BadIndex(catalog: Catalog):
    @catalog.component
    def BadExcerpt(title: str) -> str:
        return "<li>{summary}</li>"

    @catalog.component
    def BadIndex(summaries: list[str]) -> str:
        return """
        <ul>
            {
                for summary in summaries {
                    <BadExcerpt title={summary} />
                }
            }
        </ul>
        """

    return BadIndex


@pytest.fixture()
def GoodIndex(catalog: Catalog):
    @catalog.component
    def GoodExcerpt(title: str) -> str:
        return "<li>{title}</li>"

    @catalog.component
    def GoodIndex(summaries: list[str]) -> str:
        return """
        <ul>
            {
                for summary in summaries {
                    <GoodExcerpt title={summary} />
                }
            }
        </ul>
        """

    return GoodIndex


def test_raises(BadIndex: Any):
    with pytest.raises(UnboundLocalError):
        assert BadIndex(summaries=["foo", "bar"])


def test_ok(GoodIndex: Any):
    assert GoodIndex(summaries=["foo", "bar"]) == "<ul><li>foo</li><li>bar</li></ul>"
