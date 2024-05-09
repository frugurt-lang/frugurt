use crate::tests::run;

#[test]
fn test_curry_1() {
    run(r#"
            let f = fn (a, b, c) {a + b + c};
            
            let g = f$(1);
            
            assert_eq(g(2, 3), 6);
            
            assert_eq(f(1, 2, 3), 6);
            
            assert_eq(g$(2)(5), 8);

            print(f);
        "#)
}

#[test]
fn test_curry_2() {
    run(r#"
            let g = print$(1);
            g(2);
            print(g);
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            (1 / 0)$();
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            nah$(1 / 0);
        "#)
}
