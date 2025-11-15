from dataclasses import dataclass
from typing import TypedDict
from uuid import UUID
from xcomponent import Catalog
from xcomponent.xcore import XNode

import pytest

catalog = Catalog()


class User(TypedDict):
    username: str


class Product(TypedDict):
    owner: User


@catalog.component
def DummyNode(a: int) -> str:
    return """<p>{a}</p>"""


@catalog.component
def Types(a: bool, b: bool, c: int, d: str, e: UUID, f: XNode) -> str:
    return """<>{a}-{b}-{c}-{d}-{e}-{f}</>"""


@catalog.component
def DictComplexType(u: User) -> str:
    return """<>{u.username}</>"""


@catalog.component
def NestedDictComplexType(product: Product) -> str:
    return """<>{product.owner.username}</>"""


@catalog.component
def DynamicKeyDictComplexType(
    products: dict[UUID | int, Product], product_id: UUID | int
) -> str:
    return """<>{products[product_id].owner.username}</>"""


@dataclass
class UserModel:
    username: str


@dataclass
class ProductModel:
    user_id: UUID


@catalog.component
def DynamicKeyDictComplexType2(
    users: dict[UUID, UserModel], product: ProductModel
) -> str:
    return """<>{users[product.user_id].username}</>"""


@catalog.component
def DynamicKeyListComplexType(users: list[User], user_id: int) -> str:
    return """<>{users[user_id].username}</>"""


@pytest.mark.parametrize(
    "component,expected",
    [
        pytest.param(
            Types(False, True, 2, "3", UUID(int=4), DummyNode(a="5")),
            "false-true-2-3-00000000-0000-0000-0000-000000000004-<p>5</p>",
            id="simpletypes",
        ),
        pytest.param(
            DictComplexType({"username": "bob"}),
            "bob",
            id="dict",
        ),
        pytest.param(
            NestedDictComplexType(Product(owner=User(username="alice"))),
            "alice",
            id="nested-dict",
        ),
        pytest.param(
            DynamicKeyDictComplexType(
                products={
                    UUID(int=1): Product(owner=User(username="alice")),
                    UUID(int=2): Product(owner=User(username="bob")),
                    UUID(int=3): Product(owner=User(username="bernard")),
                },
                product_id=UUID(int=3),
            ),
            "bernard",
            id="key-dict-uuid",
        ),
        pytest.param(
            DynamicKeyDictComplexType(
                products={
                    1: Product(owner=User(username="alice")),
                    2: Product(owner=User(username="bob")),
                    3: Product(owner=User(username="bernard")),
                },
                product_id=3,
            ),
            "bernard",
            id="key-dict-int",
        ),
        pytest.param(
            DynamicKeyListComplexType(
                users=[
                    User(username="alice"),
                    User(username="bob"),
                    User(username="bernard"),
                ],
                user_id=2,
            ),
            "bernard",
            id="key-dict-int",
        ),
        pytest.param(
            DynamicKeyListComplexType(
                users=[
                    User(username="alice"),
                    User(username="bob"),
                    User(username="bernard"),
                ],
                user_id=-1,
            ),
            "bernard",
            id="key-dict-int",
        ),
        pytest.param(
            DynamicKeyDictComplexType2(
                users={
                    UUID(int=1): User(username="alice"),
                    UUID(int=2): User(username="bob"),
                    UUID(int=3): User(username="bernard"),
                },
                product=ProductModel(user_id=UUID(int=3)),
            ),
            "bernard",
            id="nested-attribute-with-dataclass",
        ),
    ],
)
def test_types(component: str, expected: str):
    assert component == expected
