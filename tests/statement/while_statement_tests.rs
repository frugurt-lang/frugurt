use crate::tests::run;

#[test]
#[should_panic(expected = "Expected `Bool` in while condition, got `String`")]
fn test_unexpected() {
    run(r#"
            while "true" {}
        "#)
}

#[test]
fn test_break() {
    run(r#"
            let a = 0;
            while true {
                a = a + 1;
                if a == 5 {
                    break;
                }
            }

            assert_eq(a, 5);
        "#)
}

#[test]
fn test_continue() {
    run(r#"
            let a = 0;
            let b = 0;
            while a < 10 {
                a = a + 1;
                if a == 5 {
                    continue;
                }
                b = b + 1;
            }

            assert_eq(b, 9);
        "#)
}

#[test]
fn test_return() {
    run(r#"
            let res = fn () {
                let a = 0;
                while true {
                    a = a + 1;
                    if a == 5 {
                        return a;
                    }
                }
            }();

            assert_eq(res, 5);
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            while 1 / 0 {}
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            let a = 0;
            while true {
                a = a + 1;
                if a == 5 {
                    1 / 0;
                }
            }
        "#)
}
