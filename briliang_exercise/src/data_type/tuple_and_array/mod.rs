#[cfg(test)]
mod tests {
    #[test]
    fn test_tuple() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        //rust 称之为解构(destructuring)
        let (x, y, z) = tup;
        println!("The value of x is:{}", x);
        println!("The value of y is:{}", y);
        println!("The value of z is:{}", z);
    }

    #[test]
    fn test_tuple_read() {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let five_hundred = tup.0;
        let six_point_four = tup.1;
        let one = tup.2;
        println!("0 is:{},1 is:{},2 is:{}", five_hundred, six_point_four, one);
    }

    #[test]
    fn test_array() {
        let a = [1, 2, 3, 4, 5];
        let b: [i64; 2] = [2342352352352, 6346546544565464];
        let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September"
            , "October", "November", "December"];
        println!("a is :{:?}", a);
        println!("b is :{:?}", b);
        println!("months is :{:?}", months);
        let complete = &a;
        println!("complete is :{:?}", complete);
        //equal let a = [3, 3, 3, 3, 3]
        let a = [3; 5];
        let first = a[0];
        let second = a[1];
        println!("first is :{},second is:{}", first, second)
    }
}