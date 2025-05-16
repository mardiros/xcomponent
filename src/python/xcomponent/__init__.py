from importlib import metadata
from xcomponent.service.catalog import Catalog
from xcomponent.xcore import XNode

__all__ = ["Catalog", "XNode"]
__version__ = metadata.version("xcomponent")
