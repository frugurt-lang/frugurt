use crate::run;

#[test]
fn test_basics() {
    run(r#"
            struct Vec {
                x;
                y;
            
                Length {
                    get { (x * x + y * y) ** 0.5 }
                    set(value) {
                        let l = Length / value;
                        x = x / l;
                        y = y / l;
                    }
                }
            }
            
            let v = Vec :{ x: 3, y: 4 };
            assert_eq(v.Length, 5);
            
            v.Length = 1;
            
            assert_eq(v.x + v.y, 1.4);
        "#)
}

#[test]
fn test_getter_arrow() {
    run(r#"
            struct Thing {
                x;
                
                Foo {
                    get => x + 1;
                }
            }
            
            let t = Thing :{ x: 3 };
            
            assert_eq(t.Foo, 4);
        "#)
}

#[test]
fn test_other() {
    run(r#"
            struct Thing {
                x;
                
                Foo {
                    set(val) {
                        if val == 3 {
                            return;
                        }
                        
                        x = val * 2;
                    }
                }
            }
            
            let t = Thing :{ x: 3 };

            t.Foo = 5;
            assert_eq(t.x, 10);
            
            t.Foo = 3;
            assert_eq(t.x, 10);
        "#)
}

#[test]
#[should_panic(expected = "property `X` has no getter")]
fn test_no_getter() {
    run(r#"
            struct Thing {
                X {}
            }
            
            let t = Thing :{};
            
            t.X;
        "#)
}

#[test]
#[should_panic(expected = "property `X` has no setter")]
fn test_no_setter() {
    run(r#"
            struct Thing {
                X {}
            }
            
            let t = Thing :{};
            
            t.X = 3;
        "#)
}

#[test]
#[should_panic(expected = "unexpected signal Continue")]
fn test_unexpected_signal() {
    run(r#"
            struct Thing {
                X {
                    set {
                        continue;
                    }
                }
            }
            
            let t = Thing :{};
            
            t.X = 3;
        "#)
}

#[test]
fn test_static_basics() {
    run(r#"
            let T = {
                let inner = 5;
                
                struct Thing {
                    static Foo {
                        get => inner + 5;
                        set(val) {
                            inner = val - 5;  
                        }
                    }
                }
                
                Thing
            };
            
            assert_eq(T.Foo, 10);
            
            T.Foo = 3;
            
            assert_eq(T.Foo, 3);
        "#)
}
