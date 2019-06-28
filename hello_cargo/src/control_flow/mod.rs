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
}