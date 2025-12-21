from collections.abc import Iterator
from tokenize import STRING, generate_tokens
from typing import Any, BinaryIO

from xcomponent import XNode
from xcomponent.xcore import (
    XElement,
    XExpression,
    XFragment,
    XNSElement,
    extract_expr_i18n_messages,
    parse_markup,
)

lineno = int
funcname = str
message = Any
comments = list[str]
ExtractionInfo = tuple[lineno, funcname, message, comments]


def extract_from_markup(node: XNode, offset: int) -> Iterator[ExtractionInfo]:
    match node.unwrap():
        case XFragment(children):
            for child in children:
                for nfo in extract_from_markup(child, offset):
                    yield nfo
        case XElement(_, attrs, children):
            for child in attrs.values():
                for nfo in extract_from_markup(child, offset):
                    yield nfo
            for child in children:
                for nfo in extract_from_markup(child, offset):
                    yield nfo
        case XNSElement(_, _, attrs, children):
            for child in attrs.values():
                for nfo in extract_from_markup(child, offset):
                    yield nfo
            for child in children:
                for nfo in extract_from_markup(child, offset):
                    yield nfo
        case XExpression(expr):
            try:
                msgs = extract_expr_i18n_messages(expr)
            except SyntaxError:
                # should log something here
                return
            for msg in msgs:
                match msg.funcname:
                    case "gettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            msg.message.message,
                            msg.comments,
                        )
                    case "dgettext":
                        # what I do with the domain here ?
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (msg.message.domain, msg.message.message),
                            msg.comments,
                        )
                    case "ngettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (msg.message.singular, msg.message.plural),
                            msg.comments,
                        )
                    case "dngettext":
                        # what I do with the domain here ?
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (
                                msg.message.domain,
                                msg.message.singular,
                                msg.message.plural,
                            ),
                            msg.comments,
                        )
                    case "pgettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (
                                msg.message.context,
                                msg.message.message,
                            ),
                            msg.comments,
                        )
                    case "dpgettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (
                                msg.message.domain,
                                msg.message.context,
                                msg.message.message,
                            ),
                            msg.comments,
                        )
                    case "npgettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (
                                msg.message.context,
                                msg.message.singular,
                                msg.message.plural,
                            ),
                            msg.comments,
                        )
                    case "dnpgettext":
                        yield (
                            offset + msg.lineno,
                            msg.funcname,
                            (
                                msg.message.domain,
                                msg.message.context,
                                msg.message.singular,
                                msg.message.plural,
                            ),
                            msg.comments,
                        )
                    case _:
                        ...
        case _:
            pass


def extract_xcomponent(
    fileobj: BinaryIO,
    keywords: list[str],
    comment_tags: list[str],
    options: dict[str, str],
) -> Iterator[ExtractionInfo]:
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
            try:
                markup = parse_markup(f"<>{value}</>")
            except ValueError:
                # should log something here
                continue
            for messageinfo in extract_from_markup(markup, lineno):
                yield messageinfo
