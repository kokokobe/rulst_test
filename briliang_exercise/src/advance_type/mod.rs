#[cfg(test)]
mod tests {
    #[test]
    fn creating_type_synonyms_with_type_aliases() {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
    }
}