#[cfg(test)]
mod tests {
    // `block_on` blocks the current thread until the provided future has run to
    // completion. Other executors provide more complex behavior, like scheduling
    // multiple futures onto the same thread.
    use futures::executor::block_on;

    #[test]
    fn primer() {
        async fn hello_world(){
            println!("hello, world!");
        }
        // Nothing is printed
        let future = hello_world();
        // `future` is run and "hello, world!" is printed
        block_on(future);
    }
}