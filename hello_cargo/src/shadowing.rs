//使用重复的变量名可以遮蔽之前的变量值，可以隐藏之前的值
pub fn shadowing() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}