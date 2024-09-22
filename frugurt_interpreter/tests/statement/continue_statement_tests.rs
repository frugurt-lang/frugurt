use crate::run;

#[test]
#[should_panic(expected = "Unexpected signal: Continue")]
fn test_unexpected_signal() {
    run(r#"
            continue;
        "#)
}
