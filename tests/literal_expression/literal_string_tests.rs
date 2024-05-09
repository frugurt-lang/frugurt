use crate::tests::run;

// TODO: add more escaping tests

#[test]
fn test_basics() {
    run(r#"
        let a = "hello";
        assert_eq(a, "hello");
        "#);
}

#[test]
fn test_operators() {
    run(r#"
        assert_eq("hello" <> "world", "helloworld");
        assert_eq("hi mom" * 4, "hi momhi momhi momhi mom");
        assert_eq(3 * "kek, ", "kek, kek, kek, ");
        "#)
}

#[test]
fn test_string_comparisons() {
    run(r#"
        assert_eq("hello" < "world", true);
        assert_eq("hello" <= "world", true);
        assert_eq("hello" > "world", false);
        assert_eq("hello" >= "world", false);
        assert_eq("hello" == "world", false);
        assert_eq("hello" != "world", true);
        "#)
}

#[test]
#[should_panic(expected = "integer")]
fn test_string_times_negative() {
    run(r#"
            "asd" * -4;
        "#)
}

#[test]
#[should_panic(expected = "integer")]
fn test_string_times_float() {
    run(r#"
            "asd" * 4.5;
        "#)
}

#[test]
fn test_string_escaping() {
    run(r#"
        "hello\nworld";
        "\t\n\v\f\r";
        "hi \
        mom";
        let s = "\u{041F}\u{0440}\u{0438}\u{0432}\u{0435}\u{0442}\u{002C}\u{0020}\u{043C}\u{0430}\u{043C}\u{0430}\u{0021}";
        assert_eq(s, "Привет, мама!");
        "#);
}
