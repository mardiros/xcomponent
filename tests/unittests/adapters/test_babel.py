import pytest

from xcomponent import XNode
from xcomponent.adapters.babel import ExtractionInfo, extract_from_markup
from xcomponent.xcore import parse_markup


@pytest.fixture
def markup(raw: str):
    return parse_markup(f"<>{raw}</>")


@pytest.mark.parametrize(
    "raw,expected",
    [
        pytest.param(
            "{globals.gettext('a small text')}",
            [(1, "", "a small text", "")],
            id="small",
        ),
        pytest.param(
            """
            {
                globals.gettext(
                    '''
                    a multiline text
                    '''
                )
            }
            """,
            [(1, "", "a multiline text\n", "")],
            id="multiline",
        ),
        pytest.param(
            """
            <div>
                <span>
                    {
                        globals.gettext(
                            '''a small text'''
                        )
                    }
                </span>
            </div>
            """,
            [(1, "", "a small text", "")],
            id="nested",
        ),
        pytest.param(
            """
            <div>
                <span aria-label={globals.gettext("a small desc")}>
                    {
                        globals.gettext(
                            '''a small text'''
                        )
                    }
                </span>
            </div>
            """,
            [
                (1, "", "a small desc", ""),
                (1, "", "a small text", ""),
            ],
            id="nested",
        ),
    ],
)
def test_extract_from_markup(markup: XNode, expected: list[ExtractionInfo]):
    vals = list(extract_from_markup(markup, 1))
    assert vals == expected
