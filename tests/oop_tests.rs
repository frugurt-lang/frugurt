use crate::tests::run;

#[test]
fn test_box() {
    run(r#"
            struct Box {
                pub fe : Number;
                static val = 5;
                static nope;
            } constraints {
                watch(fe) {
                    val = fe;
                    print(val);
                }
            }
            
            let b = Box :{ 5 };
            assert_eq(b.fe, 5);
            
            assert_eq(Box.val, 5);
            
            b.fe = 10;
            assert_eq(b.fe, 10);
            assert_eq(b.val, 10);

            assert_eq(Box.nope, nah);

            print(b, Box);
        "#)
}

#[test]
fn test_vector() {
    run(r#"
            struct Vec2 {
                pub x;
                y;
                static m = 10;
            } impl {
                swap() {
                    let tmp = x;
                    x = y;
                    y = tmp;
                }
                
                mul() {
                    x = x * m;
                    y = y * m;
                }
                
                static new45(x) {
                    Vec2:{x, x}
                }
            }
            
            let v = Vec2 :{ 1, 2 };
            
            v.swap();
            assert_eq(v.x, 2);
            
            v.mul();
            assert_eq(v.y, 10);
            
            let v2 = Vec2.new45(5);
            
            assert_eq(v2.y, 5);

            print(v, v2);
        "#)
}

#[test]
fn test_struct() {
    run(r#"
            struct Box {
                f;
            }
            
            let b = Box :{ 5 };
            assert_eq(b.f, 5);
            
            b.f = 10;
            assert_eq(b.f, 10);
            
            let b2 = b;
            
            assert_eq(b2.f, 10);
            
            b2.f = 20;
            assert_eq(b.f, 10);
            assert_eq(b2.f, 20);
        "#)
}

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
        "#)
}

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
#[should_panic]
fn test_named_error1() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{};
        "#)
}

#[test]
#[should_panic]
fn test_named_error2() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, x: 2};
        "#)
}

#[test]
#[should_panic]
fn test_named_error3() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, c: 2};
        "#)
}

#[test]
#[should_panic]
fn test_named_error4() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{1, y: 2};
        "#)
}

#[test]
#[should_panic]
fn test_named_error5() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{x: 1, y: 1, z: 2};
        "#)
}

#[test]
#[should_panic]
fn test_named_error6() {
    run(r#"
            struct Vec2 {
                x;
                y;
            }

            Vec2 :{ 1 };
        "#)
}

#[test]
#[should_panic(expected = "variable Box already exists")]
fn test_redeclaration() {
    run(r#"
            let Box = 1;

            struct Box {}
        "#)
}

#[test]
#[should_panic(expected = "division by zero")]
fn test_static_error() {
    run(r#"
            struct Box {
                static val = 5 / 0;
            }

            let b = Box :{ };
        "#)
}
