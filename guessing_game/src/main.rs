use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    loop{
        println!("Please input your guess.");
        //可变变量，默认不可变
        let mut guess = String::new();
        //这是个引用，引用默认也是不可变的，这里声明为可变的
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().
            expect("Please type a number");
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        };

    }

}