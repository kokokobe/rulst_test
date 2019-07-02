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

    #[test]
    fn test_ownership_and_functions() {
        //函数传参参数定义一样，有move or copy的行为
        let s = String::from("hello"); //s comes into scope
        takes_ownership(s); // s's value moves into the function...
        // ... and so is no longer valid here
        let x = 5;    // x comes into scope

        makes_copy(x);     // x whould move into the function,
        println!("x is:{}", x);
        //  but i32 is Copy, so it's okay to still
        // use x afterward
    }// Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

    #[test]
    fn test_return_values_and_scope() {
        let s1 = gives_ownership();  // gives_ownership moves its return
        // value into s1
        let s2 = String::from("hello"); //s2 comes into scope
        let s3 = takes_and_gives_back(s2);  // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
        println!("s1 is:{},s3 is:{}", s1, s3);
    }// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
    // moved, so nothing happens. s1 goes out of scope and is dropped.

    #[test]
    fn test_return_values_and_scope2() {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}", s2, len);
    }

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("some_string is: {}", some_string);
    }// Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

    fn makes_copy(some_integer: i32) {
        println!("some_integer is:{}", some_integer);
    }// Here, some_integer goes out of scope. Nothing special happens.

    fn gives_ownership() -> String {  // gives_ownership will move its
        // return value into the function
        // that calls it
        let some_string = String::from("hello");
        some_string                 // some_string is returned and
        // moves out to the calling
        // function
    }

    // takes_and_gives_back will take a String and return one
    fn takes_and_gives_back(a_string: String) -> String {   // a_string comes into
        // scope
        a_string    // a_string is returned and moves out to the calling function
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String
        (s, length)
    }
}