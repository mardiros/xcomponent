from xcomponent import Catalog
from xcomponent.xcore import XNode

catalog = Catalog()


@catalog.component()
def DummyNode(a: int) -> str:
    return """<p>{a}</p>"""


@catalog.component()
def Types(a: bool, b: bool, c: int, d: str, e: XNode) -> str:
    return """<>{a}-{b}-{c}-{d}-{e}</>"""


def test_types():
    assert Types(False, True, 2, "3", DummyNode(a="4")) == "false-true-2-3-<p>4</p>"
