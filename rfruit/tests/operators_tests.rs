use crate::tests::run;

#[test]
fn test_num() {
    run(r#"
        t_eq(5 + 13, 18);
        t_eq(100 - 42, 58);
        t_eq(34 * 12, 408);
        t_eq(100 / 10, 10);
        t_eq(105 % 10, 5);
        t_eq(10 ** 2, 100);
        "#)
}

#[test]
fn test_num_comparison() {
    run(r#"
        t_eq(5 < 10, true);
        t_eq(5 <= 10, true);
        t_eq(5 > 10, false);
        t_eq(5 >= 10, false);
        t_eq(5 == 10, false);
        t_eq(5 != 10, true);
        "#)
}

#[test]
fn test_bool() {
    run(r#"
        t_eq(true && false, false);
        t_eq(true || false, true);
        "#)
}

#[test]
fn test_string() {
    run(r#"
        t_eq("hello" <> "world", "helloworld");
        t_eq("hi mom" * 4, "hi momhi momhi momhi mom");
        t_eq(3 * "kek, ", "kek, kek, kek, ");
        "#)
}

#[test]
fn test_string_comparison() {
    run(r#"
        t_eq("hello" < "world", true);
        t_eq("hello" <= "world", true);
        t_eq("hello" > "world", false);
        t_eq("hello" >= "world", false);
        t_eq("hello" == "world", false);
        t_eq("hello" != "world", true);
        "#)
}
