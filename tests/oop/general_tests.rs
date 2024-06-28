use crate::run;

#[test]
fn test_scope() {
    run(r#"
            let Box = {
                let inner = 7;

                struct Box {
                } impl {
                    getAndInc() {
                        inner = inner + 1;
                        inner
                    }

                    static inc() {
                        inner = inner + 1;
                    }
                }

                Box
            };

            let b = Box :{ };

            assert_eq(b.getAndInc(), 8);
            Box.inc();
            assert_eq(b.getAndInc(), 10);
        "#)
}

#[test]
fn test_operators() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            operator + (a : Vec2, b : Vec2) {
                Vec2 :{ a.x + b.x, a.y + b.y }
            }

            commutative operator * (a : Vec2, k : Number) {
                Vec2 :{ a.x * k, a.y * k }
            }

            operator +-*/%=<>&|^!? (a : Number, k : Number) {
                a * k
            }

            let v1 = Vec2 :{ 1, 2 };
            let v2 = Vec2 :{ 3, 4 };

            assert_eq(v1 + v2, Vec2 :{ 4, 6 });

            assert_eq(v1 * 2, Vec2 :{ 2, 4 });
            assert_eq(5 * v1 * 2, Vec2 :{ 10, 20 });

            assert_eq(6 +-*/%=<>&|^!? 9, 54);

            print(Vec2);
        "#)
}

#[test]
#[should_panic(expected = "operator `+` is already set")]
fn test_operator_clash_1() {
    run(r#"
            operator + (a : Number, b : Number) {
                a + b
            }
        "#)
}

#[test]
#[should_panic(expected = "operator `+` is already set")]
fn test_operator_clash_2() {
    run(r#"
            struct A {}
            
            operator + (a : A, b : A) {
                a + b
            }
            operator + (a : A, b : A) {
                a + b
            }
        "#)
}

#[test]
fn test_named_fields() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            let v = Vec2 :{ x: 1, y: 2 };
            let v2 = Vec2 :{ y: 2, x: 1 };

            assert_eq(v, v2);
        "#)
}

#[test]
#[should_panic(expected = "missing field `x`")]
fn test_named_error_1() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{};
        "#)
}

#[test]
#[should_panic(expected = "field `x` is set more than once")]
fn test_named_error_2() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, x: 2};
        "#)
}

#[test]
#[should_panic(expected = "missing field `y`")]
fn test_named_error_3() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, c: 2};
        "#)
}

#[test]
#[should_panic(expected = "All arguments must be either named or not named at the same time at")]
fn test_named_error_4() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{1, y: 2};
        "#)
}

#[test]
#[should_panic(expected = "field `z` does not exist")]
fn test_named_error_5() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, y: 1, z: 2};
        "#)
}

#[test]
#[should_panic(expected = "missing field `y`")]
fn test_named_error_6() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{ 1 };
        "#)
}

#[test]
#[should_panic(expected = "variable `Box` already exists")]
fn test_redeclaration() {
    run(r#"
            let Box = 1;

            struct Box {}
        "#)
}
