use std::io;
fn main(){
    println!("Guess the number !");
    println!("Please input your guess.");
    //可变变量，默认不可变
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You guessed: {}", guess);

}