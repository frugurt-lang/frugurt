use crate::tests::run;

#[test]
fn test_arithmetic() {
    run(r#"
            assert_eq({1 + 2} * 3 + {let x = 4; let y = 5; {x + 5} / y}, 10.8);
        "#)
}

#[test]
fn test_if_else() {
    run(r#"
            let a = if true { 1 } else { 2 };
            assert_eq(a, 1);
            
            let b = if false { 1 } else if true { 2 } else { 3 };
            assert_eq(b, 2);
            
            let c = if false { 1 } else if false { 2 } else { 3 };
            assert_eq(c, 3);
        "#)
}

#[test]
fn test_precedence() {
    run(r#"
            assert_eq(1 + 2 * 3 + 4, 11);
            assert_eq(2 * 3 ** 3 * 5, 270);
            assert_eq(true || false && false, true);  
            assert_eq(3 + 4 < 5 * 3, true);    
        "#)
}
