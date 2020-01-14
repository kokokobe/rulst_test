#[cfg(test)]
mod tests {
    #[test]
    fn creating_type_synonyms_with_type_aliases() {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
        //用于定义复杂类型
        type Thunk = Box<dyn Fn() + Send + 'static>;
        let _f: Thunk = Box::new(|| println!("hi"));
        fn _take_long_type(_f: Thunk) {}
        fn _returns_long_type() -> Thunk {
            Box::new(|| println!("hello"))
        }
    }

    #[test]
    fn never_type() {
        fn _bar(_text: &str) -> ! {
            panic!()
        }
        //不是任何类型的值可以转化为任何类型
    }

    #[test]
    fn default_size_trait() {
        fn __generic<T>(_t: T) {}
        //====>
        fn _generic<T: Sized>(_t: T) {}
        //可以放开非编译时明确大小
        fn ___generic<T: ?Sized>(_t: &T) {}
    }
}