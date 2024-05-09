use crate::tests::run;

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

#[test]
fn test_vector() {
    run(r#"
            struct Vec2 {
                pub x : Number;
                y;
                static m = 10;
                static other;
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
                    Vec2:{x * m, x * m}
                }
            }
            
            let v = Vec2 :{ 1, 2 };
            
            v.swap();
            assert_eq(v.x, 2);
            
            v.mul();
            assert_eq(v.y, 10);
            
            Vec2.m = 14;
            
            let v2 = Vec2.new45(5);
            
            assert_eq(v2.y, 70);
            
            assert_eq(Vec2.other, nah);

            print(v, v2);
            
            v.other = 5;
            assert_eq(Vec2.other, 5);
        "#)
}
