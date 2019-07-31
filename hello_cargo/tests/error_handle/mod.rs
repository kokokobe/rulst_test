#[cfg(test)]
mod tests {

    //主要分为可恢复的错误和不可恢复的错误
    // 可恢复的错误返回Result<T,E>
    #[test]
    #[should_panic]
    fn test_error_handle() {
        panic!("test panic")
    }

}