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
        //同样一个字符可以在stack分配，也可以在heap分配
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

    #[test]
    fn test_references_and_borrowing() {
        let mut s1 = String::from("hello");
        //&引用传递，不需要获取所有权
        //&s1 创建了另一个引用指向s1,并且不拥有它
        //不能租借2次可变的引用
        //let r1 = &mut s1;
        //let r2 = &mut s1;
        //println!("{}, {}", r1, r2);
        let len = calculate_length2(&mut s1);


        println!("The length of '{}' is {}.", s1, len);
    }

    #[test]
    fn test_references_and_borrowing2() {
        //使用大括号构成一个新的范围
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("r1 is :{}", r1);
        };// r1 goes out of scope here, so we can make a new reference with no problems.
        //租借完之后才能使用s 指针
        let r2 = &mut s;
        println!("r2 is :{}", r2);
        println!("s is :{}", s);
    }

    #[test]
    fn test_references_and_borrowing3() {
        // Note that a reference's scope starts from where it is introduced and
        // continues through the last time that reference is used. For instance, this code will compile
        // because the last usage of the immutable references occurs before the mutable reference is introduced:
        //租借的不可变得引用期待的是读的操作，在读完之后再租借为可变引用时可以进行写操作
        let mut s = String::from("hello");
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("r1 is {} and r2 is {},s is {}", r1, r2, s);
        // r1 and r2 are no longer used after this point
        let r3 = &mut s;
        println!("r3 is {}", r3);
    }

    #[test]
    fn test_dangling_references() {
        let reference_to_nothing = dangle();
        fn dangle() -> String { // dangle returns a reference to a String
            let s = String::from("hello"); // s is a new String
            s // we return a reference to the String, s
        }// Here, s goes out of scope, and is dropped. Its memory goes away.
        // Danger!
        println!("dangle is {}", reference_to_nothing);
    }

    #[test]
    fn test_slice_type() {
        let first = String::from("sexy girl");
        let word = first_word(&first);
        //first.clear();
        println!("first word is:{}", word);
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        let hello2 = &s[..5];
        println!("hello is {},world is {},hello2 is {}", hello, world, hello2);
        let s2 = String::from("hello");
        let len = s2.len();
        let slice = &s2[3..len];
        println!("slice is {}", slice);
        let slice = &s2[3..];
        println!("slice is {}", slice);
        let slice = &s2[..];
        println!("slice is {}", slice);
    }

    #[test]
    fn test_slice_str_type() {
        let my_string = String::from("hello world");
        //first_word works on slices of `String` s
        let s = &my_string[..];
        let word = first_word(s);
        println!("word is :{}", word);
        //栈上分配字符引用，已经是slice
        let my_string_literal = "hello world";
        //first_word works on slices of string literals
        let word = first_word(&my_string_literal[..]);
        println!("word is :{}", word);
        //because string literals *are* string slices already,
        //this works too,without the slice syntax!
        let word = first_word(my_string_literal);
        println!("word is :{}", word);
    }

    #[test]
    fn test_slice_tuple(){
        let a = [1, 2, 3, 4, 5, ];
        //不包含3的边界值
        let slice = &a[1..3];
        println!("slice tuple is {:?}", slice);
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

    //这种函数拥有引用的称为租借borrowing
    fn calculate_length2(s: &mut String) -> usize {// s is a reference to a String
        s.push_str(", world!");
        s.len()
    }// Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.


    fn first_word(s: &str) -> &str {//字符串切片引用
        let bytes = s.as_bytes();
        //enumerate 的作用是包装iter函数的返回值为元祖数据
        //因为as_bytes()函数返回的是引用
        //因为enumerate 返回的是引用，所以&item也是
        let enumerate = bytes.iter().enumerate();
        for (i, &item) in enumerate {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }
}