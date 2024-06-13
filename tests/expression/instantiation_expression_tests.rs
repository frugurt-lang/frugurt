use crate::run;

#[test]
#[should_panic(expected = "variable `Lol` does not exist")]
fn test_error_propagation_1() {
    run(r#"
            Lol :{};
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_error_propagation_2() {
    run(r#"
            struct Box {
                f;
            }

            let b = Box :{ 1 / 0 };
        "#)
}

#[test]
#[should_panic(expected = "")]
fn test_error_propagation_3() {
    run(r#"
            struct Box {
                f;
            }

            Box :{ 1, 2 };
        "#)
}
