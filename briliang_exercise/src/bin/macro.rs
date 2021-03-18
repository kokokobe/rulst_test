use std::ops::Add;
macro_rules! calculate {
    (eval $e:expr) => {
            // $e 表达式被替换到右边
        let val: usize = $e;
        println!("{} = {}",stringify!{$e},val);
    }
}
fn main() {
    calculate! { eval 1+2 }
    calculate! { eval (1+2)*(3/1)}
    let i = (0..10).collect::<Vec<i32>>();
    println!("{:?}", i);
    Vec::<u8>::with_capacity(1024);
    // const fn hello() -> String {
    //     let string = String::new();
    //     string.add("hello");
    //     string
    // }
    // const S:String = hello();
    // println!("{}", S)
    let atr: [i32; 3] = [1, 2, 3];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?},{:?}", atr, arr);

    pub trait Index : PartialEq + Copy {
        fn to_usize(self) -> usize;
        fn from(_i: usize) -> Self;
    }
    let num  = 42u32;
    println!("u32 num :{}", num);
    let num = 0x2A;
    println!("u32 num :{}", num);
    let num = 0o106;
    println!("u32 num :{}", num);
    let num = 0b1101_1011;
    println!("u32 num :{}", num);
    assert_eq!(b'*', 42u8);
    assert_eq!(-3.14, -3.14f64);
    assert_eq!('\x2A', '*');
    println!("unicode 字符：{}", '\u{CA0}');
    let int_array = [0; 10];
    println!("int array {:?}", int_array);
}