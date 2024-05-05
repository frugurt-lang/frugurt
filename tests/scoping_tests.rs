use crate::tests::run;

#[test]
fn test_block() {
    run(r#"
            let a = 1;
            {
                let a = 5;
                a = 7;
                assert_eq(a, 7);
            }
            assert_eq(a, 1);
        "#)
}

#[test]
fn test_if() {
    run(r#"
            let a = 1;
            if true {
                let a = 5;
                assert_eq(a, 5);
            }
            assert_eq(a, 1);
        "#)
}

#[test]
fn test_if_else() {
    run(r#"
            let a = 1;
            if false {
                a = 7;
            } else {
                let a = 5;
                assert_eq(a, 5);
            }
            assert_eq(a, 1);
        "#)
}

#[test]
fn test_while() {
    run(r#"
            let a = 5;
            let s = 0;
            while {a = a - 1; a > 0} {
                let a = a - 4;
                s = s + a;
            }
            
            assert_eq(s, -6);
        "#)
}

#[test]
fn test_function1() {
    run(r#"
            let a = 5;
            let f = fn () { a = 7; };
            f();
            assert_eq(a, 7);
        "#)
}

#[test]
fn test_function2() {
    run(r#"
            let a = 5;
            let f = fn () { let a = 7; };
            f();
            assert_eq(a, 5);
        "#)
}

#[test]
#[should_panic]
fn test_function3() {
    run(r#"
            let f = fn () { a = 7; };
            f();
        "#)
}

#[test]
#[should_panic]
fn test_double_let() {
    run(r#"
            let a = 1;
            let a = 2;
        "#)
}

#[test]
#[should_panic]
fn test_not_exist() {
    run(r#"
            print(a);
        "#)
}
