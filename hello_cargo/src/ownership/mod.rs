#[cfg(test)]
mod tests {
    #[test]
    fn test_function() {
        {                      // s is not valid here, it’s not yet declared
            let s = "hello";   // s is valid from this point forward
            println!("stack str:{}", s);
            // do stuff with s
        }                      // this scope is now over, and s is no longer valid
        //并不是基本字符串类型就可以满足所有情况，比如声明时不分配，而是用户输入分配
        //比如说字符串可变长度
        //同样一个字符可以再stack分配，也可以在heap分配
        {
            let mut s = String::from("hello");
            s.push_str(", world!");
            println!("heap str:{}", s);
            //rust automatically call drop method to release memory
        }
    }

    #[test]
    fn test_ways_variables_and_data_interact_move() {
        //这两个引用都是简单的固定大小的值，他们是在stack上面分配
        let x = 5;
        let y = x;
        println!("x and y is:{},{}", x, y);
        let s1 = String::from("hello");
        //这种赋值操作如果实行自动释放内存，则会导致内存安全问题
        // rust 会自动把s1置为失效，rust称之为move
        let s2 = s1;
        println!("{},world!", s2);
    }

    #[test]
    fn test_ways_variables_and_data_interact_clone() {
        //如果不希望引用拷贝而是深拷贝可以使用公共方法clone
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    #[test]
    fn test_stack_only_copy() {
        //这里会执行move吗？，x是不是没用了？，不是的
        //这个固定长度大小的类型，编译器可以预知，而且在stack上分配，分配速度很快
        //所以这里的浅拷贝和深拷贝是一个样，这个和java的基本类型分配一致
        let x = 5;
        let y = x;
        println!("x = {} , y = {}", x, y);
    }
}