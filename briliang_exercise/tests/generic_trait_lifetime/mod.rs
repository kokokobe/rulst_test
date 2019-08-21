#[cfg(test)]
mod tests {
    //trait 可以定义某些公用特性,并且可以定义泛型的边界
    use std::fmt::{Display, Debug};
    use std::fmt;

    pub trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    #[derive(Debug)]
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    impl fmt::Display for Tweet {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "SomeError")
        }
    }

    #[test]
    fn test_removing_duplication_by_extracting_function() {
        let number_list: Vec<i64> = vec![489494979455, 50, 25, 100, 489494979454];
        fn largest_num<T>(num_list: &[T]) -> &T
            where T: PartialOrd
        {
            let mut largest = &num_list[0];
            for number in num_list {
                if number > largest {
                    largest = number;
                }
            };
            return largest;
        }
        let num = largest_num(&number_list);
        println!("The largest number is:{}", num);
    }

    #[test]
    fn test_generic_data_type_in_function() {
        //定义一个泛型函数
//        fn largest_num<T>() -> T {
//
//        }
    }

    #[test]
    fn test_struct_definition_generic_type() {
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }
        let integer = Point { x: 1, y: 2 };
        let float = Point { x: 1.0, y: 2.0 };
        println!("integer is:{:#?},float is:{:#?}", integer, float);
        #[derive(Debug)]
        struct Point2<T, U> {
            x: T,
            y: U,
        }
        let integer = Point2 { x: 1, y: 2.0 };
        let float = Point2 { x: 1.0, y: 4.0 };
        println!("integer is:{:#?},float is:{:#?}", integer, float);
    }

    #[test]
    fn test_enum_definition_generic_type() {
        #[derive(Debug)]
        enum A<T, U> {
            A(T),
            B(U),
        }
        let a: A<i32, f32> = A::A(1);
        let b: A<i32, f32> = A::B(1.9);
        println!("a is:{:?},b is:{:?}", a, b);
    }

    #[test]
    fn test_method_definition_generic_type() {
        struct Point<T> {
            x: T,
            y: T,
        }
        //impl<T> 的这个泛型定义是为了让编译器知道，后面定义的Point<T> 的类型是泛型而不是具体的类型
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        //我们也可以这么写，所以需要额外定义
//        impl Point<i32>{
//            fn x(&self)->&i32{
//                &self.x
//            }
//        }
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        let p = Point { x: 5, y: 10 };
        println!("p.x={}", p.x());
        let p2 = Point { x: 3.4, y: 4.4 };
        let distance = p2.distance_from_origin();
        println!("p2 distance is:{}", distance);
    }

    #[test]
    fn test_struct_multiple_generic_type() {
        #[derive(Debug)]
        struct Point<T, U> {
            x: T,
            y: U,
        }
        impl<T, U> Point<T, U> {
            //定义方法的泛型
            fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                //构建一个对象所有权会转移，所以只能move x,y
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "hello", y: 'c' };
        let point = p1.mix_up(p2);
        println!("p3 is:{:?}", point);
    }

    //定义共享的行为举动
    //Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
    #[test]
    fn test_trait() {
        let tweet = Tweet {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        };
        let news_article = NewsArticle {
            headline: "大事件".to_string(),
            location: "Guangzhou".to_string(),
            author: "BriLiang".to_string(),
            content: "earthquake".to_string(),
        };
        println!("1 new tweet:{}", tweet.summarize());
        println!("1 article new :{}", news_article.summarize());
    }

    #[test]
    fn test_traits_as_parameters() {
        pub fn notify(item: impl Summary) {
            println!("Breaking news!{}", item.summarize());
        }
        let news_article = NewsArticle {
            headline: "大事件".to_string(),
            location: "Guangzhou".to_string(),
            author: "BriLiang".to_string(),
            content: "增城：earthquake".to_string(),
        };
        notify(news_article);
    }

    #[test]
    fn test_trait_bound_syntax() {
        let tweet = Tweet {
            username: "horse_ebooks".to_string(),
            content: "of course, as you probably already know, people".to_string(),
            reply: false,
            retweet: false,
        };
        pub fn notify<T: Summary>(item: T) -> T {
            println!("Breaking news! {}", item.summarize());
            return item;
        }
        let tweet = notify(tweet);
        //定义trait 多个特性的边界
        pub fn notify2<T: Summary + Display>(_item: T) -> T {
            return _item;
        }
        pub fn notify3(_item: impl Summary + Display) {}
        let tweet = notify2(tweet);
        notify3(tweet);
    }

    #[test]
    fn test_trait_clearer_bound_with_where() {
        //在泛型里面定义多个trait的边界时，会使代码更难以阅读
        fn some_fun<T: Display + Clone + Debug, U: Clone + Debug>(t: T, u: U) -> i32 {
            println!("t is:{:?},u is:{:?}", t, u);
            0
        }
        //可以简写为
        fn some_function<T, U>(t: T, u: U) -> i32
            where T: Display + Clone + Debug, U: Clone + Debug
        {
            println!("t is:{:?},u is:{:?}", t, u);
            0
        }
        some_fun(1, 2);
        some_function(3, 5);
    }

    #[test]
    fn test_returning_type_with_implement_trait() {
        fn return_summarize(switch: bool) -> impl Summary {
            //由于编译器实现impl trait限制，不能返回两种不同类型
            let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from("of course, as you probably already know, people"),
                reply: false,
                retweet: false,
            };
            if switch {
                tweet
            } else {
//                NewsArticle {
//                    headline: String::from("Penguins win the Stanley Cup Championship!"),
//                    location: String::from("Pittsburgh, PA, USA"),
//                    author: String::from("Iceburgh"),
//                    content: String::from("The Pittsburgh Penguins once again are the best
//            hockey team in the NHL."),
//                }
                tweet
            }
        }
        let summarize = return_summarize(true);
        println!("return summarize is:{}", summarize.summarize());
    }

    //附加条件实现边界
    #[test]
    fn test_trait_bound_to_conditionally_implement_method() {
        #[derive(Debug)]
        struct Pair<T> {
            x: T,
            y: T,
        }
        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self {
                    x,
                    y,
                }
            }
        }
        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x > self.y {
                    println!("The largest member is x={}", self.x);
                } else {
                    println!("The largest member is y={}", self.y);
                }
            }
        }
        let pair: Pair<i32> = Pair { x: 1, y: 33 };
        pair.cmp_display();
        let pair2: Pair<&str> = Pair::new("apple", "banana");
        println!("pair2 is:{:?}", pair2);
        //可以实现任何的trait，设置实现的边界，这个是类似源码的实现
