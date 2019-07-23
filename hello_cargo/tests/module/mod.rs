mod raw_identifier;
pub mod sound;
mod shadowing;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {}

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    crate::module::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}

mod back_of_house {
    #[derive(Debug)]
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            let break_fast = BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
            println!("summer breakfast is:{:#?}",break_fast);
            break_fast
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::BreakFast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

mod performance_group {
    //重导入，当做成员变量使用
    pub use crate::module::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        //instrument::clarinet();
        //instrument::clarinet();
    }
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

#[cfg(test)]
mod tests {
    //使用super 类似于linux文件系统的..
    use super::{eat_at_restaurant, eat_at_restaurant2};
    // 使用as 可以避免重名
    use std::fmt::Result;
    use std::io::Result as IoResult;
    use super::raw_identifier;
    use super::plant;
    use super::menu;
    //一般枚举、结构体、其他项目直接使用use
    use std::collections::HashMap;
    use super::performance_group;
    use super::shadowing;

    #[test]
    fn test_module1() {
        eat_at_restaurant();
        eat_at_restaurant2();
    }

    #[test]
    fn test_module2() {
        fn function1() -> Result {
            // --snip--
            Ok(())
        }
        fn function2() -> IoResult<()> {
            // --snip--
            Ok(())
        }
        let result = function1();
        let result2 = function2();
        println!("result is :{:#?},io result is :{:#?}", result, result2);
    }

    #[test]
    fn test_separate_module() {
        println!("Hello, world!");
        crate::module::raw_identifier::test1();
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
}