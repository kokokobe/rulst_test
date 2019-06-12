fn float_data_type(){
    let x = 2.0;
    let y: f32 = 3.0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std;
    #[test]
    fn test_float_data(){
        super::float_data_type();
        println!("test float data");
    }
}