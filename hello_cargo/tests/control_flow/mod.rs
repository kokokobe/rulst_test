#[cfg(test)]
mod tests {
    #[test]
    fn test_control_flow_if() {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false")
        }
    }

    #[test]
    fn test_if_with_statement() {
        //if 是一个表达式所以可以用于 let 右边
        let condition = true;
        let number = if condition {
            5
        } else {
            6
        };
        println!("The value of number is : {} ", number)
    }

    //永久循环模式
    #[test]
    fn test_loops() {
        loop {
            println!("again! loop!");
            break;
        }
    }

    //永久循环模式
    #[test]
    fn test_loop_return_value() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);
    }

    //while 其实是可以使用loop if else break 组合实现的
    #[test]
    fn test_while() {
        let mut number = 3;
        while number != 0 {
            println!("{}!", number);
            number -= 1;
        }
        println!("LIFTOFF!!!")
    }

    #[test]
    fn test_while2() {
        let a = [10, 20, 30, 40, 50];
        let mut index: usize = 0;
        while index < 5 {
            println!("the value is: {}", a[index]);
            index += 1;
        }
    }

    //可以对比 test_while_2()方法
    #[test]
    fn test_for() {
        let a = [10, 20, 30, 40, 50];
        for element in a.iter() {
            println!("the value is: {}", element);
        }
    }

    #[test]
    fn test_for2() {
        for number in (1..4).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!")
    }

    #[test]
    fn test_match() {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                //箭头之后是表达式
                Coin::Penny => {
                    println!("match penny!");
                    1
                }
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        let cents = value_in_cents(Coin::Quarter);
        println!("Coin quarter cents is:{}", cents);
        let cents = value_in_cents(Coin::Dime);
        println!("Coin Dime cents is:{}", cents);
        let cents = value_in_cents(Coin::Nickel);
        println!("Coin Nickel cents is:{}", cents);
        let cents = value_in_cents(Coin::Penny);
        println!("Coin Penny cents is:{}", cents);
    }

    #[test]
    fn test_match_2() {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
        }
        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:#?}!", state);
                    25
                }
            }
        }
        let in_cents = value_in_cents(Coin::Quarter(UsState::Alaska));
        println!("Coin quarter alaska cent is:{}", in_cents);
        let in_cents = value_in_cents(Coin::Quarter(UsState::Alabama));
        println!("Coin quarter Alabama cent is:{}", in_cents);
        let in_cents = value_in_cents(Coin::Penny);
        println!("Coin Penny cent is:{}", in_cents);
        let in_cents = value_in_cents(Coin::Nickel);
        println!("Coin Nickel cent is:{}", in_cents);
        let in_cents = value_in_cents(Coin::Dime);
        println!("Coin Dime cent is:{}", in_cents);
    }

    #[test]
    fn test_match_3() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
        println!("add some is:{:?},add none is:{:?}", six, none);
    }

    #[test]
    fn test_match_4() {
        //定义u8 0 的值
        let some_u8_value = 12u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            //匹配剩余值
            _ => println!("other is:{}", some_u8_value),
        };
    }

    #[test]
    fn test_if_let_syntax() {
        //只希望匹配 match 中的一个条件逻辑处理
        let some_u8_value = Some(4u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => println!("not three"),
        }
        if let Some(3) = some_u8_value {
            println!("three")
        } else {
            println!("not three")
        }
        if Some(3) == some_u8_value {
            println!("three")
        } else {
            println!("not three")
        }
    }
}