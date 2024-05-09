use crate::tests::run;

#[test]
#[should_panic(expected = "variable `foo` does not exist")]
fn test_error_propagation_1() {
    run(r#"
            foo + 1;
        "#)
}

#[test]
#[should_panic(expected = "variable `bar` does not exist")]
fn test_error_propagation_2() {
    run(r#"
            0 + bar;
        "#)
}
