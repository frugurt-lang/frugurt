use crate::tests::run;

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_1() {
    run(r#"
            let x = 1 / 0;
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            let x = 1;
            x = 1 / 0;
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_3() {
    run(r#"
            let x = 1;
            x.x = 1 / 0; // this assignment give error, but divison happens earlier
        "#)
}

#[test]
#[should_panic(expected = "does not exist")]
fn test_faulty_assignment() {
    run(r#"
            x.x = 1 / 0;
        "#)
}
