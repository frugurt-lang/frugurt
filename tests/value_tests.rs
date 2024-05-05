use crate::tests::run;

#[test]
fn test_nah() {
    run(r#"
        let a = nah;
        assert_eq(a, nah);
        "#);
}

#[test]
fn test_number() {
    run(r#"
        let a = 1;
        let c = a + 5.65;
        assert_eq(a, 1);
        assert_eq(c, 6.65);
        "#);
}

#[test]
fn test_string() {
    run(r#"
        let a = "hello";
        assert_eq(a, "hello");
        "#);
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

#[test]
fn test_bool() {
    run(r#"
        let a = true;
        let b = false;
        assert_eq(a, true);
        assert_eq(b, false);
        "#);
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
