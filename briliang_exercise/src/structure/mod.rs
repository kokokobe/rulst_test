#[cfg(test)]
mod tests {
    use std::fmt;
    use std::fmt::{Formatter, Error};

    //#[derive(Debug)] 第一种实现打印对象的方法
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    //重写
    impl fmt::Debug for User {
        fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
            writeln!(f, "username is {:?},email is {:?},sign_in_count is {:?},active is {:?}"
                     , self.username, self.email, self.sign_in_count, self.active)
        }
    }

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    #[test]
    fn test_struct() {
        //初始化时不需要保持字段顺序
        let mut user1 = User {
            // rust 不允许单独的field设定为mutable
            email: String::from("someone@example.com"),
            username: String::from("Briliang"),
            active: true,
            sign_in_count: 1,
        };
        println!("username is:{}", user1.username);
        user1.email = String::from("kobetys@163.com");
        let user = build_user(String::from("kobetys@163.com"), String::from("kokokobe"));
        println!("user is:{:#?}", user);
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("antherusername567"),
            active: user1.active,
            sign_in_count: user1.sign_in_count,
        };
        println!("user2 is :{:#?}", user2);
        let user2 = User {
            email: String::from("a@example.com"),
            username: String::from("b"),
            //使用user1的值赋值其他field
            ..user1
        };
        println!("user2 is :{:#?}", user2);
    }

    #[test]
    fn test_tuple_struct() {
        //tuple struct 没有类型名字，只有类型
        //非常有用当你需要把整个tuple 命名为一个，并且不关心类型，可以和其他tuple区分开来
        #[derive(Debug)]
        struct Color(i32, i32, i32);
        #[derive(Debug)]
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        println!("black color is:{:#?}", black);
        println!("origin point is:{:#?}", origin);
        println!("black color rgb pixel R is :{}", black.0);
    }

    #[test]
    fn test_unit_like_struct() {
        //the unit type. Unit-like structs can be useful in situations in which you need
        // to implement a trait on some type but don’t have any data that you want to store in the type itself.
        #[derive(Debug)]
        struct Car {};
        let x = Car {};
        println!("unit like struct car is:{:#?}", x);
    }

    #[test]
    fn struct_example() {
        let width = 30;
        let height = 50;
        println!("The area of the rectangle is {} square pixels.", area(width, height));
        let rect1 = Rectangle {
            width,
            height,
        };
        //冒号告诉 println!宏使用Debug 特性来格式化
        println!("The area of the rectangle is {} square pixels with structure.rect1 is:{:#?}", area2(&rect1), rect1);
    }

    #[test]
    fn struct_method() {
        //enum or trait or struct will cotain method syntax
        #[derive(Debug)]
        struct Rectangle2 {
            width: u32,
            height: u32,
        }
        impl Rectangle2 {
            //成员方法
            fn area(&mut self) -> u32 {
                //Methods can take ownership of self, borrow self immutably as we’ve done here,
                // or borrow self mutably, just as they can any other parameter.
                self.width = 50;
                self.width * self.height
                // If we wanted to change the instance that we’ve called the method on as part of what the method does,
                // we’d use &mut self as the first parameter
            }
        }
        let mut rect = Rectangle2 {
            width: 30,
            height: 50,
        };
        println!("Thea area of the rectangle is {} square pixels with struct method", rect.area());
        println!("Thea rectangle changed to:{:#?}", rect);
    }

    #[test]
    fn test_struct_method2() {
        impl Rectangle {
            fn can_hold(&self, rectangle: &Rectangle) -> bool {
                self.width * self.height > rectangle.width * rectangle.height
            }
        }
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    #[test]
    fn test_associated_function() {
        //struct 关联函数常常用来返回一个新的struct示例
        impl Rectangle {
            fn square(size: u32) -> Rectangle {
                Rectangle { width: size, height: size }
            }
        }
        //关联函数使用双冒号调用::
        let sq = Rectangle::square(20);
        println!("square rectangle is:{:#?}", sq);
    }

    fn build_user(email: String, username: String) -> User {
        User {
            // rust 不允许单独的field设定为mutable
            //名字一样的时候可以简写
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    fn area2(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}