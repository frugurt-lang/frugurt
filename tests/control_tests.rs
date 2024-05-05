use crate::tests::run;

#[test]
#[should_panic]
fn test_unexpected_signal_1() {
    run(r#"
            return;
        "#)
}

#[test]
#[should_panic]
fn test_unexpected_signal_2() {
    run(r#"
            continue;
        "#)
}

#[test]
#[should_panic]
fn test_unexpected_signal_3() {
    run(r#"
            break;
        "#)
}
