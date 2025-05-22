from typing import Any
from urllib.parse import urlencode
from xcomponent import Catalog


class Request:
    def route_path(self, route_name: str, /, **kwargs: Any) -> str:
        return f"/{route_name}?{urlencode(kwargs)}"


catalog = Catalog()


@catalog.component
def SidebarItem(title: str, route_name: str, globals: Any) -> str:
    return """
        <li><a href={globals.request.route_path(route_name, foo="bar")}>{title}</a></li>
    """


@catalog.component
def Sidebar() -> str:
    return """
        <ul>
            <SidebarItem title="home" route_name="home" />
            <SidebarItem title="settings" route_name="account-settings" />
        </ul>
    """


def test_render_globals():
    assert (
        catalog.render(
            '<SidebarItem title="settings" route_name="account-settings"/>',
            globals={"request": Request()},
        )
        == '<li><a href="/account-settings?foo=bar">settings</a></li>'
    )


def test_render_globals_nested():
    assert catalog.render(
        "<Sidebar/>",
        globals={"request": Request()},
    ) == (
        '<ul><li><a href="/home?foo=bar">home</a></li>'
        '<li><a href="/account-settings?foo=bar">settings</a></li></ul>'
    )
