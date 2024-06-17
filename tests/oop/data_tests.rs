use crate::run;

#[test]
#[should_panic(expected = "cannot set field `f` in 'data' type `Number`")]
fn test_data() {
    run(r#"
            data Box {
                f;
            }

            let b = Box :{ 5 };
            assert_eq(b.f, 5);

            b.f = 10;
        "#)
}
