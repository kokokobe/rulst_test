#[cfg(test)]
mod tests {
    ///
    /// 函数指针默认实现了一下几个trait
    /// Fn, FnMut, and FnOnce
    ///
    #[test]
    fn function_pointer() {
        fn add_one(x: i32) -> i32 {
            x + 1
        }
        fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
            f(arg) + f(arg)
        }
        let answer = do_twice(add_one, 5);
        println!("The answer is :{}", answer);
        let list_of_numbers = vec![1, 2, 3];
        //两种方法，一种传递闭包，一种传递函数
        let list_of_strings: Vec<String> = list_of_numbers.iter().map(|item| item.to_string())
            .collect();
        let list_of_strings2: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
        #[derive(Debug)]
        enum Status {
            Value(u32),
            Stop,
        }
        let list_of_statuses : Vec<Status> = (0u32..20).map(Status::Value).collect();
        println!("list_of_strings is:{:?}", list_of_strings);
        println!("list_of_strings2 is:{:?}", list_of_strings2);
        println!("list_of_statuses is:{:?}", list_of_statuses);
    }
}