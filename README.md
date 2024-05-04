### Frugurt is an interpreted language, with focus on functional and OOP.

> It is proof-of-concept, showing interesting features in active development still

The main purpose of Frugurt is to present an entirely different approach to OOP,
compared to other languages like Python or JavaScript.

Example

```frugurt
let square = fn(x) {
    // if the last expression has no semicolon then it is returned
    x * x
};

let square_other = fn(x) {
    // or you can use return keyword
    return x * x;
};

// square_other is equivalent to square

print(square(7)); // 49

```

My main goal is to make objects strictly typed (not variables!).

All types have fixed schema, that means:

- All fields must be declared at once
- Any other fields can never be declared

Also, there are three flavors of types:

- `struct` - mutable, passed by value
- `class` - mutable, passed by reference
- `data` - immutable, passed by reference

There is also builtin data validation, using "watches",
see [docs](https://frugurt-lang.github.io/frugurt/03-object-oriented-programming/06-watches.html).

```frugurt
struct Vector {
    x;
    y;
} impl {
    static new(x, y) {
        return Vector:{ x, y };
    }

    add(other) {
        // fields are accessible like in complied languages
        // there are static fields too (see docs)
        Vector:{x + other.x, y + other.y }
    }
} constraints {
    watch (x) {
        if x < -1000 {
            x = -1000;
        }
        if x > 1000 {
            x = 1000;
        }
    }

    watch (y) {
        if y < -1000 {
            y = -1000;
        }
        if y > 1000 {
            y = 1000;
        }
    }
}

// you can define operator with any name you want!
operator <+> (v1 : Vector, v2 : Vector) {
    v1.add(v2) // no semicolon = return
}

let v1 = Vector.new(1, 2);
let v2 = Vector.new(3, 4);
let v3 = v1 <+> v2;

print(v3); // Vector{x=4, y=6}
```

[See docs for more details](https://leokostarev.github.io/frugurt-lang/)
