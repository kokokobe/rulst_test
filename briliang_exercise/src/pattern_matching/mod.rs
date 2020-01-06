#[cfg(test)]
mod tests {
    #[test]
    fn test_if_let() {
        let favourite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
        if let Some(color) = favourite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    #[test]
    fn test_while_loop() {
        let mut stack = vec![];
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    #[test]
    fn test_for_loop() {
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    #[test]
    fn test_function_parameter() {
        fn print_coordinates((x, y): &(i32, i32)) {
            println!("Current location:({},{})", x, y);
        }
        let point = (3, 5);
        print_coordinates(&point);
    }

    #[test]
    fn test_match_pattern_tuple() {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched,y = {:?}", y),
            _ => println!("Default case,x = {:?}", x)
        };
        println!("at the end: x= {:?},y = {:?}", x, y);

        let x = 1;
        //match or
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything")
        };
        //match range
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else")
        };
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("later ASCII letter"),
            _ => println!("something else")
        }
    }

    #[test]
    fn test_destructure_struct() {
        //destructing struct
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        let (k, j) = (p.x, 3);
        println!("k is:{},j is:{}", k, j);
        println!("p.x is:{}", p.x);
        assert_eq!(0, a);
        assert_eq!(7, b);
        //简化写法
        let Point { x, y } = p;
        println!("p is:{:?}", p);
        assert_eq!(0, x);
        assert_eq!(7, y);
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at:{}", x),
            Point { x: 0, y } => println!("On the y axis at:{}", y),
            Point { x, y } => println!("On neither axis:({},{})", x, y)
        };
        let ((_feet, _inches), Point { x: _a, y: _b }) = ((3, 10), Point { x: 3, y: -10 });
    }

    #[test]
    fn test_destructure_enum() {
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quite variant has not data to destructure");
            }
            //其实就是struct 解构
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => println!("Text message:{}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {},green {},and blue {}", r, g, b);
            }
        };
    }

    #[test]
    fn test_destructure_nested_struct_and_enums() {
        #[allow(dead_code)]
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        #[allow(dead_code)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {},green {},and blue{}", r, g, b);
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change the color to hue {}, saturation {}, and value {}", h, s, v);
            }
            _ => ()
        };
    }

    #[test]
    fn test_ignore_pattern() {
        //ignore function param
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter:{}", y);
        }
        foo(3, 4);
        // ignore part of value
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);
        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        };
        println!("setting is {:?}", setting_value);
        //Ignoring an Unused Variable by Starting Its Name with _
        let _x = 5;
        //Ignoring Remaining Parts of a Value with ..
        struct Point {
            x: i32,
            _y: i32,
            _z: i32,
        }
        let origin = Point { x: 0, _y: 0, _z: 0 };
        match origin {
            Point { x, .. } => {
                println!("x is :{}", x);
            }
        }
        let numbers = (2, 4, 8, 16, 32);
        match numbers {
            (first, .., last) => {
                println!("Some numbers: {},{}", first, last);
            }
        }
    }

    #[test]
    fn test_extra_conditionals_with_match_guards() {
        //Extra Conditionals with Match Guards，多条件匹配
        let num = Some(4);
        match num {
            Some(x) if x <= 5 => {
                println!("less than five:{}", x);
            }
            Some(x) => println!("{}", x),
            None => println!("num is none")
        };

        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == 7 => println!("Matched,n={}", n),
            _ => println!("Default case,x={:?}", x)
        };
        println!("at the end:x={:?},y={}", x, y);
    }

    #[test]
    fn test_at_bindings() {
        enum Message {
            Hello { id: i32 }
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range:{}", id_variable);
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in anther range");
            }
            Message::Hello { id } => {
                println!("Found some other id:{}", id);
            }
        }
    }
}