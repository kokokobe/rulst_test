#[cfg(test)]
mod tests {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
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
}