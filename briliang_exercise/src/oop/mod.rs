#[cfg(test)]
mod tests {
    #[test]
    fn trait_object() {
        pub trait Draw {
            fn draw(&self);
        }
        pub struct Screen {
            //定义了一个 trait object
            //动态分配在编译时无法知道全部类型，所以增加了灵活性，降低了部分性能
            pub components: Vec<Box<dyn Draw>>,
        }
        impl Screen {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
        #[derive(Debug)]
        pub struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }
        impl Draw for Button {
            fn draw(&self) {
                //draw a button
                println!("draw a button:{:?}", self);
            }
        }
        #[derive(Debug)]
        pub struct SelectBox {
            width: u32,
            height: u32,
            options: Vec<String>,
        }
        impl Draw for SelectBox {
            fn draw(&self) {
                //draw a select box
                println!("draw select box:{:?}", self);
            }
        }
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No")
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ]
        };
        screen.run();
    }

    #[test]
    fn trait_object_safety() {
        //A trait is object safe if all the methods defined in the trait have the following properties:
        //The return type isn’t Self.
        //There are no generic type parameters.
        //compile error
        //        pub struct Screen {
        //            pub components: Vec<Box<dyn Clone>>
        //        }
    }

    #[test]
    fn impl_object_oriented_design_pattern() {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }
        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }
            pub fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(&self)
            }
            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }
            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }
        }
        trait State {
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, _post: &'a Post) -> &'a str {
                ""
            }
        }
        struct Draft {}
        struct PendingReview {}
        struct Published {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {})
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                Box::new(Published {})
            }
        }
        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }

            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
        }


        let mut post = Post::new();
        let content = "I ate a salad for lunch today";
        post.add_text(content);
        assert_eq!("", post.content());
        post.request_review();
        assert_eq!("", post.content());
        post.approve();
        assert_eq!(content, post.content());
    }

    #[test]
    fn impl_object_oriented_design_pattern2() {
        pub struct Post {
            content: String
        }
        pub struct DraftPost {
            content: String
        }
        pub struct PendingReviewPost {
            content: String,
        }
        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new()
                }
            }
            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text)
            }
            pub fn request_review(self) -> PendingReviewPost {
                PendingReviewPost { content: self.content }
            }
        }
        impl PendingReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content
                }
            }
        }

        let mut post = Post::new();
        let x = "I ate a salad for lunch today";
        post.add_text(x);
        let post = post.request_review();
        let post = post.approve();
        assert_eq!(x, post.content());

    }
}