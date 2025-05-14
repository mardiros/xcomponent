from xcomponent import Catalog, XNode

catalog = Catalog()


@catalog.component()
def AddInt(a: int, b: int) -> str:
    return """<>{a + b}</>"""


@catalog.component()
def AddManyInt(a: int, b: int, c: int) -> str:
    return """<>{a + b + c}</>"""


@catalog.component()
def AddStr(a: str, b: str) -> str:
    return """<>{a + b}</>"""


@catalog.component()
def NestedOperation(aa: str, bb: str) -> str:
    return """<AddStr a={aa} b={bb} />"""


@catalog.component()
def NestedExpression(aa: str, bb: str) -> str:
    return """<>{<AddStr a={aa} b={bb} />}</>"""


def test_add_int():
    assert AddInt(1, 2) == "3"


def test_add_many_int():
    assert AddManyInt(1, 2, 3) == "6"


def test_add_str():
    assert AddStr("1", "2") == "12"


def test_nested_operation():
    assert NestedOperation("1", "2") == "12"


def test_nested_expression():
    assert NestedExpression("1", "2") == "12"
