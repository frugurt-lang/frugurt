# Functions

Functions can be created with the `fn` keyword.

```frugurt
let f = fn (x) {
    x + 1
};

print(f(5)); // 6
```

Functions can take other functions as arguments and they capture their scope.

```frugurt
let f = fn (x) {
    fn (y) {
        x + y
    }
};

print(f(5)(10)); // 15
```

They can also return other functions. Function that takes function and returns the modified version of it is called decorator.

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

Functions can be curried, will talk about in in the next chapter.