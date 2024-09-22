use crate::run;

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation() {
    run(r#"
            return 1 / 0;
        "#)
}

#[test]
#[should_panic(expected = "Unexpected signal: Return")]
fn test_unexpected_signal() {
    run(r#"
            return;
        "#)
}
