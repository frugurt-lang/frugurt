# Watches

Fields can be verified using watches. They are essentially methods that run when a field is assigned.

```frugurt
struct Vector {
    x;
    y;

    -----constraints-----

    watch(x) {
        if x < -1000 {
            x = -1000;
        }

        if x > 1000 {
            x = 1000;
        }
    }

    watch(y) {
        if y < -1000 {
            y = -1000;
        }

        if y > 1000 {
            y = 1000;
        }
    }
}

let v = Vector:{ 4, 5 };

v.x = 5000;
v.y = -10000;

print(v); // Vector{x=1000, y=-1000}
```
