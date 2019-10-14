#[cfg(test)]
mod tests {
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }

    #[derive(Debug)]
    enum IpAddrKind2 {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    #[test]
    fn test_enum() {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("ip v4 :{:#?},ip v6:{:#?}", four, six);
    }

    #[test]
    fn test_enum2() {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        println!("hone address is:{:#?}", home);
        println!("loopback address is:{:#?}", loopback);
    }

    #[test]
    fn test_enum3() {
        let home = IpAddrKind2::V4(String::from("127.0.0.1"));
        let loopback = IpAddrKind2::V6(String::from("::1"));
        println!("hone address is:{:#?}", home);
        println!("loopback address is:{:#?}", loopback);
    }

    #[test]
    fn test_enum4() {
        //强大的枚举
        #[derive(Debug)]
        enum Message {
            Quit,
            //定义了关联struct
            Move { x: i32, y: i32 },
            //类似 tuple struct
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        // 枚举定义方法
        impl Message {
            fn call(&self) {
                //method body would be defined here
                println!("message is:{:#?}", self);
            }
        }
        let msg = Message::Write(String::from("hello"));
        msg.call();
        println!("quit is:{:#?}", Message::Quit);
        println!("move is:{:#?}", Message::Move { x: 1, y: 2 });
        println!("change color is:{:#?}", Message::ChangeColor(-1, -2, 1));
    }

    #[test]
    fn test_enum5() {
        let some_num = Some(5);
        let some_str = Some("hello");
        let absent_number: Option<i32> = None;
        println!("some num is:{:#?},some string is:{:#?},absent number is:{:#?}", some_num, some_str, absent_number);
        let x: i8 = 5;
        let y: Option<i8> = Some(9);
        let sum = x + y.unwrap_or(1);
        println!("sum is :{}", sum);
    }
}