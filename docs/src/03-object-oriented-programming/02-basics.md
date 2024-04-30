# Basics

Types come in 3 flavors:

- `struct` - mutable, copied by value
- `class` - mutable, copied by reference
- `data` - immutable, copied by reference

```frugurt
struct Vector {
    // fields can be marked public or/and annotated with type,
    // but this does not change semantics(now)
    x;
    pub y : Number;
}

let v = Vector:{ 5, 10 };

print(v); // Vector{x=5, pub y: Number=10}

let a = v;

a.x = 1;

print(v, a); // Vector{x=5, pub y: Number=10} Vector{x=1, pub y: Number=10}
```
