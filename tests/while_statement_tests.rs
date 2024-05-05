use crate::tests::run;

#[test]
#[should_panic(expected = "division by zero")]
fn test_() {
    run(r#"
            while 1 / 0 {}
        "#)
}
