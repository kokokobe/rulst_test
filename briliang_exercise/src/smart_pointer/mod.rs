///!
/// # Most Common Smart Pointer:
/// 1.Box<T> for allocating values on the heap
/// 2.Rc<T>, a reference counting type that enables multiple ownership
/// 3.Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
/// # Recap Smart Pointer:
/// 1.Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
/// 2.Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
/// 3.Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.
#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    /// - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    /// - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    /// - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    #[test]
    fn using_box_to_point_to_data_on_heap() {
        let b = Box::new(5);
        println!("the box integer is:{}", b);
    }

    #[test]
    fn box_with_recursive_type() {
        /// 这样使用是有问题的，因为rust不知道这样的递归结构的大小
//        enum List {
//            Cons(i32, List),
//            Nil,
//        }
//        use List::{Cons, Nil};
//        let list = Cons(1, Cons(2, Cons(3, Nil)));
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        use List::*;
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        println!("list is:{:#?}", list);
    }

    #[test]
    fn test_box_with_deref_trait() {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
        // using Box<T> like a reference
        let x = 5;
        let y = Box::new(x);
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    //枚举类型结构
    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            //为什么是返回引用是因为所有权不想改变
            &self.0
        }
    }

    #[derive(Debug)]
    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping customSmartPointer with data `{}`!", self.data);
        }
    }

    #[test]
    fn test_defining_our_own_smart_pointer() {
        let x = 5;
        let y = MyBox::new(x);
        assert_eq!(5, x);
        //实际调用了 *(y.deref())
        assert_eq!(5, *y);
    }

    #[test]
    fn test_implicit_deref_coercions_with_functions_and_methods() {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }
        hello("Rust");
        let m = MyBox::new(String::from("Rust"));
        //强迫式解耦 deref coercions
        hello(&m);
        //如果不使用上面的方式，使用原生的方式会非常繁琐
        hello(&(*m))
    }

    #[test]
    fn test_drop_trait() {
        let c = CustomSmartPointer {
            data: String::from("my stuff")
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff")
        };
        println!("CustomSmartPointers created. with:{:?},{:?}", c, d);
    }

    #[test]
    fn test_drop_trait_with_early_release() {
        let c = CustomSmartPointer {
            data: String::from("som data")
        };
        println!("CustomSmartPointer created.");
        drop(c);
        println!("CustomSmartPointer dropped before the end of main.");
    }

    ///单线程使用Rc<T>共享数据，可以有多个拥有者
    #[test]
    fn test_multiple_ownership_with_rc_in_single_thread() {
        #[derive(Debug)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        };
        use List::*;
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {},{:?}", Rc::strong_count(&a), a);
        //Rc::clone 不执行深拷贝，仅仅增加了引用计数
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {},{:?}", Rc::strong_count(&a), b);
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {},{:?}", Rc::strong_count(&a), c);
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }

    /// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
    /// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
    /// RefCell<T> is only for use in single-threaded scenarios
    #[test]
    fn test_ref_cell_with_interior_mutability_pattern() {
        pub trait Messenger {
            fn send(&self, msg: &str);
        }
        pub struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }
        impl<'a, T> LimitTracker<'a, T> where T: Messenger {
            pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value;
                let percentage_of_max = self.value as f64 / self.max as f64;
                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger.send("Urgent warning: You've used up over 98% of your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger.send("Waring: You've used up over 75% of your quota!");
                };
            }
        }
        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger { sent_messages: RefCell::new(vec![]) }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }

        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }

    #[test]
    fn test_combine_rc_and_ref_cell() {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }
        use List::*;
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
        //Rc 调用borrow_mut方法时自动解构为RefCell<>，再加上操作符解构RefCell 为它的值
        *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }

    #[test]
    fn test_cycle_reference_memory_leak() {
        //循环引用会造成引用计数无法达到0，所以没办法避免内存泄露。
        //内存泄露在严格意义上讲，rust并不认为是不安全
        use List::*;
        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }
        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }
        //a-->b-->a 循环引用问题
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial rc count ={}", Rc::strong_count(&a));
        println!("a next item ={:?}", a.tail());
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a rc count after b creation ={}", Rc::strong_count(&a));
        println!("b initial rc count={}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());
        if let Some(link) = a.tail() {
            //a-->b
            *link.borrow_mut() = Rc::clone(&b);
        };
        println!("b rc count after changing a={}", Rc::strong_count(&b));
        println!("a rc count after changing a={}", Rc::strong_count(&a));
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());
    }

    //Creating a Tree Data Structure: a Node with Child Nodes
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    #[test]
    fn test_turning_rc_to_weak() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        println!("leaf parent= {:?}", leaf.parent.borrow().upgrade());
    }

    #[test]
    fn test_visualize_count() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Default::default()),
                children: RefCell::new(vec![Rc::clone(&leaf)]),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak={}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
}