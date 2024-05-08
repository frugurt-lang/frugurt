use crate::tests::run;


#[test]
#[should_panic(expected = "Unexpected signal: Break")]
fn test_unexpected_signal() {
    run(r#"
            break;
        "#)
}
