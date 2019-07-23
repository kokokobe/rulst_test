#[cfg(test)]
mod tests {
    #[test]
    fn test_float_data() {
        float_data_type();
        println!("test float data");
    }

    fn float_data_type() {
        let x = 2.0;
        let y: f32 = 3.0;
        println!("x is:{},y is:{}", x, y);
    }
}