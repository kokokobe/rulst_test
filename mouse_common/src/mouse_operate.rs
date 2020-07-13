use rand::{thread_rng, Rng};
use rand::prelude::ThreadRng;
use enigo::*;
use std::thread;
use std::time::Duration;

pub struct Mouse {
    engine: Enigo,
}

pub fn get_engine() -> Mouse {
    Mouse {
        engine: Enigo::new(),
    }
}

impl Mouse {
    fn get_random_move_point(point: (i32, i32), point2: (i32, i32), mut rng: ThreadRng) -> (i32, i32) {
        let x1 = point.0;
        let y1 = point.1;
        let x2 = point2.0;
        let y2 = point2.1;
        let mut x_margin = (x1 - x2).abs();
        let mut y_margin = (y1 - y2).abs();
        if x_margin <= 1 {
            x_margin = 2;
        }
        if y_margin <= 1 {
            y_margin = 2;
        }
        let random_x = rng.gen_range(0, x_margin);
        let random_y = rng.gen_range(0, y_margin);
        (random_x, random_y)
    }

    pub fn start_end_point_click(&mut self, mut start: (i32, i32), mut end: (i32, i32), fuzzy: bool) {
        let mut rng = thread_rng();
        if fuzzy {
            let fuzzy_val = rng.gen_range(0, 5);
            start = (fuzzy_val + start.0, fuzzy_val + start.1);
            end = (fuzzy_val + end.0, fuzzy_val + end.1);
        }
        self.engine.mouse_move_to(start.0, start.1);
        println!("move to start {:?},end {:?}", start, end);
        self.engine.mouse_click(MouseButton::Left);
        let mut start_point = (start.0, start.1);

        loop {
            let x;
            let y;
            let random_point = Mouse::get_random_move_point(start_point, end, rng);
            if start_point.0 > end.0 {
                x = start_point.0 - random_point.0;
            } else if start_point.0 < end.0 {
                x = start_point.0 + random_point.0;
            } else {
                x = start_point.0;
            }
            if start_point.1 > end.1 {
                y = start_point.1 - random_point.1;
            } else if start_point.1 < end.1 {
                y = start_point.1 + random_point.1;
            } else {
                y = start_point.1;
            }
            self.engine.mouse_move_to(x, y);
            println!("move point x:{},y:{}", x, y);
            thread::sleep(Duration::from_millis(rng.gen_range(10, 100)));
            start_point.0 = x;
            start_point.1 = y;
            if x == end.0 && y == end.1 {
                self.engine.mouse_click(MouseButton::Left);
                println!("end ");
                break;
            }
        }
    }

    pub fn mouse_click(&mut self) {
        self.engine.mouse_click(MouseButton::Left);
    }

    pub fn get_mouse_pos(&self) -> (i32, i32) {
        Enigo::mouse_location()
    }
}

