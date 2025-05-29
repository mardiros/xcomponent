"""Registry of XComponents."""

from functools import wraps
import inspect
from types import ModuleType
from typing import Callable, Any, overload

from xcomponent.xcore import (
    RenderContext,
    XCatalog,
    XNode,
)

__all__ = ["Component", "Function", "Catalog"]


Component = Callable[..., str]
"""
A component is a function that takes parameters which are the component parameters,
and always return a string, which is the template.

Then the callable has to be decorated with the [@catalog.component](#xcomponent.Catalog.component)
"""

Function = Callable[..., Any]
"""
A component is a function that takes parameters which are the component parameters,
and always return a string, which is the template.

Then the callable has to be decorated with the [@catalog.component](#xcomponent.Catalog.component)
"""


class Catalog:
    """
    Store all the components and functions to render templates.
    """

    def __init__(self) -> None:
        self.scanned: set[ModuleType] = set()
        self._catalog = XCatalog()

    def render(self, content: str, **params: Any) -> str:
        """
        Render the given markup.

        :param content: The markup to render
        :param params: Global variables that does not need to do props drilling.
        :return: the rendered template.
        """
        return self._catalog.render(content, **params)

    def register_component(
        self,
        component_name: str,
        component: Component,
    ) -> None:
        """
        Register a template.

        :param component_name: the name of the component.
        :param component: function called when a step in a scenario match the pattern.
        """
        signature = inspect.signature(component)

        kwargs: dict[str, Any] = {}
        parameters: dict[str, type | Any] = {}
        defaults: dict[str, Any] = {}

        for name, param in signature.parameters.items():
            kwargs[name] = None
            if param.default != inspect._empty:
                defaults[name] = param.default
            if param.annotation is not inspect.Parameter.empty:
                parameters[name] = param.annotation
            else:
                parameters[name] = Any

        template = component(**kwargs)
        self._catalog.add_component(component_name, template, parameters, defaults)

    @overload
    def component(self, name: Component) -> Component: ...

    @overload
    def component(self, name: str) -> Callable[[Component], Component]: ...

    def component(
        self, name: str | Component = ""
    ) -> Callable[[Component], Component] | Component:
        """
        Decorator to register a template with its schema parameters.

        :param name: optional name for the component, by default, it is the function name.
        :return: A function that render the component without global variable supports.
        """
        component_name = name.__name__ if isinstance(name, Callable) else name

        def decorator(fn: Component):
            @wraps(fn)
            def render(*args: Any, **kwargs: Any) -> str:
                template = self._catalog.get(component_name or fn.__name__)
                context = RenderContext()
                context.push(template.defaults)
                if args:
                    for i, key in enumerate(template.params.keys()):
                        if i < len(args):
                            kwargs[key] = args[i]
                        else:
                            break
                for key, typ in template.params.items():
                    if typ is XNode:
                        kwargs[key] = self._catalog.render(kwargs[key])

                context.push(kwargs)
                return self._catalog.render_node(template.node, context)

            self.register_component(component_name or fn.__name__, fn)
            return render

        if isinstance(name, Callable):
            return decorator(name)
        else:
            return decorator

    @overload
    def function(self, name: Function) -> Function: ...

    @overload
    def function(self, name: str) -> Callable[[Function], Function]: ...

    def function(self, name: str | Function = "") -> Function:
        """
        Decorator to register a template with its schema parameters.

        It can be used as a decorator ( `@catalog.function` )
        or a parametrized decorator ( `@catalog.function(name="registered_name")` )

        :param name: name of the function in case it is the parametrized function
        :return: the decorated method.
        """
        if isinstance(name, Callable):
            self._catalog.add_function(name.__name__, name)
            return name

        def decorator(fn: Function):
            self._catalog.add_function(name or fn.__name__, fn)
            return fn

        return decorator
