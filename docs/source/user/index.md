# XComponent

## What Is XComponent

XComponent is a template engine, inspired by JSX, to embed template in Python.

It diverge from all existing Python template engine since all the templates
must be written in Python code.

This is a design decision and a matter of preference for the locality of behavior.


**Hello world example:**

```python

from xcomponent import Catalog

catalog = Catalog()


@catalog.component()
def HelloWorld(name: str = "world") -> str:
    return """<p>Hello {name}</p>"""

HelloWorld(name)
# will render <p>Hello Bob</p>

catalog.render("<HelloWorld name='Bob'/>")
# will render <p>Hello Bob</p>
```

## How it works

Using XComponent, templates are recorded in a catalog of component, and then
they can be rendered to HTML.

All components can be reused in other component in order to build an html tree at the
end.


Using curly brace let's have a rich expression language, this is neither Python or
JSX syntax, but it is user friendly.

[Getting started ?](./getting_started.md)
