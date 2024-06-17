# Operators

You can define any operator between any two types(even builtin ones).

```frugurt
struct Vector {
    x;
    y;
}

operator + (a : Vector, b : Vector) {
    Vector:{
        a.x + b.x,
        a.y + b.y
    }
}

operator += (a : Vector, b : Vector) {
    a.x = a.x + b.x;
    a.y = a.y + b.y;
}

commutative operator * (k : Number, b : Vector) {
    Vector:{
        k * b.x,
        k * b.y
    }
}

let a = Vector :{ 1, 2 };
let b = Vector :{ 3, 4 };

print(a + b); // Vector{x=4, y=6}
print(a * 2, 2 * a); // Vector{x=2, y=4} Vector{x=2, y=4}

a += b;

print(a); // Vector{x=4, y=6}
```

Operator precedences from highest to lowest:

- All custom operators
- `**` `<>`
- `*` `/` `%`
- `+` `-`
- `<` `>` `<=` `>=`
- `==` `!=`
- `&&`
- `||`

All operators are left associative
