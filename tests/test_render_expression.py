from xcomponent import Catalog, XNode

catalog = Catalog()


@catalog.component()
def AddInt(a: int, b: int) -> str:
    return """<>{a + b}</>"""


@catalog.component()
def AddManyInt(a: int, b: int, c: int) -> str:
    return """<>{a + b + c}</>"""


def test_add_int():
    assert AddInt(1, 2) == "3"
