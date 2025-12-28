## 0.11.0  -  2025-12-28

* Upgrade pyo3
  * Drop python 3.9 support, add python 3.14 support. 

## 0.10.4  -  2025-12-22

* Fix lazy evuation of binary operator (and and or).
  before this version, right arm branch was also evaluated leading to bugs,
  and requires to add nested if statement to avoid them.

## 0.10.3  -  2025-12-21

* Do not alter default templates parameter values. 

## 0.10.2  -  2025-12-21

* Fix i18n extraction for namescape components. 

## 0.10.1  -  2025-12-20

* Fix namespace components for expressions.
  Refactor to track all namespaces in a dedicated stack.

## 0.10.0  -  2025-12-20

* Introducing namespace components.
  Now the catalog can include components from another catalog
  in order to render itself from a cross catalog reference.
  the "use" parameter while registering a component is used to
  declare the reference to another catalog.
  the use parameter is a dict where the key is the namespace name
  to use in the template, and, the value is the catalog containing
  the components. to render a component Section from an ui catalog,
  the `<ui.Section />` component is available using `use={"ui": ui_ctaalog}`.

## 0.9.0  -  2025-11-16

* Improve UUID support.
  * Implement equality comparison
  * Render uuid with hyphen
  * Downcast uuid to python while running into_py

## 0.8.4  -  2025-10-19

* Improve more messages extraction from nested fragment. 

## 0.8.3  -  2025-10-18

* Improve more messages extraction from nested expressions. 

## 0.8.2  -  2025-10-18

* Be tolerant in case of markup or expression error while extracting locales. 

## 0.8.1  -  2025-10-18

* Fix babel entrypoint to find xcomponent messages extraction method.

## 0.8.0  -  2025-10-18

* Implement extractor for babel in order to get easy i18n support. 

## 0.7.0  -  2025-10-10

* Release triple quote string with auto dedent.

## 0.6.10  -  2025-10-05

* Fix truthy comparison for any python objects. 

## 0.6.9  -  2025-10-04

* Drop wheel for i686 for windows 11 for python 3.9,3.10 and 3.11 since
  it does not build anymore.
* Update python setup to v6.

## 0.6.8  -  2025-10-04

* Authorize trailing comma on function call
* Improve error display on markup error. (Display the component name)

## 0.6.7  -  2025-09-20

* Fix behavior for None == None and None != None. 

## 0.6.6  -  2025-09-12

* Add support of None for == operator.
* Improve error message for binary operators.

## 0.6.5  -  2025-09-09

* Fix string quote that are wrongly evaluated and eaten by the parser. 

## 0.6.4  -  2025-09-01

* Fix expression that starts with a string 

## 0.6.3  -  2025-08-30

* Implement not operator for str and int 

## 0.6.2  -  2025-08-15

* Fix nested var in if stmt 

## 0.6.1  -  2025-06-13

* Authorize full markup inside statement instead of self closed element only.
* Upgrade sphinx version. 

## 0.6.0  -  2025-06-06

* Add a let keyword in order to set variable in expression.
* Add support of string methods on expression. 

## 0.5.2  -  2025-06-03

* Hotfix component havin attributes'name containing python reserved keyword. 

## 0.5.1  -  2025-06-02

* Preserve empty string in markup such as stript tag to render.
* Fix rendering of json in attributes such as hx-vals.
* Check mypy in the CI.

## 0.5.0  -  2025-05-31

* Improve html attributes supports.
  * hyphen can be used in markup language and is bound
    to underscore variable in python.
  * attribute for is bound to for_.
  * attribute class is bount to class_.

## 0.4.1  -  2025-05-30

* Fix nested dict key 

## 0.4.0  -  2025-05-30

* Add support of None type.
* Fix default values that must never mutate.
* Fix default values while rendering using function call.

## 0.3.2  -  2025-05-29

* Authorize hiphen in attribute names 

## 0.3.1  -  2025-05-27

* Fix PyPI classifiers 

## 0.3.0  -  2025-05-27

* Implement `not` operator.
* Fix binary operation precedence.
* Switch documentation from mkdocs to sphinx.

## 0.2.1  -  2025-05-24

* Deploy a documentation 

## 0.2.0  -  2025-05-22

* Breaking changes: Now the catalog.render takes a kwargs for parameters, 
  and, its not global parameters, to have global parameters, do a globals
  named keyword arguments.
* Breaking changes: Now, to access to global variable in a component,
  the function must declared a parameter named 'globals', that will
  received the globals context from the catalog.render keyword arguments.
* Now any components can be rendered by calling the function, since the
  signature of the functions must contains global for global variables.

## 0.1.10  -  2025-05-22

* Implement comments in expression using /* this is a comment */ 
* Bug fix on PyAny evaluation while props drilling

## 0.1.9  -  2025-05-21

* Remove debug logging 

## 0.1.8  -  2025-05-21
* Fix postfix usage on right paremeter of binary expressions
* Authorize expression inside function 

## 0.1.7  -  2025-05-20

* Fix PyAny object serialization passed from parent to child element. 

## 0.1.6  -  2025-05-19

* Fix default values
* Add support of @catalog.component without parameter.

## 0.1.5  -  2025-05-18

* Add support of dict, list, objects, indexes, attributes, method calls.
* Add support of globals.
* Add support of boolean attributes

## 0.1.4  -  2025-05-16

* Publish to pypi using uv 
* Fix description and classifiers 

## 0.1.2  -  2025-05-16

* Initial release 

