use crate::tests::run;

#[test]
#[should_panic]
fn test_unexpected_signal() {
    run(r#"
            continue;
        "#)
}
