use std::time::Duration;
use lazy_static::lazy_static;
use crate::midi_connect::GLOBAL_MIDI_CONNECTOR;
use tokio::sync::broadcast;

static DUR: Duration = Duration::from_millis(1);
// 广播通道，在一个pitchwheel事件被发送时打断之前所有，要不可能造成错乱
lazy_static! {
    static ref STOP_CHAN: (broadcast::Sender<()>, broadcast::Receiver<()>) = broadcast::channel(16);
}

pub fn move_to_smoothly(prev_pos: i8, pos: i8) {
    let vec:Vec<i8> =
        if prev_pos > pos { (pos..=prev_pos).rev().collect() } // 如果前一个更大，由于Rust只能创建正序Range，所以反过来创建并rev
        else { (prev_pos..=pos).collect() };                    // 前一个更小
    // 广播停止事件，让之前所有正在执行的pitchwheel停下来
    let _ = &STOP_CHAN.0.send(());
    tokio::task::spawn(async move {
        // 订阅停止事件
        let rx = &mut STOP_CHAN.0.subscribe();
        for value in vec {
            // 如果停止事件发生 跳出
            if rx.try_recv().is_ok() {
                break;
            }
            move_to(value);
            tokio::time::sleep(DUR).await;
        }
    });
}

pub fn move_to(pos: i8) {
    GLOBAL_MIDI_CONNECTOR.lock().unwrap().pitch_wheel_message(pos);
}