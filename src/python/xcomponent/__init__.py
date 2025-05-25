from importlib import metadata
from xcomponent.service.catalog import Catalog, Component, Function
from xcomponent.xcore import XNode

__all__ = ["Catalog", "Component", "Function", "XNode"]
__version__ = metadata.version("xcomponent")
