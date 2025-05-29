import pytest

from xcomponent import Catalog

catalog = Catalog()


@catalog.component
def BadExcerpt(title: str) -> str:
    return "<li>{summary}</li>"


@catalog.component()
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


@catalog.component
def GoodExcerpt(title: str) -> str:
    return "<li>{title}</li>"


@catalog.component()
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


def test_raises():
    with pytest.raises(UnboundLocalError):
        assert BadIndex(summaries=["foo", "bar"])


def test_ok():
    assert GoodIndex(summaries=["foo", "bar"]) == "<ul><li>foo</li><li>bar</li></ul>"
