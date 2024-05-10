# Functions

Functions can be created with the `fn` keyword.

```frugurt
let f = fn (x) {
    x + 1
};

print(f(5)); // 6
```

Functions can take other functions as arguments, and they capture their scope.

```frugurt
let f = fn (x) {
    fn (y) {
        x + y
    }
};

print(f(5)(10)); // 15
```

They can also return other functions.
Function that takes function and returns the modified version of it is called
decorator.

```frugurt
let decorator = fn (func) {
    fn (x) {
        func(func(x))
    }
};

let f = fn (x) {
    x * 2
};

f = decorator(f);

print(f(5)); // 20
```

Functions can have named parameters. They must go after positional parameters.

```frugurt
let f = fn (x, y=1) {
    x + y
};

print(f(5)); // 6

print(f(5, 10)); // 15

print(f(y: 10, x: 5)); // 15
```

Named parameters can be computed using positional ones and other named parameters that go before them.

```frugurt
let f = fn (x, y=x + 5, z=x + y + 5) {
    x + y + z
};

print(f(5)); // 35
```

Functions can be curried, will talk about in the next chapter.