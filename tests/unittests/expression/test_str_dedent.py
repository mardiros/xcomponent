from xcomponent import Catalog

import pytest
import textwrap


@pytest.fixture(autouse=True)
def components(catalog: Catalog):
    @catalog.component
    def TripleSingleQuote() -> str:
        return """
            <>
                {
                    '''
                    The lazy dog jumped over the quick brown fox.
                       The lazy dog jumped over the quick brown fox.
                          The lazy dog jumped over the quick brown fox.
                    '''
                }
            </>
            """

    @catalog.component
    def TripleDoubleQuote() -> str:
        return '''
            <>
                {
                    """
                    The lazy dog jumped over the quick brown fox.
                       The lazy dog jumped over the quick brown fox.
                          The lazy dog jumped over the quick brown fox.
                    """
                }
            </>
            '''

    @catalog.component
    def SafeDedent() -> str:
        return """
            <>
                {
                    '''The lazy dog jumped over the quick brown fox.
                    The lazy dog jumped over the quick brown fox.
                       The lazy dog jumped over the quick brown fox.
                    '''
                }
            </>
            """

    @catalog.component
    def RevertDedent() -> str:
        return """
            <>
                {
                    '''
                          The lazy dog jumped over the quick brown fox.
                       The lazy dog jumped over the quick brown fox.
                    The lazy dog jumped over the quick brown fox.
                    '''
                }
            </>
            """

    return TripleSingleQuote, TripleDoubleQuote, SafeDedent, RevertDedent


@pytest.mark.parametrize(
    "template_string,expected",
    [
        pytest.param(
            "<TripleSingleQuote />",
            textwrap.dedent(
                """\
                The lazy dog jumped over the quick brown fox.
                   The lazy dog jumped over the quick brown fox.
                      The lazy dog jumped over the quick brown fox.
                """
            ),
            id="single",
        ),
        pytest.param(
            "<TripleDoubleQuote />",
            textwrap.dedent(
                """\
                The lazy dog jumped over the quick brown fox.
                   The lazy dog jumped over the quick brown fox.
                      The lazy dog jumped over the quick brown fox.
                """
            ),
            id="double",
        ),
        pytest.param(
            "<SafeDedent />",
            """The lazy dog jumped over the quick brown fox.
                    The lazy dog jumped over the quick brown fox.
                       The lazy dog jumped over the quick brown fox.
                    """,
            id="safe",
        ),
        pytest.param(
            "<RevertDedent />",
            textwrap.dedent(
                """\
                      The lazy dog jumped over the quick brown fox.
                   The lazy dog jumped over the quick brown fox.
                The lazy dog jumped over the quick brown fox.
                """
            ),
            id="revert",
        ),
    ],
)
def test_render_dedent_str(catalog: Catalog, template_string: str, expected: str):
    assert catalog.render(template_string) == expected
