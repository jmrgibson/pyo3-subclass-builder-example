# pyo3-subclass-builder-example

An example of a builder pattern using subclasses implemented in pyo3. For example

```py
b = BaseClass().foo().bar()
b.do_things()

s = SubClass().foo().bar()  # inherited from BaseClass
s.do_specific_thing()
```
