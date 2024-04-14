# Control Flow

The control flow in Frugurt can be implemented both using statements and expressions.



## Conditionals

### using statements
```frugurt
let age = 16;

if age < 12 {
    print("child");
} else if age < 18 {
    print("teenager"); // this branch is executed
} else {
    print("adult");
}
```


### using expressions
```frugurt
let age = 16;

print(
    // this big expression is evaluated to "teenager"
    // and then returned to print function
    if age < 12 {
        "child"
    } else if age < 18 {
        "teenager"
    } else {
        "adult"
    }
);
```

## Loops

There is only `while` loop statement in Frugurt for now

```frugurt
let i = 0;

while i < 10 {
    print(i); // 0 1 2 3 4 5 6 7 8 9
    i = i + 1;
}
```