#[cfg(test)]
mod tests {
    use std::ops::Deref;

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
            _Closure,
        }
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
        println!("list_of_strings is:{:?}", list_of_strings);
        println!("list_of_strings2 is:{:?}", list_of_strings2);
        println!("list_of_statuses is:{:?}", list_of_statuses);
    }

    #[test]
    fn returning_closures() {
        //rust 不允许使用函数指针返回值
//        fn return_closure() -> Fn(i32) -> i32 {
//            |x| x + 1
//        }
        fn return_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
        let closure = return_closure();
        let x1 = closure(33);
        let x2 = closure.deref()(32);
        let x3 = (*closure)(31);
        println!("call return closure value:{}", x1);
        println!("call return closure value:{}", x2);
        println!("call return closure value:{}", x3);
    }
}