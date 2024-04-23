# Possible bugs

- `fru_clone()` horror

# TODO

- add `overload` keyword for functions and methods
- add trait polymorphism for native types
- make cd for windows and linux releases
- add derive and implicit derives, and make them overridable (main reason - equality of objects)

# Needed fixes

- move method creation from execution to parsing
- implement Display for FruValue and fix for FruObject
- remove of fix BACKWARDS_MAP in Identifier (can cause problems in testing with some probability)
- introduce new error type for "method/field not found"