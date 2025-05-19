"""Registry of XComponents."""

from functools import wraps
import inspect
from types import ModuleType
from typing import Callable, Any

from xcomponent.xcore import (
    XCatalog,
    XNode,
)

VENUSIAN_CATEGORY = "xcomponent"

Component = Callable[..., str]


class Catalog:
    """Store all the handlers for gherkin action."""

    def __init__(self) -> None:
        self.scanned: set[ModuleType] = set()
        self._catalog = XCatalog()

    def render(self, content: str, params: dict[str, Any] | None = None) -> str:
        return self._catalog.render(content, params or {})

    def register_template(
        self,
        component_name: str,
        params: Component,
    ) -> None:
        """
        Register a template.

        :param name: the name of the template.
        :param handler: function called when a step in a scenario match the pattern.
        """
        signature = inspect.signature(params)

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

        template = params(**kwargs)
        self._catalog.add_component(component_name, template, parameters, defaults)

    def component(
        self, name: str | Callable[..., str] = ""
    ) -> Callable[[Component], Component] | Component:
        """
        Decorator to register a template with its schema parameters
        """
        component_name = name.__name__ if isinstance(name, Callable) else name

        def decorator(fn: Callable[..., str]):
            @wraps(fn)
            def render(*args, **kwargs):
                template = self._catalog.get(component_name or fn.__name__)
                if args:
                    for i, key in enumerate(template.params.keys()):
                        if i < len(args):
                            kwargs[key] = args[i]
                        else:
                            break
                for key, typ in template.params.items():
                    if typ is XNode:
                        kwargs[key] = self._catalog.render(kwargs[key], {})

                return self._catalog.render_node(template.node, kwargs, {})

            self.register_template(component_name or fn.__name__, fn)
            return render

        if isinstance(name, Callable):
            return decorator(name)
        else:
            return decorator

    def function(self, name: str | Callable[..., Any] = "") -> Callable[..., Any]:
        """
        Decorator to register a template with its schema parameters
        """
        if isinstance(name, Callable):
            self._catalog.add_function(name.__name__, name)
            return name

        def decorator(fn: Callable[..., str]):
            self._catalog.add_function(name or fn.__name__, fn)
            return fn

        return decorator
