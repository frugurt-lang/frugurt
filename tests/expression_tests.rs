use crate::tests::run;

#[test]
fn test_arithmetic() {
    run(r#"
            t_eq({1 + 2} * 3 + {let x = 4; let y = 5; {x + 5} / y}, 10.8);
        "#)
}

#[test]
fn test_if_else() {
    run(r#"
            let a = if true { 1 } else { 2 };
            t_eq(a, 1);
            
            let b = if false { 1 } else if true { 2 } else { 3 };
            t_eq(b, 2);
            
            let c = if false { 1 } else if false { 2 } else { 3 };
            t_eq(c, 3);
        "#)
}

#[test]
fn test_precedence() {
    run(r#"
            t_eq(1 + 2 * 3 + 4, 11);
            t_eq(2 * 3 ** 3 * 5, 270);
            t_eq(true || false && false, true);  
            t_eq(3 + 4 < 5 * 3, true);    
        "#)
}
