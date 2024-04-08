use crate::tests::run;

#[test]
fn test_function1() {
    run(r#"
            let func = fn (x, y) {
                return x + y + {x * y};
            };
            
            let func_same = fn (x, y) {
                x + y + {x * y}
            };
            
            t_eq(func(1, 2), 5);
            t_eq(func_same(5, 5), 35);
        "#)
}

#[test]
fn test_function_decorator() {
    run(r#"
            let func = fn (x, y) {
                return x + y + {x * y};
            };
            
            let decorator = fn (func) {
                return fn (x, y) {
                    return func(x - 1, y + 1) + 1;
                };
            };
            
            func = decorator(func);
            
            t_eq(func(2, 1), 6);
            t_eq(func(6, 4), 36);
        "#)
}

#[test]
fn test_function_nested() {
    run(r#"
            let comp = fn(x) {
                let tr1 = fn(y) { x + y };
            
                let tr2 = fn(y) { x * y };
            
                tr2(tr1(1))
            };
            
            t_eq(comp(6), 42);
    "#)
}

#[test]
fn test_curry() {
    run(r#"
            let f = fn (a, b, c) {a + b + c};
            
            let g = f$(1);
            
            t_eq(g(2, 3), 6);
            
            t_eq(f(1, 2, 3), 6);
            
            t_eq(g$(2)(5), 8);
        "#)
}

#[test]
fn test_overall() {
    run(r#"
            let f = fn (a, b) {a + b};
            
            let dec = fn (func) {
                fn (w) { func$(w) }
            };
            
            let g = dec(f);
            
            t_eq(g(1)(2), 3);
        "#)
}
