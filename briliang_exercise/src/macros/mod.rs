#[cfg(test)]
mod tests {
    #[test]
    fn declarative_macro() {
        //宏可以实现某个类型的trait
        //宏更难阅读和理解，因为你在在编写生成rust代码的代码
        //如果使用函数来实现这个，你无法猜测数据类型和数据参数个数

        //#[macro_export]
        macro_rules! vec {
              ($($x:expr),*)=>{
                {
                    let mut temp_vec = Vec::new();
                    //这一段表示每一次匹配所做的事情
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
              };
        }
        let v: Vec<u32> = vec![1, 2, 3];
        println!("vector is:{:?}", v);
    }

    #[test]
    fn procedural_macro() {
        use crate::HelloMacro;
        use hello_marco_derive::HelloMacro;
        #[derive(HelloMacro)]
        struct Pancake;
        Pancake::hello_macro();
    }

    #[test]
    fn attribute_like_macro(){
        //和派生式宏差不多，只不过是可以自己定义属性，
        //派生宏仅仅使用在struct、enum上
        //属性红则可以使用在函数上

    }
}