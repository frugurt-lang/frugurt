# Possible bugs

- `fru_clone()` horror

# TODO

- add function named parameters, introduce ArgumentList
    - add `overload` keyword for functions and methods
- add trait polymorphism for native types
    - move FruValue::Object and FruValue::Type to FruValue:NativeObject
    - add collections
        - list
        - set
        - map
        - tuple?
- implement scope manipulation features
    - implement modules and imports
- implement "evil" features
- make cd for windows and linux releases
- add derivation and implicit derivation, and make them overridable (the main reason is equality of objects)

# Needed fixes

- get rid of `Nah` in `Control`
- move method creation from execution to parsing
- implement Display for FruValue and fix for FruObject
- remove or fix BACKWARDS_MAP in Identifier (fails testing with some probability)
- introduce a new error type for "method/field not found"

# Possible improvements

- introduce new wrapper extensions like `wrap_ok`, `wrap_err`, `wrap_value`? Needs lots of thinking.
