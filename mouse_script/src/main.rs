use std::time::{SystemTime, UNIX_EPOCH, Duration};
use mouse_common::mouse_operate;
use std::thread;
use rand::{thread_rng, Rng};
// 关卡一
static BATTLE_POINT_1: (i32, i32) = (804, 766);
// 关卡二
static BATTLE_POINT_2: (i32, i32) = (797, 814);
// 关卡三
static BATTLE_POINT_3: (i32, i32) = (816, 873);
// 关卡四
static BATTLE_POINT_4: (i32, i32) = (811, 931);
// 关卡五
static BATTLE_POINT_5: (i32, i32) = (810, 981);

fn main() {
    play()
}

fn play() {
    let mut mouse = mouse_operate::get_engine();
    // 喝水确认按钮
    let drinking_confirm_point = (536, 933);
    // 确认挑战
    let fight_point = (744, 999);
    // 大跳
    let all_skip_point = (765, 1017);
    // 大跳确认
    let all_skip_point_confirm = (537, 930);

    let execute_time_sec = 60 * 60;
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
        .expect(" time went backwards");
    let mut rng = thread_rng();
    loop {
        let battle_point = BATTLE_POINT_2;
        //选关，开始挑战
        mouse.start_end_point_click(battle_point, battle_point, true);
        mouse.start_end_point_click(battle_point, battle_point, true);
        mouse.start_end_point_click(battle_point, battle_point, true);
        // 开始挑战
        mouse.start_end_point_click(battle_point, fight_point, true);
        mouse.start_end_point_click(fight_point, fight_point, true);
        // 喝水确认
        mouse.start_end_point_click(drinking_confirm_point, drinking_confirm_point, true);
        mouse.start_end_point_click(drinking_confirm_point, drinking_confirm_point, true);
        // 等待加载画面
        thread::sleep(Duration::from_millis(rng.gen_range(5000, 9000)));
        // 大跳确认
        mouse.start_end_point_click(all_skip_point, all_skip_point_confirm, true);
        mouse.start_end_point_click(all_skip_point_confirm, all_skip_point_confirm, true);;
        // 等待执行胜利
        thread::sleep(Duration::from_millis(rng.gen_range(1000, 2000)));
        // 多点几次
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        thread::sleep(Duration::from_millis(rng.gen_range(5000, 8000)));
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        mouse.mouse_click();
        //多选几次关卡
        mouse.start_end_point_click(battle_point, battle_point, true);
        mouse.start_end_point_click(battle_point, battle_point, true);
        mouse.start_end_point_click(battle_point, battle_point, true);

        let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
            .expect(" time went backwards");
        if end_time.as_millis() - start_time.as_millis() > 1000 * execute_time_sec {
            break;
        }
    }
}