//        impl<T: Display> ToString for T {
//            fn to_string(&self) -> String {
//                return String::from("hahahah");
//            }
//        }
        let s = 3.to_string();
        println!("to string is:{}", s);
    }

    #[test]
    fn test_lifetime() {
        let r = 5;
        {
            let x;
            x = &r;
            println!("x:{}", x);
        }
        println!("r:{}", r);
    }

    #[test]
    fn test_generic_type_lifetimes_in_func() {
        fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
            //这意味着最长函数返回的引用的生命周期与传入的引用的生命周期中的较小者相同。
            //这么写，我们不知道传进来的引用，str1,str2的引用使用返回，
            //假如外层传进来的引用已经结束了生命周期，则这里返回的引用则是错误的
            // str1,str2的生命周期取小的值
            if str1.len() > str2.len() {
                str1
            } else {
                str2
            }
        }
        let result;
        let string1 = String::from("long string is lon");
        let string2 = String::from("xyz");
        {
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is:{}", result);
    }

    #[test]
    fn test_lifetime2() {
        fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
            x
        }
        let a = "apple";
        let b = "banana";
        let longest_str = longest(a, b);
        println!("longest is:{}", longest_str);
    }

    #[test]
    fn test_lifetime3() {
        //返回lifetime类型的值时，必须关联参数的lifetime，否则需要关联函数内部创建的值
        //内部所有权结束，返回它的引用会造成错误，所以不应该这么使用
//        fn longest<'a>(_x: &str, _y: &str) -> &'a str {
//            let result = String::from("really long string");
//            result.as_str()
//        }
    }

    //给struct 结构定义引用，需要给每个引用指定lifetime
    //此注释意味着一个ImportantExcerpt实例不能超过它在其part字段中保存的引用。
    #[derive(Debug)]
    struct ImportantExcerpt<'a, T> {
        part: &'a str,
        x: T,
    }

    #[test]
    fn test_lifetime_annotation_in_struct() {
        let novel = String::from("Call me Ishmael. Some years ago...");

        let first_sentence = novel.split(".").next().expect("Could not find a '.'");
        {
            let i = ImportantExcerpt { part: first_sentence, x: 1 };
            println!("ImportantExcerpt is:{:?}", i);
        }
    }

    //lifetime省略
    #[test]
    fn test_lifetime_elision() {
        //这个方法的写法是一种简写，在rust的早期版本并不支持
        fn first_word_before<'a>(s: &'a str) -> &'a str {
            s
        }
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                };
            };
            &s[..]
        }
        first_word("a");
        first_word_before("b");
    }

    #[test]
    fn test_lifetime_default_rule() {
        //编译器使用3个规则来确认生命周期引用，但不是显示注释时
        //1. 函数入参 2.3. 函数出参
        //规则一：每个引用参数都有它的生命周期参数，比如：
        fn foo<'a>(_x: &'a i32) {}
        //或者多个参数,以此类推：
        fn foo2<'a, 'b>(_x: &'a i32, _y: &'b i32) {}
        //规则二：如果入参只有一个参数，那么生命周期参数将附加在所有返回值参数上
        fn foo3<'a>(x: &'a i32) -> &'a i32 {
            x
        }
        //规则三：如果有不同生命周期的入参，并且有一个参数是&mut self或者&self，是一个方法
        //那么self的生命周期将会附加在所有的出参上，这个规则使得成员方法更好阅读和更少的符号

        //示例一：编译器是怎么做的
        fn first_word<'a>(s: &str) -> &str {
            s
        }
        //编译器应用第一个规则和第二个规则，则方法转化为：
        fn first_word_1_and_2<'a>(s: &'a str) -> &'a str {
            s
        }
        //所有参数都有生命周期声明了，编译器可以自己工作判断了，而不需要开发者自己注释

        //示例二：
        fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
            x
        }
        //应用规则一：
        fn longest_1<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
            x
        }
        //规则二和规则三并不能适用,并且无法确认返回值的声明周期，所以会导致编译器报错,所以要指定返回值的生命周期
        foo(&12);
        foo2(&12, &13);
        foo3(&12);
        first_word("a");
        first_word_1_and_2("a");
        longest("a", "b");
        longest_1("a", "b");
    }

    #[test]
    fn test_lifetime_annotation_in_method_definition() {
        impl<'a, T> ImportantExcerpt<'a, T> {
            //self 为啥不用生命周期声明？因为默认规则一已经在编译器添加
            fn level(&self) -> i32 {
                3
            }
            //默认返回值不生命声明周期的话，使用的是和对象声明一致的
            fn announce_and_return_part<'b>(&self, announcement: &'b str) -> (&'a str, &'b str) {
                println!("Attention please: {}", announcement);
                (self.part, announcement)
            }
        }
        let important = ImportantExcerpt {
            part: "briliang",
            x: 1.0,
        };
        let x = important.announce_and_return_part("hello");
        println!("announce is:{:?}", x);
        important.level();
    }

    #[test]
    fn test_static_lifetime() {
        //整个程序运行期间都保持的声明周期'static
        //该字符串的文本直接存储在程序的二进制文件中，该二进制文件始终可用。因此，所有字符串文字的生命周期都是“静态的”。
        let s: &'static str = "I have a static lifetime";
        println!("static str is:{}", s);
        let integer: &'static i32 = &12;
        println!("static int is:{}", integer);
    }

    #[test]
    fn test_generic_type_trait_bounds_and_lifetimes() {
        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where T: Display
        {
            println!("Announcement ! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        //这种写法会导致报错，因为字符串在堆上面分配，租借的内容在block块就会被回收
//        let str1 = String::from("BriLiang");
//        let announcement;
//        {
//            let str2 = String::from("hello");
//            announcement = longest_with_an_announcement(str1.as_str(), str2.as_str(), 123);
//        }
//        println!("largest str is:{}", announcement);

        let str1 = "BriLiang";
        let announcement;
        {
            let str2 = "hello";
            announcement = longest_with_an_announcement(str1, str2, 123);
        };
        println!("largest str is:{}", announcement);

        println!("---------------------------------------------");

        let str1 = "BriLiang";
        let announcement;
        {
            fn hahah(_str: &str) -> &str {
                let str2 = "hello";
                str2
            }
            let str2 = hahah("aa");
            //堆栈frame的创建不依赖作用域，也就是这个大括号，所以在这里创建的字符 str2和外层的str1永远在一个
            //stack frame 上所以，生命周期相同，不会报错。那为什么在另一个函数调用获取的 slice又没问题呢？
            //函数返回值依然在这个function，多个函数调用的顺序是，自顶向下，所以先回收hahaha，再回首test function
            //的栈帧，所以，生命周期还是一致
            announcement = longest_with_an_announcement(str1, str2, 123);
        };
        println!("largest str is:{}", announcement);
    }
}