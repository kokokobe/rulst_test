mod data_type;
mod function;
mod control_flow;
mod ownership;
mod structure;
mod enums;
mod module;
mod collections;
mod error_handle;
mod generic_trait_lifetime;
#[test]
fn exploration() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[should_panic]
#[ignore]
fn another() {
    panic!("Make this test fail")
}

#[test]
fn larger_can_hold_smaller() {
    let larger = Rectangle { width: 8, height: 7 };
    let smaller = Rectangle { width: 5, height: 1 };
    assert!(larger.can_hold(&smaller));
    //assert_eq!()
    //assert_ne!()
}

#[test]
fn greeting_contains_name() {
    let name = "Carol";
    let result = greeting(name);
    //断言包含的提示信息
    assert!(result.contains(name), "Greeting did not contain name,value was `{}`", result);
}

//断言是否匹配特定的错误描述
#[test]
#[should_panic(expected = "Guess value must be less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}

#[test]
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 { Ok(()) } else { Err(String::from("two plus two does not equal four")) }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[derive(Debug)]
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }
        Guess {
            value
        }
    }
}