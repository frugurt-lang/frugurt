use crate::run;

#[test]
fn test_if_else() {
    run(r#"
            let a = if true { 1 } else { 2 };
            assert_eq(a, 1);

            let b = if false { 1 } else if true { 2 } else { 3 };
            assert_eq(b, 2);

            let c = if false { 1 } else if false { 2 } else { 3 };
            assert_eq(c, 3);
        "#)
}

#[test]
#[should_panic(expected = "Expected `Bool` in if condition, got `String`")]
fn test_type_mismatch() {
    run(r#"
            if "nope" {
                1
            } else {
                2
            };
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            if 1 / 0 {
                1
            } else {
                2
            };
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            if true {
                1 / 0
            } else {
                2
            };
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_3() {
    run(r#"
            if false {
                1
            } else {
                2 / 0
            };
        "#)
}
