///!
/// # Most Common Smart Pointer:
/// 1.Box<T> for allocating values on the heap
/// 2.Rc<T>, a reference counting type that enables multiple ownership
/// 3.Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
#[cfg(test)]
mod tests {
    ///
    /// - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    /// - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    /// - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    #[test]
    fn using_box_to_point_to_data_on_heap() {
        let b = Box::new(5);

    }
}