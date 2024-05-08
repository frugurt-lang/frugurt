use crate::tests::run;


#[test]
fn test_basic() {
    run(r#"
            assert_eq("assert_eq", "assert_eq");
        "#)
}


#[test]
#[should_panic]
fn test_not_eq_1() {
    run(r#"
            assert_eq(1, 2);
        "#)
}

#[test]
#[should_panic]
fn test_not_eq_2() {
    run(r#"
            assert_eq(1, "1");
        "#)
}
