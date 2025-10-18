import pytest

from xcomponent import XNode
from xcomponent.adapters.babel import ExtractionInfo, extract_from_markup
from xcomponent.xcore import parse_markup


@pytest.fixture
def markup(raw: str):
    return parse_markup(f"<>{raw}</>")


empty_comment: list[str] = []


@pytest.mark.parametrize(
    "raw,expected",
    [
        pytest.param(
            "{globals.gettext('a small text')}",
            [(1, "gettext", "a small text", empty_comment)],
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
            [(1, "gettext", "a multiline text\n", empty_comment)],
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
            [(1, "gettext", "a small text", empty_comment)],
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
                (1, "gettext", "a small desc", empty_comment),
                (1, "gettext", "a small text", empty_comment),
            ],
            id="nested",
        ),
        pytest.param(
            """
            {
                globals.ngettext(
                    '''
                    a singular text
                    ''',
                    '''
                    a plural text
                    ''',
                )
            }
            """,
            [
                (
                    1,
                    "ngettext",
                    ("a singular text\n", "a plural text\n"),
                    empty_comment,
                ),
            ],
            id="ngettext",
        ),
        pytest.param(
            """
            {
                globals.dgettext(
                    'domain',
                    'multi domain extracted',
                )
            }
            """,
            [
                (1, "dgettext", ("domain", "multi domain extracted"), empty_comment),
            ],
            id="dgettext",
        ),
        pytest.param(
            """
            {
                globals.dngettext(
                    'domain',
                    'multi domain extracted',
                    'multi domain plural extracted',
                    42
                )
            }
            """,
            [
                (
                    1,
                    "dngettext",
                    (
                        "domain",
                        "multi domain extracted",
                        "multi domain plural extracted",
                    ),
                    empty_comment,
                ),
            ],
            id="dngettext",
        ),
        pytest.param(
            """
            {
                globals.pgettext(
                    'the go game. neigher the verb nor the programing language.',
                    'go',
                )
            }
            """,
            [
                (
                    1,
                    "pgettext",
                    (
                        "the go game. neigher the verb nor the programing language.",
                        "go",
                    ),
                    empty_comment,
                ),
            ],
            id="pgettext",
        ),
        pytest.param(
            """
            {
                globals.dpgettext(
                    'domain',
                    'the verb to go.',
                    'go',
                )
            }
            """,
            [
                (
                    1,
                    "dpgettext",
                    ("domain", "the verb to go.", "go"),
                    empty_comment,
                ),
            ],
            id="dpgettext",
        ),
        pytest.param(
            """
            {
                globals.npgettext(
                    "goat for the animal, not the greatest.",
                    "the {number} goat",
                    "the {number} goats",
                    number
                )
            }
            """,
            [
                (
                    1,
                    "npgettext",
                    (
                        "goat for the animal, not the greatest.",
                        "the {number} goat",
                        "the {number} goats",
                    ),
                    empty_comment,
                ),
            ],
            id="npgettext",
        ),
        pytest.param(
            """
            {
                globals.dnpgettext(
                    "domain",
                    "goat for the greatest of all times.",
                    "the {number} goat",
                    "the {number} goats",
                    number
                )
            }
            """,
            [
                (
                    1,
                    "dnpgettext",
                    (
                        "domain",
                        "goat for the greatest of all times.",
                        "the {number} goat",
                        "the {number} goats",
                    ),
                    empty_comment,
                ),
            ],
            id="dnpgettext",
        ),
    ],
)
def test_extract_from_markup(markup: XNode, expected: list[ExtractionInfo]):
    vals = list(extract_from_markup(markup, 1))
    assert vals == expected
