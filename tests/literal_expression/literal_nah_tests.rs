use crate::run;

#[test]
fn test_basics() {
    run(r#"
            let x = nah;

            assert_eq(x, nah);
        "#)
}
