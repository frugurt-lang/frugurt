# Currying

You can apply first n arguments to a function to obtain a new function that accepts the rest of the arguments. This is called currying. Curried can be curried as many times as you want.

```frugurt
let add = fn(a, b) {
    a + b
};

let add3 = add$(3);

print(add3(7), add(1, 2)); // 10 3

let five = add3$(2);

print(five()); // 5
```
