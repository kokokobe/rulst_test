mod mouse_operate;

use enigo::*;
use std::time::{SystemTime, UNIX_EPOCH};
use mouse_operate::start_end_point_click;

fn main() {
    let mut enigo = Enigo::new();
    // 喝水确认按钮
    let drinking_confirm_point = (200, 200);
    // 挑战关卡按钮
    let fight_point = (1000, 1000);
    // 关卡
    let battle_point = (500, 300);
    // 大跳
    let all_skip_point = (500, 500);
    // 大跳确认
    let all_skip_point_confirm = (300, 300);

    let execute_time_sec = 10;
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect(" time went backwards");
    loop {
        //选关，开始挑战
        start_end_point_click(battle_point, fight_point, &mut enigo, true);
        //大跳确认
        start_end_point_click(all_skip_point, all_skip_point_confirm, &mut enigo, true);
        // 喝水确认
        start_end_point_click(all_skip_point_confirm, drinking_confirm_point, &mut enigo, true);

        let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect(" time went backwards");
        if end_time.as_millis() - start_time.as_millis() > 1000 * execute_time_sec {
            break;
        }
    }

}