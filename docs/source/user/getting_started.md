# Getting Started

There are two distinct grammars in XComponent: the markup language and the
expression language.

The markup is designed to build XNode, and the expression language is designed
for scripting purposes.

Let's begin with the markup.

## XComponent Markup

Markup is made up of object-typed `XNode`, and each `XNode` represents an element
in the markup DOM.

A component is a tree of XNode, and as a tree, it can have only one parent element.

The fragment `<>` (and `</>`) can be used to create components that return a list
of elements.

If a tag like `<div>` has not been declared as a component, it is rendered as is;
it is a final DOM element during the rendering.

As a convention, tags that are lowercase are not components, and component tags
are capitalized.

However, if a component is not rendered where it is expected, then it means that
it has not been declared. **XComponent does not look up whether a tag is
capitalized or lowercase.**

When a component has children, it is declared as a list of XNode. The children
are isolated from their parent, meaning that the context of an XNode is not inherited
from its parent; the parent must pass context as attributes.

There are three kinds of context:

- The catalog, where components and functions are registered. It is context-less,
  meaning that, like nodes, they are reused from one rendering to another as static.

- Then, during a rendering, a global context can be passed to have data shared
  by all the nodes without having to do what we call "props drilling"—passing variables
  from parent to child using attribute variables.
  The global context is not static; it is global for a given rendering.

- Properties which are local to a component. In that case, the variables of the
  component are purely local; they are the parameters of the function of the template,
  except the globals one, which has to be declared too.

### Building a web page

Usually, to build a web page, we start with a layout component that has dynamic
content.

The children are used to put the body, but extra parameters can also be used to
provide an XNode.

```python
from xcomponent import Catalog, XNode

catalog = Catalog()

@catalog.component
def Layout(head: XNode, children: XNode) -> str:
    return """
        <>
            <!DOCTYPE html>
            <html>
                <head>
                    {head}
                </head>
                <body>
                    {children}
                </body>
            </html>
        </>
    """
```

Note that we use a fragment here (the `<>`) because the DOCTYPE is also a node.
It appears once, in the layout, and that's all.

The head has been declared as a component that can be passed as an attribute.

So we can create a new component used for the head.

```python
@catalog.component
def HtmlHead(title: str, description: str) -> str:
    return """
        <>
            <title>{title}</title>
            <meta name="description" content={description}>
            <meta charset="UTF-8"/>
            <meta content="width=device-width, initial-scale=1" name="viewport"/>
        </>
    """
```

Now we can create our first web page:

```python
@catalog.component()
def HelloWebPage(title: str) -> str:
    return """
      <Layout head={<HtmlHead title={title} />} title={title}>
          <h1>Hello, world!</h1>
      </Layout>
    """
```

```{important}
The head attribute cannot be a markup; it must be a single closed tag-only
component.
This limitation is currently a design choice to avoid code complexity.
```

The title here must be passed to the layout in order for the layout to pass it
on to the HTML head component.

This code below will produce an error because the title of the HtmlHead
component will raise an UnboundLocalError.
Even if the expression is not Python, it raises a Python exception.

```jsx
{
  /* ❌ this is wrong */
}
<Layout head={<HtmlHead title={title} />}>
  <h1>Hello, world!</h1>
</Layout>;
```

```jsx
{
  /* ✅ this is correct */
}
<Layout head={<HtmlHead title={title} />} title={title}>
  <h1>Hello, world!</h1>
</Layout>;
```

In the React world, this is called "props drilling." Components are autonomous;
they don't share their states.

But, using XComponent, there is a special variable named "globals" that can be
used to break this rule.

### Rendering the page

To render this web page now, we have two options.

We can use the catalog.render method:

```python
catalog.render("<HelloWebPage title='my title'/>")
```

Or we can directly call the component function:

```python
HelloWebPage(title="my title")
```

Both will yield the same result.
There are two ways to trigger a rendering because calling the component's function
directly is cleaner and easier to write with proper type checking. However, if the
component has XNode parameters, this becomes untrue. The component will not
produce an XNode; it will produce a string.

```{note}
The resulting HTML from the rendering will have all whitespace characters removed.
There is no option to generate pretty HTML.
```

### Using globals

At the moment, to avoid props drilling, there is no solution like a hook context.
Instead, there is a special variable named "globals" that is available in any
component, just by declaring it as an argument.

```python
from xcomponent import Catalog, XNode

catalog = Catalog()

@catalog.component
def Layout(head: XNode, children: XNode) -> str:
    return """
        <>
            <!DOCTYPE html>
            <html>
                <head>
                    {head}
                </head>
                <body>
                    {children}
                </body>
            </html>
        </>
    """

@catalog.component
def HtmlHead(globals: Any) -> str:
    return """
        <>
            <title>{globals.title}</title>
            {
                if globals.description {
                    <meta name="description" content={globals.description}/>
                }
            }
            <meta charset="UTF-8"/>
        </>
    """

@catalog.component()
def HelloWebPage(globals: Any) -> str:
    return """
      <Layout head={<HtmlHead />}>
          <h1>Hello, world!</h1>
      </Layout>
    """

assert HelloWebPage(globals={"title": "my title", "description": ""}) == (
    "<!DOCTYPE html><html><head><title>my title</title>"
    '<meta charset="UTF-8"/>'
    '</head><body><h1>Hello, world!</h1></body></html>'
)
```

