use crate::tests::run;

#[test]
fn test_basics() {
    run(r#"
        let a = true;
        let b = false;
        assert_eq(a, true);
        assert_eq(b, false);
        
        print(true);
        "#);
}

#[test]
fn test_operators() {
    run(r#"
        assert_eq(true && false, false);
        assert_eq(true || false, true);
        "#)
}

#[test]
fn test_precedence() {
    run(r#"
            assert_eq(true || false && false, true);
        "#)
}
