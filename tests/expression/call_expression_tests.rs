use crate::tests::run;


#[test]
#[should_panic(expected = "DoesNotExist")]
fn test_named_error_1() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(c: 1);
        "#)
}

#[test]
#[should_panic(expected = "SameSetTwice")]
fn test_named_error_2() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(a: 1, a: 1);
        "#)
}

#[test]
#[should_panic(expected = "SameSetTwice")]
fn test_named_error_3() {
    run(r#"
            let f = fn (a = 1, b = 2) {2 * a + b};

            f(1, a: 1);
        "#)
}

#[test]
#[should_panic(expected = "NotSet")]
fn test_named_error_4() {
    run(r#"
            let f = fn (a, b = 2) {2 * a + b};

            f(b: 1);
        "#)
}

#[test]
#[should_panic(expected = "Positional parameters should be before default parameters")]
fn test_named_error_5() {
    run(r#"
            fn (a = 1, b) {2 * a + b};
        "#)
}

#[test]
#[should_panic(expected = "Positional arguments should be before named arguments")]
fn test_named_error_6() {
    run(r#"
            let f = fn (a, b) {2 * a + b};

            f(b: 1, 1);
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_named_error_7() {
    run(r#"
            let f = fn (a, b = 5 / a) {2 * a + b};

            f(0);
        "#)
}

#[test]
#[should_panic(expected = "TooMany")]
fn test_count_error_1() {
    run(r#"
            let f = fn (a, b) {};

            f(1, 2, 3);
        "#)
}

#[test]
#[should_panic(expected = "NotSet")]
fn test_count_error_2() {
    run(r#"
            let f = fn (a, b) {};

            f(1);
        "#)
}
