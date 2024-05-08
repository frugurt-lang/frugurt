use crate::tests::run;


#[test]
fn test_class() {
    run(r#"
            class Box {
                f;
            }
            
            let b = Box :{ f: 5 };
            assert_eq(b.f, 5);
            
            b.f = 10;
            assert_eq(b.f, 10);
            
            let b2 = b;
            
            assert_eq(b2.f, 10);
            
            b2.f = 20;
            assert_eq(b.f, 20);
            assert_eq(b2.f, 20);
        "#)
}
