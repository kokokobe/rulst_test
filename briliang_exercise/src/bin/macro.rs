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
        fn from(usize) -> Self;
    }
}