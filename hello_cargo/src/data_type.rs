mod float;
mod number;
mod tuple_and_array;

pub fn test(){
    println!("data type");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_boolean() {
        let t = true;
        let f: bool = false;
        debug_assert_ne!(t, f);
    }
    #[test]
    fn test_character() {
        let _c = 'z';
        let _z = "Z";
        let _heart_eyed_cat = '😻';
        //println!("add character is:{}",c + z + heart_eyed_cat);
    }
}