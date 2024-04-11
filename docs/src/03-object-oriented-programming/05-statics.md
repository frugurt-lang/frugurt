# Statics

You can define shared fields and methods that are available to all instances of a type.

```frugurt
struct Vector {
    x;
    y;
    static scaler = 10;

    -----static-----

    scale(v) {
        Vector :{ v.x * scaler, v.y * scaler}
    }

    double_scaler() {
        scaler = scaler * 2;
    }
}

let v = Vector :{ 1, 2 };

print(Vector.scale(v)); // {x: 10, y: 20}

Vector.double_scaler();

print(Vector.scale(v)); // {x: 20, y: 40}
```
