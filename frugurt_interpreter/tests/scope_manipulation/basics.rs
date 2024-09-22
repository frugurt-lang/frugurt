use crate::run;

#[test]
fn test_basics() {
    run(r#"
            let f1 = fn() {
                let a = 5;
                let b = 3;
                scope()
            };

            let s = f1();

            scope s {
                let c = a + b;
                assert_eq(c * a, 40);
            }

            assert_eq(s.c, 8);

            assert_eq(scope s {
                c = c * c;
                c + 1
            }, 65);

            s.w = 1;
            s.w = s.w + 1;

            assert_eq(s.w, 2);
        "#)
}

#[test]
#[should_panic(expected = "Expected `Scope` in scope modifier statement, got `Number`")]
fn test_unexpected_type_1() {
    run(r#"
            scope 1 {}
        "#)
}

#[test]
#[should_panic(expected = "Expected `Scope` in scope modifier expression, got `Number`")]
fn test_unexpected_type_2() {
    run(r#"
            scope 1 { nah };
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            scope 1 / 0 {}
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            scope 1 / 0 { 1 };
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_3() {
    run(r#"
            scope scope() { 1 / 0; }
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_4() {
    run(r#"
            scope scope() { 1 / 0; nah };
        "#)
}
