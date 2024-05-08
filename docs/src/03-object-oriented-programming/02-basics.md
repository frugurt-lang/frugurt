# Basics

Types come in 3 flavors:

- `struct` - mutable, copied by value
- `class` - mutable, copied by reference
- `data` - immutable, copied by reference

You can label the fields to make code more readable.
You can instantiate a class either by labeling all the fields or labeling none.
Labeling part of the fields is not allowed.

```frugurt
struct Vector {
    // fields can be marked public or/and annotated with type,
    // but this does not change semantics(now)
    x;
    pub y : Number;
}

let v = Vector:{ x: 5, y: 10 };

print(v); // Vector{x=5, pub y: Number=10}

// struct is copied by value, so `a` is not the same object as `v`
let a = v;

a.x = 1;

print(v, a); // Vector{x=5, pub y: Number=10} Vector{x=1, pub y: Number=10}

let v2 = Vector:{ x: 5, y: 10 };
// let v2 = Vector:{ 5, y: 10 }; // would throw an error
```
