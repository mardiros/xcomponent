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

