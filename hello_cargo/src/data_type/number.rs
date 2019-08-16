pub fn number_operation() {
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    // remainder
    let remainder = 43 % 5;
    println!("sum is:{},difference is:{},product is:{},quotient is:{},remainder is:{}"
             , sum, difference, product, quotient, remainder)
}
#[cfg(test)]
mod tests {
    use crate::data_type::number::number_operation;

    #[test]
    fn test_float_data() {
        number_operation();
    }
}