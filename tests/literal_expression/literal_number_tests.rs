use crate::tests::run;


#[test]
fn test_arithmetic() {
    run(r#"
            assert_eq({1 + 2} * 3 + {let x = 4; let y = 5; {x + 5} / y}, 10.8);
        "#)
}

#[test]
fn test_num_comparisons() {
    run(r#"
        assert_eq(5 < 10, true);
        assert_eq(5 <= 10, true);
        assert_eq(5 > 10, false);
        assert_eq(5 >= 10, false);
        assert_eq(5 == 10, false);
        assert_eq(5 != 10, true);
        "#)
}

#[test]
fn test_operators() {
    run(r#"
        assert_eq(5 + 13, 18);
        assert_eq(100 - 42, 58);
        assert_eq(34 * 12, 408);
        assert_eq(100 / 10, 10);
        assert_eq(105 % 10, 5);
        assert_eq(10 ** 2, 100);
        "#)
}

#[test]
fn test_precedence() {
    run(r#"
            assert_eq(1 + 2 * 3 + 4, 11);
            assert_eq(2 * 3 ** 3 * 5, 270);
            assert_eq(3 + 4 < 5 * 3, true);
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_divide_by_zero() {
    run(r#"
            1 / 0;
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_mod_by_zero() {
    run(r#"
            1 % 0;
        "#)
}

#[test]
#[should_panic(expected = "operator `Operator(Number +++ Number)` does not exist")]
fn test_unknown_operator() {
    run(r#"
            4 +++ 6;
        "#)
}
