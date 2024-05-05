use crate::tests::run;


#[test]
#[should_panic]
fn test_type_mismatch() {
    run(r#"
            if "nope" {1} else {2};
        "#)
}
