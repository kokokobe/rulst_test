#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{ErrorKind, Read, Write};
    use std::{io, fs};

    //主要分为可恢复的错误和不可恢复的错误
    // 可恢复的错误返回Result<T,E>
    #[test]
    #[should_panic]
    fn test_error_handle() {
        panic!("test panic")
    }

    #[test]
    #[should_panic]
    fn test_panic_backtrace() {
        let v = vec![1, 2, 3];
        let number = v[4];
        println!("number is:{}", number);
    }

    #[test]
    #[should_panic]
    fn test_recoverable_errors() {
        match File::open("hello.txt") {
            Ok(file) => file,
            Err(error) => {
                panic!("Problem opening the file:{:?}", error);
            }
        };
    }

    #[test]
    fn test_recoverable_matching_different_error() {
        let path = "hello.txt";
        //优化版，使用闭包closure
        create_and_del(path);
        create_file(path);
        fs::remove_file(path).expect("remove file error");
    }

    fn create_and_del(path: &str) {
        let file1 = File::open(path).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file: {:?}", error);
            }
        });
        println!("file1 is:{:?}", file1);
        //这个删除要等到函数结束，file1没有所属权时才会删除
        //Note that there is no guarantee that the file is immediately deleted
        // (e.g., depending on platform, other open file descriptors may prevent immediate removal).
        fs::remove_file(path).expect("remove file error");
    }

    fn create_file(path: &str) -> File {
        let file = match File::open(path) {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(path) {
                    Ok(file2) => file2,
                    Err(error_create) => panic!("Problem create file :{:?},error:{:?}", path, error_create)
                },
                other_error => panic!("Problem opening the file:{:?}", other_error),
            },
        };
        println!("create file is:{:?}", file);
        file
    }

    #[test]
    #[should_panic]
    fn test_shortcut_panic_error() {
        //使用match 来匹配相关的错误类型显得很冗余，可以使用unwrap 、expect等方法简化
        let path = "hello2.txt";
        File::open(path).unwrap();
        File::open(path).expect("Failed to open hello.txt");
    }

    #[test]
    fn test_propagate_error() {
        let path = "hello3.txt";
        let mut file1 = create_file(path);
        let result_write = file1.write("BriLiang".as_bytes());
        if result_write.is_err() {
            return;
        }
        fn read_username_from_file(path: &str) -> Result<String, io::Error> {
            let mut f = match File::open(path) {
                Ok(file) => file,
                Err(error) => return Err(error)
            };
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e)
            }
        }
        let result = read_username_from_file(path);
        println!("read file username is:{:?}", result);
        let remove_file = fs::remove_file(path);
        if remove_file.is_err() {
            panic!("error remove file");
        }
    }

    #[test]
    fn test_propagate_error_with_shortcut() {
        let path = "hello4.txt";
        let mut file = create_file(path);
        let result_write = file.write("BriLiang".as_bytes());
        if result_write.is_err() {
            return;
        }
        fn read_username_from_file(path: &str) -> Result<String, io::Error> {
            //如果问号的Result 值为error则直接返回，否则返回ok的值并继续执行
            let mut f = File::open(path)?;
            let mut s = String::new();
            //？实现了From trait，帮助它转换为不同的错误类型
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        let result = read_username_from_file(path);
        println!("shortcut propagate error result is:{:?}", result);
        //更短的优化写法
        fn read_username_from_file2(path: &str) -> Result<String, io::Error> {
            let mut s = String::new();
            File::open(path)?.read_to_string(&mut s)?;
            Ok(s)
        }
        let result = read_username_from_file2(path);
        println!("much more shortcut propagate error result is:{:?}", result);

        fn read_username_from_file3(path: &str) -> Result < String, io::Error > {
            fs::read_to_string(path)
        }
        let result = read_username_from_file3(path);
        println!("most shortcut propagate error result is:{:?}", result);
        fs::remove_file(path).expect(&("remove file error:".to_owned() + path));
    }

    #[test]
    fn test_panic_or_result_prototype_and_tests(){

    }

}