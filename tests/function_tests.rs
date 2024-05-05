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
            
            assert_eq(func(1, 2), 5);
            assert_eq(func_same(5, 5), 35);
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
            
            assert_eq(func(2, 1), 6);
            assert_eq(func(6, 4), 36);
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
            
            assert_eq(comp(6), 42);
    "#)
}

#[test]
fn test_curry() {
    run(r#"
            let f = fn (a, b, c) {a + b + c};
            
            let g = f$(1);
            
            assert_eq(g(2, 3), 6);
            
            assert_eq(f(1, 2, 3), 6);
            
            assert_eq(g$(2)(5), 8);
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
            
            assert_eq(g(1)(2), 3);
        "#)
}

#[test]
fn test_named() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            assert_eq(f(), 4);
            assert_eq(f(3), 8);
            assert_eq(f(3, 4), 10);
            assert_eq(f(b: 3, a: 4), 11);
            assert_eq(f(b: 6), 8);
        "#)
}

#[test]
fn test_named_eval() {
    run(r#"
            let f = fn (a = 1, b = a + 2) {2 * a + b};

            assert_eq(f(), 5);
            assert_eq(f(b: 7), 9);
        "#)
}

#[test]
#[should_panic]
fn test_named_error1() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(c = 1)
        "#)
}

#[test]
#[should_panic]
fn test_named_error2() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(a = 1, a = 1)
        "#)
}

#[test]
#[should_panic]
fn test_named_error3() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(1, a = 1)
        "#)
}

#[test]
#[should_panic]
fn test_named_error4() {
    run(r#"
            let f = fn (a, b = 2) {2 * a + b};

            f(b = 1)
        "#)
}
