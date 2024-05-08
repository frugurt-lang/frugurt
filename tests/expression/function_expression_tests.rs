use crate::tests::run;


#[test]
fn test_function() {
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
fn test_named_eval() {
    run(r#"
            let f = fn (a = 1, b = a + 2) {2 * a + b};

            assert_eq(f(), 5);
            assert_eq(f(b: 7), 9);
        "#)
}

#[test]
fn test_named_eval_2() {
    run(r#"
            let f = fn (a, b = 5 / a) {2 * a + b};

           assert_eq(f(1), 7);
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
#[should_panic(expected = "unexpected signal Continue")]
fn test_unexpected_signal_1() {
    run(r#"
            fn () {continue;}();
        "#)
}

#[test]
#[should_panic(expected = "unexpected signal Break")]
fn test_unexpected_signal_2() {
    run(r#"
            fn () {break;}();
        "#)
}

#[test]
#[should_panic(expected = "unexpected signal Continue")]
fn test_unexpected_signal_default_1() {
    run(r#"
            fn (a={continue; 1}) {}();
        "#)
}

#[test]
#[should_panic(expected = "unexpected signal Break")]
fn test_unexpected_signal_default_2() {
    run(r#"
            fn (a={break; 1}) {}();
        "#)
}

#[test]
#[should_panic(expected = "unexpected signal Return")]
fn test_unexpected_signal_default_3() {
    run(r#"
            fn (a={return 1; 1}) {}();
        "#)
}

