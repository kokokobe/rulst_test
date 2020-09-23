//新特性不需要使用了
//extern crate briliang_exercise;

#[test]
fn test_briliang_exercise() {
    briliang_exercise::data_type::test();
    struct Foo {
        x: i32,
    }

    fn print_value(x: i32) {
        println!("{}", x)
    }

    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // -> 42; expected result: 13
    print_value(foo.x);
}