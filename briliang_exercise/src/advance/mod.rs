
#[cfg(test)]
mod tests {
    use std::ops::Add;
    use std::fmt;
    use std::fmt::{Display, Formatter, Error};
    use log::{info};

    ///Those superpowers include the ability to:
        ///
        ///Dereference a raw pointer
        ///Call an unsafe function or method
        ///Access or modify a mutable static variable
        ///Implement an unsafe trait
    #[test]
    fn test_unsafe() {
        //Different from references and smart pointers, raw pointers:
        //
        //Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
        //Aren’t guaranteed to point to valid memory
        //Are allowed to be null
        //Don’t implement any automatic cleanup
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        let address = 0x12344usize;
        let r = address as *const i32;
        println!(" r is :{:?}", r);
        unsafe {
            println!("r1 is:{}", *r1);
            println!("r2 is:{}", *r2);
        }
    }


    #[test]
    #[allow(dead_code)]
    fn test_unsafe_function() {
        use std::slice;
        unsafe fn dangerous() {}
        unsafe {
            dangerous();
        }
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();
            assert!(mid <= len);
            unsafe {
                (slice::from_raw_parts_mut(ptr, mid), slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
            }
        }

        let address = 0x01234usize;
        let r = address as *mut i32;
        let _slice: &[i32] = unsafe {
            slice::from_raw_parts_mut(r, 10000)
        };
        //println!("slice is {:?}", _slice);
//        let &mut str: [i32] = [22];
//        let (a, b) = split_at_mut( str, 3);
//        println!("split tuple a is:{:?},b is:{:?}", a, b);
    }

    #[test]
    fn test_extern() {
        //extern always not safe
        extern "C" {
            fn abs(input: i32) -> i32;
        }
        unsafe {
            println!("Absolute value of -3 according to C:{}", abs(-3));
        }

        #[no_mangle]
        pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!")
        }
    }

    #[test]
    fn test_access_or_modify_mutable_static_var() {
        static HELLO_WORLD: &str = "Hello, world";
        println!("name is:{}", HELLO_WORLD);
        static mut COUNTER: u32 = 0;
        fn add_to_count(inc: u32) {
            unsafe {
                COUNTER += inc;
            }
        }
        add_to_count(3);
        unsafe {
            println!("COUNTER:{}", COUNTER);
        }
    }

    #[test]
    fn implement_unsafe_trait() {
        unsafe trait Foo {
            //methods go here
        }
        //自己维护不安全的代码
        unsafe impl Foo for i32 {
            //method implementations go here
        }
    }

    #[test]
    fn advanced_trait_associated_type() {
        // type item是一个关联类型
        //一个占位符的类型
        //好像和泛型有点类似？？
        pub trait Iterator {
            type Item;
            fn next(&mut self) -> Option<Self::Item>;
        }
        //关联类型只有一种具体类型，而泛型可以定义多种类型针对一个类
        impl Iterator for String {
            type Item = i32;

            fn next(&mut self) -> Option<Self::Item> {
                unimplemented!()
            }
        }
    }

    #[test]
    fn default_generic_type_parameters_and_operator_overloading() {
        //重载操作符
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        impl Add for Point {
            type Output = Point;

            fn add(self, rhs: Self) -> Self::Output {
                Point {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                }
            }
        }
        assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
        //pub trait Add<Rhs=Self> { 这种叫做默认类型,RHS 的是right hand side的缩写
        //如果我们不填写这个类型，默认类型就是自己
        //下面我们不使用默认类型，而是定义这个类型

        struct Millimeters(u32);
        struct Meters(u32);
        impl Add<Meters> for Millimeters {
            type Output = Millimeters;

            fn add(self, meter: Meters) -> Self::Output {
                Millimeters(self.0 + (meter.0 * 1000))
            }
        }
    }

    #[test]
    fn different_trait_same_method_name() {
        trait Pilot {
            fn fly(&self);
        }
        trait Wizard {
            fn fly(&self);
        }
        struct Human;
        impl Pilot for Human {
            fn fly(&self) {
                println!("This is your captain speaking.")
            }
        }
        impl Wizard for Human {
            fn fly(&self) {
                println!("Up!")
            }
        }
        impl Human {
            fn fly(&self) {
                println!("*waving arms furiously*")
            }
        }
        //默认直接调用方法
        let person = Human;
        person.fly();
        //调用飞行员trait的方法实现
        Pilot::fly(&person);
        Wizard::fly(&person);

        trait Animal {
            fn baby_name() -> String;
        }
        struct Dog;
        impl Dog {
            fn baby_name() -> String {
                String::from("Spot")
            }
        }
        impl Animal for Dog {
            fn baby_name() -> String {
                String::from("puppy")
            }
        }
        println!("A baby dog is called a {}", Dog::baby_name());
        //Animal baby_name关联方法，没有self类型，无法判断调用哪个方法
        //println!("A baby dog is called a {}", Animal::baby_name());
        //为了调用这个Dog的 Animal trait 方法，需要使用 完全限定语法
        //<Type as Trait>::function(receiver_if_method, next_arg, ...);
        println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    }

    #[test]
    fn super_trait() {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

        trait OutlinePrint: fmt::Display {
            fn outline_print(&self) {
                let output = self.to_string();
                let len = output.len();
                info!("{}", "*".repeat(len + 4));
                info!("*{}*", " ".repeat(len + 2));
                info!("* {} *", output);
                info!("*{}*", " ".repeat(len + 2));
                info!("{}", "*".repeat(len + 4));
            }
        }

        struct Point {
            x: i32,
            y: i32,
        }
        impl OutlinePrint for Point {}
        impl Display for Point {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                write!(f, "({},{})", self.x, self.y)
            }
        }
        let point = Point {
            x: 2,
            y: 33,
        };
        point.outline_print();
    }

    #[test]
    fn newtype_pattern(){
        //可以通过这种方式实现Vec的 display trait
        struct Wrapper(Vec<String>);
        impl fmt::Display for Wrapper {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
                write!(f, "[{}]", self.0.join(","))
            }
        }
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}