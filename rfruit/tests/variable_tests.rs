use crate::tests::run;

#[test]
fn test_nah() {
    run(r#"
        let a = nah;
        t_eq(a, nah);
        "#);
}

#[test]
fn test_num() {
    run(r#"
        let a = 1;
        let c = a + 5.65;
        t_eq(a, 1);
        t_eq(c, 6.65);
        "#);
}

#[test]
fn test_str() {
    run(r#"
        let a = "hello";
        t_eq(a, "hello");
        "#);
}

#[test]
fn test_bool() {
    run(r#"
        let a = true;
        let b = false;
        t_eq(a, true);
        t_eq(b, false);
        "#);
}
