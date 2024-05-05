use crate::tests::run;

#[test]
#[should_panic(expected = "Expected bool in if condition, got Number")]
fn test_type_mismatch() {
    run(r#"
            if 1 {}
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            if 1 / 0 {
            }
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            if false {
            } else {
                1 / 0;
            }
        "#)
}
