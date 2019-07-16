mod raw_identifier;
mod sound;
mod shadowing;
mod data_type;
mod function;
mod control_flow;
mod ownership;
mod structure;
//一般枚举、结构体、其他项目直接使用use
use std::collections::{HashMap};

mod performance_group {
    //重导入，当做成员变量使用
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        //instrument::clarinet();
        //instrument::clarinet();
    }
}

fn main() {
    println!("Hello, world!");
    crate::raw_identifier::test1();
    raw_identifier::test1();
    let mut v = plant::Vegetable::new("squash");
    v.name = String::from("butternut squash");
    println!("{},id:{} are delicious", v.name, v.id);
    let _order1 = menu::Appetizer::Soup;
    let _order2 = menu::Appetizer::Salad;
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("map is :{:?}", map);
    performance_group::clarinet_trio();
    performance_group::instrument::clarinet();
    shadowing::shadowing();
}

//直接定义模块
mod plant {
    pub struct Vegetable {
        pub name: String,
        pub id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}