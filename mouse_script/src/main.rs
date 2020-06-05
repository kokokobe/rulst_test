use enigo::*;
use rand::{thread_rng, Rng};
use std::{thread};
use std::time::Duration;
use rand::prelude::ThreadRng;

fn main() {
    let mut enigo = Enigo::new();
    // enigo.key_click(Key::Meta);
    // paste
    // enigo.mouse_move_to(1000, 800);
    // enigo.mouse_click(MouseButton::Right);
    // enigo.mouse_move_relative(100, 100);
    let drinking_point = (200, 200);
    let fight_point = (1000, 1000);
    loop {
        start_end_point_click(fight_point, drinking_point, &mut enigo);
    }


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

    fn start_end_point_click(start: (i32, i32), end: (i32, i32), enigo: &mut Enigo) {
        enigo.mouse_move_to(start.0, start.1);
        println!("move to start x:{},y:{}", start.0, start.1);
        enigo.mouse_click(MouseButton::Left);
        let mut start_point = (start.0, start.1);
        let mut rng = thread_rng();
        loop {
            let x;
            let y;
            let random_point = get_random_move_point(start_point, end, rng);
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
            enigo.mouse_move_to(x, y);
            println!("move point x:{},y:{}", x, y);
            thread::sleep(Duration::from_millis(10));
            start_point.0 = x;
            start_point.1 = y;
            if x == end.0 && y == end.1 {
                enigo.mouse_click(MouseButton::Left);
                println!("end ");
                break;
            }
        }
    }
}