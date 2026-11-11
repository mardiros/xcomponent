from importlib import metadata

from xcomponent.adapters.babel import extract_xcomponent
from xcomponent.service.catalog import Catalog, Component, Function
from xcomponent.xcore import XNode

__all__ = ["Catalog", "Component", "Function", "XNode", "extract_xcomponent"]
__version__ = metadata.version("xcomponent")
