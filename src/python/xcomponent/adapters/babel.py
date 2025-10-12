from collections.abc import Iterator
from tokenize import STRING, generate_tokens
from typing import BinaryIO

from xcomponent import XNode
from xcomponent.xcore import (
    XElement,
    XExpression,
    XFragment,
    extract_expr_i18n_messages,
    parse_markup,
)


lineno = int
funcname = str
message = str
comments = str
ExtractionInfo = tuple[lineno, funcname, message, comments]


def extract_from_markup(node: XNode, offset: int) -> Iterator[ExtractionInfo]:
    funcname = ""
    match node.unwrap():
        case XFragment(children):
            funcname = "fragment"
            for child in children:
                for nfo in extract_from_markup(child, offset):
                    yield nfo
        case XElement(name, attrs, children):
            funcname = name
            for child in attrs.values():
                for nfo in extract_from_markup(child, offset):
                    yield nfo
            for child in children:
                for nfo in extract_from_markup(child, offset):
                    yield nfo
        case XExpression(expr):
            msgs = extract_expr_i18n_messages(expr)
            for msg in msgs:
                yield offset, funcname, msg, ""
        case _:
            pass


def extract_xtemplate(
    fileobj: BinaryIO,
    keywords: list[str],
    comment_tags: list[str],
    options: dict[str, str],
) -> Iterator[tuple[int, str, str, str]]:
    """
    Extract messages from a xcomponent templates in python file.

    :param fileobj: the file-like object the messages should be extracted
                    from
    :param keywords: a list of keywords (i.e. function names) that should
                     be recognized as translation functions
    :param comment_tags: a list of translator tags to search for and
                         include in the results
    :param options: a dictionary of additional options (optional)
    :return: an iterator over ``(lineno, funcname, message, comments)``
             tuples
    :rtype: ``iterator``
    """

    encoding = options.get("encoding", "UTF-8")

    def next_line():
        return fileobj.readline().decode(encoding)

    tokens = generate_tokens(next_line)

    for tok, value, (lineno, _), _, _ in tokens:
        if tok == STRING:
            markup = parse_markup(f"<>{value}</>")
            for messageinfo in extract_from_markup(markup, lineno):
                yield messageinfo
