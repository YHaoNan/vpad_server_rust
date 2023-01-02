use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;
use tokio::task::JoinHandle;
use crate::message::Message;
use crate::message::Message::Arp;
use crate::midi_connect::GLOBAL_MIDI_CONNECTOR;
use crate::pulse_generator::PulseGenerator;

/// Arp Handler是一个琶音处理器，它全局唯一
/// 它的主要作用就是以预定的速度和模式循环产生midi音符
///
pub struct ArpHandler {
    // 存储的实际是一个琶音器识别符到一个它的关闭通道的映射
    // 在关闭时，可以通过识别符找出通道，然后向其发送消息
    arp_tasks: Mutex<HashMap<String, tokio::sync::oneshot::Sender<()>>>,
    rate_scales: Vec<f64>
}

impl ArpHandler {
    pub fn handle(&self, identifier: String, message: Message) {
        if let Arp { note, velocity, state, method, rate, swing_pct,
                up_note_cnt, velocity_automation, dynamic_pct, bpm} = message {
            // 关闭arp卫语句
            if state == 0 {
                if let Some(tx) = self.arp_tasks.lock().unwrap().remove(&identifier) {
                    tx.send(());
                }
                return;
            }

            // 一拍的长度
            let beat_dur = Duration::from_secs_f64(60 as f64 / bpm as f64);
            // 根据指定的rate，计算出的一次琶音的长度（这里不考虑swing，只是一个标准除法）
            let ticktime = beat_dur.mul_f64(self.rate_scales[rate as usize]);
            println!("{:?}", ticktime);

            let (tx, mut rx) = tokio::sync::oneshot::channel();

            // 在tokio里提交一个阻塞任务的时候记得用spawn_blocking
            tokio::task::spawn_blocking(move || {
                // 根据一次琶音的长度以及swing_pct，计算带swing情况下的间隔列表，交给脉冲生成器来生成脉冲（间隔列表只有两个元素，放心）
                let mut pulse_generator = PulseGenerator::new(calcu_swing_pulse_tick_vec(ticktime, swing_pct));
                while let Some(_) = pulse_generator.next() {
                    // 若接到关闭通知，关闭并退出
                    if let Ok(()) = rx.try_recv() {
                        &mut pulse_generator.stop();
                        break;
                    }
                    let mut conn = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
                    conn.midi_note_message(note, velocity, state);
                }
            });

            self.arp_tasks.lock().unwrap().insert(identifier, tx);
        }
    }

}

/// 计算swing情况下，脉冲生成器的ticktime列表
fn calcu_swing_pulse_tick_vec(ticktime_in_given_bpm: Duration, swing_pct: i8) -> Vec<Duration> {
    let swing_pct = swing_pct as f64 / 100f64;
    let swing_dly = ticktime_in_given_bpm.mul_f64(swing_pct);
    vec![ticktime_in_given_bpm + swing_dly, ticktime_in_given_bpm - swing_dly]
}

const METHOD_NO_METHOD: i8 = 0;
const METHOD_UP: i8 = 1;
const METHOD_DOWN: i8 = 2;
const METHOD_UPDOWN: i8 = 3;
const METHOD_DOWNUP: i8 = 4;
const METHOD_3CHORD: i8 = 5;
const METHOD_7CHORD: i8 = 6;

const RATE_1_1: i8 = 0;
const RATE_1_2_D: i8 = 1;
const RATE_1_1_T: i8 = 2;
const RATE_1_2: i8 = 3;
const RATE_1_4_D: i8 = 4;
const RATE_1_2_T: i8 = 5;
const RATE_1_4: i8 = 6;
const RATE_1_8_D: i8 = 7;
const RATE_1_4_T: i8 = 8;
const RATE_1_8: i8 = 9;
const RATE_1_16_D: i8 = 10;
const RATE_1_8_T: i8 = 11;
const RATE_1_16: i8 = 12;
const RATE_1_32_D: i8 = 13;
const RATE_1_16_T: i8 = 14;
const RATE_1_32: i8 = 15;
const RATE_1_64_D: i8 = 16;
const RATE_1_32_T: i8 = 17;
const RATE_1_64: i8 = 18;
const RATE_1_64_T: i8 = 19;

lazy_static! {
    pub static ref GLOBAL_ARP_HANDLER: ArpHandler = ArpHandler {
        arp_tasks: Mutex::new(HashMap::new()),
        rate_scales: {
            let mut RATE_SCALE: Vec<f64> = vec![0.0f64; 20];
            RATE_SCALE[RATE_1_1 as usize] = 4f64;
            RATE_SCALE[RATE_1_1_T as usize] = RATE_SCALE[RATE_1_1 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_2 as usize] = 2f64;
            RATE_SCALE[RATE_1_2_D as usize] = RATE_SCALE[RATE_1_2 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_2_T as usize] = RATE_SCALE[RATE_1_2 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_4 as usize] = 1f64;
            RATE_SCALE[RATE_1_4_D as usize] = RATE_SCALE[RATE_1_4 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_4_T as usize] = RATE_SCALE[RATE_1_4 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_8 as usize] = 0.5f64;
            RATE_SCALE[RATE_1_8_D as usize] = RATE_SCALE[RATE_1_8 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_8_T as usize] = RATE_SCALE[RATE_1_8 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_16 as usize] = 0.25f64;
            RATE_SCALE[RATE_1_16_D as usize] = RATE_SCALE[RATE_1_16 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_16_T as usize] = RATE_SCALE[RATE_1_16 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_32 as usize] = 0.125f64;
            RATE_SCALE[RATE_1_32_D as usize] = RATE_SCALE[RATE_1_32 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_32_T as usize] = RATE_SCALE[RATE_1_32 as usize] * 2f64 / 3f64;
            RATE_SCALE[RATE_1_64 as usize] = 0.0625f64;
            RATE_SCALE[RATE_1_64_D as usize] = RATE_SCALE[RATE_1_64 as usize] * 1.5f64;
            RATE_SCALE[RATE_1_64_T as usize] = RATE_SCALE[RATE_1_64 as usize] * 2f64 / 3f64;
            RATE_SCALE
        }
    };
}
const VELOCITY_NO_AUTOMATION: i8 = 0;
const VELOCITY_UP: i8 = 1;
const VELOCITY_DOWN: i8 = 2;
const VELOCITY_UP_DOWN: i8 = 3;
const VELOCITY_DOWN_UP: i8 = 4;
const VELOCITY_STEP: i8 = 5;
const VELOCITY_RANDOM: i8 = 6;
