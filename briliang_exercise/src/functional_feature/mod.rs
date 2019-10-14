#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    //rust closures 是匿名函数，可以传入一个参数或者作为参数传递到别的函数
    //不像函数，可以在定义的地方使用捕获的值
    #[test]
    fn test_creating_an_abstraction_of_behavior_with_closures() {
        fn generate_workout(intensity: u32, random_number: u32) {
            //定义一个闭包，并赋值给变量，双竖线标识的是入参
            //为什么闭包不需要类型定义，返回值类型定义？因为它不需要暴露给其他人使用，不面向用户
            //所以闭包在有限的范围使用，编译器能够推断它的类型
            let expensive_closure = |num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            };
            //等价于
            let _expensive_closure_2 = |num: u32| -> u32{
                num
            };
            if intensity < 25 {
                println!("Today, do {} pushups!", expensive_closure(intensity));
                println!("Next, do {} situps!", expensive_closure(intensity));
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!("Today, run for {} minutes!", expensive_closure(intensity));
                };
            };
        }
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    //每一个闭包的都是唯一的，并且必须要实现Fn, FnMut Fnonce 中的一个特征
    //函数定义也同样实现了Fn 特征
    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        //代表是一个闭包
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
                Some(v) => v,
            }
        }
    }


    #[test]
    fn test_storing_closures_using_generic_parameters_and_the_fn_traits() {
        fn generate_workout(intensity: u32, random_number: u32) {
            let mut expensive_result = Cacher::new(|num| {
                println!("calculating slowly...");
                thread::sleep(Duration::from_secs(2));
                num
            });
            if intensity < 25 {
                println!(
                    "Today, do {} pushups!",
                    expensive_result.value(intensity)
                );
                println!(
                    "Next, do {} situps!",
                    expensive_result.value(intensity)
                );
            } else {
                if random_number == 3 {
                    println!("Take a break today! Remember to stay hydrated!");
                } else {
                    println!(
                        "Today, run for {} minutes!",
                        expensive_result.value(intensity)
                    );
                }
            }
        }

        let simulated_user_specified_value = 30;
        let simulated_random_number = 7;
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    #[test]
    #[should_panic]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let _v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    //Example of a closure that refers to a variable in its enclosing scope
    //Closures can capture values from their environment in three ways,
    // which directly map to the three ways a function can take a parameter:
    // taking ownership, borrowing mutably, and borrowing immutably.
    //FnOnce consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    //FnMut can change the environment because it mutably borrows values.
    //Fn borrows values from the environment immutably.
    #[test]
    fn capturing_the_environment_with_closures() {
        let x = 4;
        //闭包可以使用上下文中的变量,这样的使用closure 会有额外的内存开销
        let equal_to_x = |z| z == x;
        let y = 4;
        assert!(equal_to_x(y))
    }

    #[test]
    fn move_ownership_with_closure() {
        let x = vec![1, 2, 3];
        //强制把环境值x move到闭包
        let equal_to_x = move |z| z == x;
        //println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y));
    }

    #[test]
    fn processing_a_series_of_items_with_iterators() {
        //iterators are lazy，只有到调用方法和consume相关才会执行
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val);
        }
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3, ];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
        //使用for 循环迭代 v1_iter时为啥不用可变声明？因为for 默认编译器声明为可变，并且拥有所有权
        //iter()迭代器默认返回的是不可变得引用类型，如果想拥有所有权，则需要调用into_iter()
        //如果想获取可变的引用类型，可以调用iter_mut
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1: Vec<i32> = vec![1, 2, 3];
        //iter 是lazy的map，需要consume之后才能实际调用
        let v2: Vec<i32> = v1.iter().map(|x| x + 1).collect();
        println!("map increment value is:{:?}", v2);
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn using_closures_that_capture_their_environment() {
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }
        fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }
        let shoes = vec![
            Shoe { size: 10, style: "sneaker".to_string() },
            Shoe { size: 13, style: "sandal".to_string() },
            Shoe { size: 10, style: "boot".to_string() }
        ];
        let in_my_size = shoes_in_my_size(shoes, 10);
        assert_eq!(in_my_size, vec![Shoe { size: 10, style: String::from("sneaker") },
                                    Shoe { size: 10, style: String::from("boot") }, ])
    }

    #[test]
    fn creating_own_iterators_with_iterator_trait() {
        struct Counter {
            count: u32,
        }
        impl Counter {
            fn new() -> Counter {
                Counter { count: 0 }
            }
        }
        impl Iterator for Counter {
            //代表iterator会返回u32
            type Item = u32;
            fn next(&mut self) -> Option<Self::Item> {
                self.count += 1;
                if self.count < 6 {
                    Some(self.count)
                } else {
                    None
                }
            }
        }
        let counter = Counter::new();
        for x in counter {
            println!("custom iterator counter:{}", x);
        }
        let sum: u32 = Counter::new().zip(Counter::new().skip(1)).map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0).sum();
        assert_eq!(18, sum);
    }
}