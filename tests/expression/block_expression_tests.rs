use crate::run;

#[test]
fn test_basics() {
    run(r#"
            let x = 1;
            let y = {
                let x = x + 7;
                x * x
            };
            
            assert_eq(y, 64);
        "#)
}
