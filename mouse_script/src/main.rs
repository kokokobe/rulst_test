use std::time::{SystemTime, UNIX_EPOCH, Duration};
use mouse_common::mouse_operate;
use std::thread;
use rand::{thread_rng, Rng};

/// 远程操作设置
// 关卡一
// static BATTLE_POINT_1: (i32, i32) = (804, 766);
// // 关卡二
// static BATTLE_POINT_2: (i32, i32) = (797, 814);
// // 关卡三
// static BATTLE_POINT_3: (i32, i32) = (816, 873);
// // 关卡四
// static BATTLE_POINT_4: (i32, i32) = (811, 931);
// // 关卡五
// static BATTLE_POINT_5: (i32, i32) = (810, 981);
// // 喝水确认按钮
// static DRINKING_CONFIRM_POINT: (i32, i32) = (536, 933);
// // 确认挑战
// static FIGHT_POINT: (i32, i32) = (744, 999);
// // 大跳
// static ALL_SKIP_POINT: (i32, i32) = (765, 1017);
// // 大跳确认
// static ALL_SKIP_POINT_CONFIRM: (i32, i32) = (537, 930);
// // 喝水选择 100
// static DRINK_SELECT_100: (i32, i32) = (446, 798);
// // 喝水大水
// static DRINK_WHOLE: (i32, i32) = (445, 854);

///
///     家里配置
///     关卡
///     1： 724 678
///     2： 722 726
///     3:  720 775
///     4:  721 821
///     5:  731 866
///
///     fight: 660 887
///     all skip : 675 896
///
///     # 这两个一样
///     confirm : 476 822
///     drinking confirm : 477 824
///
// 关卡一
static BATTLE_POINT_1: (i32, i32) = (724, 678);
// 关卡二
static BATTLE_POINT_2: (i32, i32) = (722, 726);
// 关卡三
static BATTLE_POINT_3: (i32, i32) = (720, 775);
// 关卡四
static BATTLE_POINT_4: (i32, i32) = (721, 821);
// 关卡五
static BATTLE_POINT_5: (i32, i32) = (731, 866);

// 确认挑战
static FIGHT_POINT: (i32, i32) = (660, 887);
// 大跳
static ALL_SKIP_POINT: (i32, i32) = (675, 896);
// 大跳确认
static CONFIRM: (i32, i32) = (476, 822);

// 喝水选择 100
static DRINK_SELECT_100: (i32, i32) = (446, 798);
// 喝水大水
static DRINK_WHOLE: (i32, i32) = (399, 756);


fn main() {
    play()
}

fn play() {
    let mut mouse = mouse_operate::get_engine();
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
        mouse.start_end_point_click(battle_point, FIGHT_POINT, true);
        mouse.start_end_point_click(FIGHT_POINT, FIGHT_POINT, true);
        // 喝水选择
        mouse.start_end_point_click(DRINK_WHOLE, DRINK_WHOLE, false);
        mouse.start_end_point_click(DRINK_WHOLE, DRINK_WHOLE, false);
        // 喝水确认
        mouse.start_end_point_click(CONFIRM, CONFIRM, true);
        mouse.start_end_point_click(CONFIRM, CONFIRM, true);
        // 等待加载画面
        thread::sleep(Duration::from_millis(rng.gen_range(5000, 9000)));
        // 大跳确认
        mouse.start_end_point_click(ALL_SKIP_POINT, ALL_SKIP_POINT_CONFIRM, true);
        mouse.start_end_point_click(ALL_SKIP_POINT_CONFIRM, ALL_SKIP_POINT_CONFIRM, true);
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