# Imports

Other files can be imported into your code by using the `import` expression.
Import expression returns the same scope object, which was mentioned in the previous chapter.

Example:

`main.fru`
```frugurt
let foo = import "foo.fru";

print(foo.f(1, 2)); // 3

// this is as badass as extremely stupid
scope foo {
    let wow = 5;

    print(omg()); // 5
}
```

`foo.fru`
```frugurt
let f = fn(x, y) {
    x + y
};

let omg = fn() { wow };
```
