#[cfg(test)]
mod tests {

    #[test]
    fn test_collections_vector() {
        let v: Vec<i32> = Vec::with_capacity(5);
        println!("Vector v is:{:#?}", v);
        let v = vec![1, 2, 3];
        println!("Vector with initial values:{:#?}", v);
        let mut v = Vec::with_capacity(4);
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        println!("Vector with updated values:{:#?}", v);
        // vector gets dropped ,all element will dropped
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element is: {}", third);
        let option = v.get(2);
        match option {
            Some(third) => println!("The third element is: {}", third),
            None => println!("There is no third element.")
        };
        let mut v = vec![1, 2, 3, 4, 5];
        let first = &v[0];
        println!("The first element is :{}", first);
        v.push(6);
        //为啥官方说不能这么用，看起来好像没问题，因为这个vector涉及到内存扩容分配，会导致
        //第一个元素的数据被改变，租借的引用已经指向的路径已经失效
        //println!("The first element is: {:#?}", first);
        println!("The vector element is: {:#?}", v);
    }

    #[test]
    fn test_collections_vector_iterate() {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("vector iterate ele is:{}", i);
        }
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            //* 的操作叫做dereference
            *i += 50;
            println!("after increase i is:{}", i);
        }
    }

    #[test]
    fn test_collections_vector_enums() {
        #[derive(Debug)]
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }
        let row = vec![SpreadSheetCell::Int(3), SpreadSheetCell::Float(10.12),
                       SpreadSheetCell::Text(String::from("blue"))];
        println!("enums vector is :{:#?}", row);
    }

    #[test]
    fn test_collections_string() {
        //String 是Unicode格式的
        let string = String::new();
        println!("string one :{}", string);
        let data = "initial contents";
        let s = data.to_string();
        println!("string second:{}", s);
        let mut s = String::from("foo");
        s.push_str("bar");

        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);
        s1.push('l');
        println!("s is {}", s1);
    }

    #[test]
    fn test_collections_string2() {
        // + operator operate string
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        //+操作符其实使用的字符串的方法add 第二个参数是&str，而不是&String，为啥可以使用呢？
        //因为编译器强制替换了&String --> &str
        //&s2 --> &s2[..]，把这个变成slice
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("s3 is :{} ", s3);
        println!("s2 is :{} ", s2);
        //这么写的方式不太友好
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3;
        println!("s is:{} ", s);
        //这样写高大上？
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        //这样写高大上？
        //format! 宏类似println! 但是它返回一个值而不是打印到控制台
        //format! 不会用占用任何的参数
        let s = format!("{}-{}-{}", s1, s2, s3);
    }

    #[test]
    fn test_collection_str_indexing() {
        let s1 = String::from("hello");
        //其他语言有类似的的查询某个字符串的某个位置的字符，但是rust这样访问是有问题的
        //let x = s1[0];
        let i = s1.len();
        //每个字符一个字节Unicode-8存储
        println!("s1 str length is:{}", i);
        //这种字符每个字符用的2 byte来存储
        let cyrillic = String::from("Здравствуйте");
        println!("s1 str length is:{}", cyrillic.len());
        // not work
        //let answer = &cyrillic[0];
        //println!("which is the cyrillic answer is:{}", answer);
        //println!("the hello first index:{}", &"hello"[0]);
        println!("the hello first index:{:#?}", &"hello".chars().nth(0));
        //as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).
        // 字符串可以看做是字节、标量值，字母集合
        let str = "नमस्ते";
        println!("नमस्ते store value is:{:?}", str.as_bytes());
    }

    #[test]
    fn test_slice_str(){
        //这个字符每个符号用2个字节存储,0..4表示前四个字节
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        println!("slice str is:{}", s);
    }
}