use crate::tests::run;

#[test]
fn test_box() {
    run(r#"
            struct Box {
                pub fe : Number;
                static val = 5;
                
                -----constraints-----
                
                watch(fe) {
                    val = fe;
                    print(val);
                }
            }
            
            let b = Box :{ 5 };
            t_eq(b.fe, 5);
            
            t_eq(Box.val, 5);
            
            b.fe = 10;
            t_eq(b.fe, 10);
            t_eq(b.val, 10);
        "#)
}

#[test]
fn test_vector() {
    run(r#"
            struct Vec2 {
                x;
                y;
                static m = 10;
                
                -----impl-----
                
                swap() {
                    let tmp = x;
                    x = y;
                    y = tmp;
                }
                
                mul() {
                    x = x * m;
                    y = y * m;
                }
                
                -----static-----
                
                new45(x) {Vec2:{x, x}}
            }
            
            let v = Vec2 :{ 1, 2 };
            
            v.swap();
            t_eq(v.x, 2);
            
            v.mul();
            t_eq(v.y, 10);
            
            let v2 = Vec2.new45(5);
            
            t_eq(v2.y, 5);
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
                        
            t_eq(v1 + v2, Vec2 :{ 4, 6 });
            
            t_eq(v1 * 2, Vec2 :{ 2, 4 });
            t_eq(5 * v1 * 2, Vec2 :{ 10, 20 });
            
            t_eq(6 +-*/%=<>&|^!? 9, 54);
        "#)
}
