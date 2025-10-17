from importlib import metadata
from xcomponent.service.catalog import Catalog, Component, Function
from xcomponent.xcore import XNode
from xcomponent.adapters.babel import extract_xtemplate

__all__ = ["Catalog", "Component", "Function", "XNode", "extract_xtemplate"]
__version__ = metadata.version("xcomponent")
