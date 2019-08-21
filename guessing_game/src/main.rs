use std::io;
use std::cmp::Ordering;
use rand::Rng;
use briliang_exercise::data_type::number::number_operation;

#[derive(Debug)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess {
            value
        }
    }
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    number_operation();
    println!("Guess the number !");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    //println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");
        //可变变量，默认不可变
        let mut guess = String::new();
        //这是个引用，引用默认也是不可变的，这里声明为可变的
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            //下划线表示捕获所有异常
            Err(_) => {
                println!("Please input number!");
                continue;
            }
        };
        let guess = Guess::new(guess);
        println!("You guessed: {:?}", guess);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    };
}