The description is passed as an empty string and is mandatory.
The access to `globals.description` will raise a KeyError if it is not present.

XComponent is still in its early stages and should offer solutions for this.
A good alternative would be to use **a type with dataclasses or pydantic for the
globals**, ensuring all declared fields have an appropriate default value and
proper documentation.

## XComponent Expression

Everything in the Markup that has been enclosed by curly braces is an expression.
An expression enables the dynamic rendering of a page, using variables,
`if` statements, `for` statements, `let` statements, and operators.

### Types

Variables in expressions are typed. They are converted from Python types to Rust
native types using pyo3.
For a list of simple types, they are freed from the GIL.
Other objects that are not simple types remain as Python types and function correctly.
Access to methods and properties is tied to the Python GIL.

Native Types:

- None
- str
- int
- bool
- UUID
- list
- dict

Every other type is kept as a Python type and can be consumed with all their methods.

String objects can be enclosed by double quotes or single quotes.

Boolean values are `true` and `false`, like in JavaScript, Rust, and many languages,
except Python.

None value is render has an empty string for a XNode, and is used to remove
the rendering of tag attributes.

### Functions

The catalog can be used to register functions that can be called from expressions.

```python
@catalog.function
def capitalize(text: str):
  return text.capitalize()

@catalog.component
def HelloWorld(name: str) -> str:
    return """<>{"Hello " + capitalize(name)}</>"""
```

When the functions are called, the simple types are cast to Python types.

````{node}

It is possible to call the str methods directly in the XComponent expression.

The capitalized function was a simple exemple for function explanation.

```python
@catalog.component
def HelloWorld(name: str) -> str:
    return """<>{"Hello " + name.capitalize()}</>"""
```

````

### List Index

To access an index in a list, the `[]` must be used.

```python
@catalog.component
def HelloWorld(names: list[str]) -> str:
    return """<>{"Hello " + names[0]}</>"""
```

### Dict and Object Attributes

To access a dictionary or an attribute of any Python object, the `.` must be used,
or the `[]` can be used like in JavaScript.

There is no distinction between accessing dictionary keys and object attributes.

```python
@catalog.component
def HelloWorld(names: dict[str, str]) -> str:
    return """<>{"Hello " + names['foo']}</>"""
```

### Operators

| Type | +      | -        | \*       | /      |
| ---- | ------ | -------- | -------- | ------ |
| bool | add    | subtract | multiply | divide |
| int  | add    | subtract | multiply | divide |
| str  | concat | n/a      | repeat   | n/a    |
| UUID | n/a    | n/a      | n/a      | n/a    |
| dict | n/a    | n/a      | n/a      | n/a    |
| list | n/a    | n/a      | n/a      | n/a    |
| any  | n/a    | n/a      | n/a      | n/a    |

Due to the nature of booleans in Python, bools are integers 0 and 1, so operations
between booleans and integers are permitted.

Strings can be multiplied by an integer, like in Python, to produce a repeated string.

#### Binary Operators

All types support Python truthy/falsey values.

The operators include `and` and `or`. The `not` operator is used to reverse a condition.

Also, Python functions can be registered to mitigate or implement complex binary operations.

#### Comparison Operators

| `==`   | `!=`       | `>=`                  | `<=`               | `>`          | `<`       |
| ------ | ---------- | --------------------- | ------------------ | ------------ | --------- |
| equals | not equals | greater than or equal | less than or equal | greater than | less than |

#### Priority

The priority of operators follows the mathematical order.
Multiplication and division have the highest priority, followed by addition and
subtraction, then greater than or equal to and less than or equal to, next are
equals and not equals, followed by the and operator, and finally the or operator.

The parenthesis, such as **( _condition_ )** can be used to override the priority.

### If Syntax

The `if` syntax is the Rust syntax.
It means that the binary expression is not enclosed by parentheses, and blocks
are enclosed by curly braces.

Example:

```jsx
{/* if statement without else */}
{ if elements > 0 { <Elements elements={elements} /> } }

{/* if statement with else */}
{ if elements > 0 { <Elements elements={elements} /> } else { <Empty/>} }
```

Blocks can contain other expressions or markup.

### For Syntax

The `for` syntax is the Rust syntax.

Example:

```jsx
{/* for statement applied to list */}
{for x in my_list { <Item item={x}/> }}

{/* for statement applied to Python dict */}
{for k in my_dict { <Item key={k} value={my_dict[k]}/>}}
```

Blocks can contain other expressions or markup.

````{important}
this is not python code, you can do
```jsx
{
 /* ❌ this is wrong */
 if x in my_list { <hr/> }
}
```
````

### Let Syntax

The `let` keywork let you declare new constant in the component.

The expression cannot have multiple statement separated by indentation or semicolum.
Since the expression language is minimal, an other expression can be used to display
a variable that has been set, or reuse it to do a condition.
The variable will be local to the component.
Says differently, the `let` keyword declare constant scoped by component.

The `let` syntax is like the Rust syntax.
It means that the binary expression is affected to the variable.

Example:

```jsx
{/* simple let statement */}
{ let x = 42 }
{/* display the value */}
{x}

{/* if statement with else */}
{let y = if (x >= 1) {"Yes"} else {"No"}}
{/* display the value */}
{y}
```
