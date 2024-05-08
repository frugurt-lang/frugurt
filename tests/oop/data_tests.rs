use crate::tests::run;


#[test]
#[should_panic]
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
