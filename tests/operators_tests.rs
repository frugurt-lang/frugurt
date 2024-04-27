use crate::tests::run;

#[test]
fn test_num() {
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
fn test_num_comparison() {
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
fn test_bool() {
    run(r#"
        assert_eq(true && false, false);
        assert_eq(true || false, true);
        "#)
}

#[test]
fn test_string() {
    run(r#"
        assert_eq("hello" <> "world", "helloworld");
        assert_eq("hi mom" * 4, "hi momhi momhi momhi mom");
        assert_eq(3 * "kek, ", "kek, kek, kek, ");
        "#)
}

#[test]
fn test_string_comparison() {
    run(r#"
        assert_eq("hello" < "world", true);
        assert_eq("hello" <= "world", true);
        assert_eq("hello" > "world", false);
        assert_eq("hello" >= "world", false);
        assert_eq("hello" == "world", false);
        assert_eq("hello" != "world", true);
        "#)
}
