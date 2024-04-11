# Methods

You can access fields and methods by name, there is no `this` or `self` keyword.

```frugurt
struct Vector {
    x;
    y;

    -----impl-----

    rotate90() {
        let old_x = x;
        x = -1 * y;
        y = old_x;
    }

    rotate180() {
        rotate90();
        rotate90();
    }
}

let v = Vector:{ 4, 5 };

v.rotate90();

print(v); // Vector{x=-5, y=4}

v.rotate180();

print(v); // Vector{x=5, y=-4}
```