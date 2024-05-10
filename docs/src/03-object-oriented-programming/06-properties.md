# Properties

Properties are type members that externally look like fields, but internally behave like methods.

```frugurt
struct Vec {
    x;
    y;

    Length {
        get => (x * x + y * y) ** 0.5;
        set(new_length) {
            let k = new_length / Length;
            x = x * k;
            y = y * k;
        }
    }
}

let v = Vec :{ x: 3, y: 4 };
print(v.Length); // 5

v.Length = 1;

print(v); // Vec { x: 0.6, y: 0.8 }
```

In this example, Vec has a "property" Length, that is freely computable from other fields.
Like methods, properties can access fields, methods and other properties of the object.
`(new_length)` can be omitted, in which case the default identifier `value` is used.
Properties can be static.
Also, there is no need to implement `get` and `set` every time.

```frugurt
class Time {
    static time = 0;
    
    pub static Now {
        get { time } // this is equivalent to `get => time;`
    }
}

print(Time.Now);
```

In this example, imagine game engine.
Static field time is updated by game engine every frame, and public property `Now` can be used to obtain current time
on the user side.
