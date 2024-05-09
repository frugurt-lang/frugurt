use crate::tests::run;

#[test]
#[should_panic(expected = "variable `foo` does not exist")]
fn test_error_propagation_1() {
    run(r#"
            foo.bar;
        "#)
}
