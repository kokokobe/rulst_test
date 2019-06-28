#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {
        another_function(5, 6);
    }

    fn another_function(x: i32, y: i32) {
        println!("The value of x is : {},y is:{}", x, y);
    }

    //整个函数也算是一个声明，声明没有返回值
    #[test]
    fn statement_in_func() {
        //这是一个声明
        let y = 6;
        println!("statement y is:{}", y);
        let x = 5;
        println!("outer scope x is: {}", x);
        let y = {
            //独立的scope
            let x = 3;
            // Expressions do not include ending semicolons.
            // If you add a semicolon to the end of an expression, you turn it into a statement,
            // which will then not return a value. Keep this in mind as you explore function return values and expressions next.
            //如果有分号表明是一个声明没有返回值，则Y没有值
            //如果没有分号则是一个expression表达式，自动采用它的值作为返回值
            x + 1
        };
        println!("The value of y is : {:?}", y);
    }

    #[test]
    fn function_with_return_values() {
        fn five() -> i32 {
            //数字是一个表达式，并且没有分号,有分号是statement哦
            //自动作为返回值
            5
        }
        let x = five();
        println!("The value of x is : {}", x);
    }

    #[test]
    fn function_with_return_values2() {
        fn plus_one(x: i32) -> i32 {
            x + 1
        }
        let x = plus_one(5);
        println!("The value of x is : {}", x);
    }
}