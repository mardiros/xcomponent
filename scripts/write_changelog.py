#!/usr/bin/env python3
import datetime
import xcomponent

__version__ = xcomponent.__version__

header = (
    f"## {__version__}  - " f" {datetime.datetime.now().date().isoformat()}"
)
with open("CHANGELOG.md.new", "w") as changelog:
    changelog.write(header)
    changelog.write("\n")
    changelog.write("\n")
    changelog.write("* please write here \n\n")
