from xcomponent import Catalog


catalog = Catalog()


@catalog.component
def SidebarItem(title: str, route_name: str) -> str:
    return """
        <li><a href={route_path[route_name]}>{title}</a></li>
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
            {"route_path": {"account-settings": "/account/settings"}},
        )
        == '<li><a href="/account/settings">settings</a></li>'
    )


def test_render_globals_nested():
    assert catalog.render(
        "<Sidebar/>",
        {
            "route_path": {
                "home": "/",
                "account-settings": "/account/settings",
            }
        },
    ) == (
        '<ul><li><a href="/">home</a></li>'
        '<li><a href="/account/settings">settings</a></li></ul>'
    )
