use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::thread;
use mouse_common::mouse_operate;
use rand::{thread_rng, Rng};
fn main() {
    get_mouse_pos();
}

fn get_mouse_pos() {
    let mouse = mouse_operate::get_engine();
    let mut rng = thread_rng();
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect(" time went backwards");
    let mut x;
    let execute_time_sec = 10;
    loop {
        thread::sleep(Duration::from_millis(rng.gen_range(500, 1000)));
        x = mouse.get_mouse_pos();
        println!("position is :{:?}", x);
        let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect(" time went backwards");
        if end_time.as_millis() - start_time.as_millis() > 1000 * execute_time_sec {
            break;
        }
    }
}
