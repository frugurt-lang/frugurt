# Variables and Types

These are primitive types in Frugurt, they can be assigned to variables.

- `Nah`
- `Number`
- `Bool`
- `String`

There are also `Function`s and custom types

## Nah

```frugurt
let x = nah;

print(x);
```

## Number

```frugurt
let x = 7;
let y = 3;

print(x + y, x * y); // 10 21
```

## Bool

```frugurt
let x = true;
let y = false;

print(x && y, x || y); // false true
```

## String

```frugurt
let x = "hello";
let y = "world";

print(x <> ", " <> y); // hello, world
```

## Function

```frugurt
let f = fn(x, y) {
    return x + y;
};

print(f(1, 2)); // 3
```

We will learn more about functions in the [corresponding section](https://leokostarev.github.io/frugurt-lang/02-common-concepts/05-functions.html).
