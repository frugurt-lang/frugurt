# Scope keyword

Scope keyword can be used in three constructs:

- `scope()` - captures scope in which it was evaluated
- `scope s { statements... }` - run statements in specified scope
- `scope s { statements... expression }` - run statements in specified scope and return result of expression

Example:

```frugurt
let f = fn () {
    let a = 5;
    let b = 3;
    scope()
};

let scope_object = f();

print(scope_object.a); // 5

scope scope_object {
    // this statement is executed in the same scope as the body of function f ran
    // so the variables a and b are available here
    print(a * b); // 15
}

scope_object.a = 10; // old variables can be re-assigned
scope_object.c = 20; // new variables can be declared

print(scope scope_object {
    let r = a + c;
    r * b
}); // 90
```
