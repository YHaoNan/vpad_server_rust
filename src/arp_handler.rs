use std::cmp::{max, min};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;
use midi_control::Channel;
use rand::Rng;
use crate::circle_container::CircleContainer;
use crate::message::Message;
use crate::message::Message::Arp;
use crate::midi_connect::GLOBAL_MIDI_CONNECTOR;
use crate::pulse_generator::PulseGenerator;

/// Arp Handler是一个琶音处理器，它全局唯一
/// 它的主要作用就是以预定的速度和模式循环产生midi音符
pub struct ArpHandler {
    // 存储的实际是一个琶音器识别符到一个它的关闭通道的映射
    // 在关闭时，可以通过识别符找出通道，然后向其发送消息
    arp_tasks: Mutex<HashMap<String, tokio::sync::oneshot::Sender<()>>>,
    velocity_automation_span: Vec<i8>,
    rate_scales: Vec<f64>
}

impl ArpHandler {
    pub fn handle(&self, identifier: String, message: Message) {
        if let Arp { state, .. } = message {
            // state == 1 => 开启arp
            if state == 1 { self.start_arp_task(identifier, message) }
            else { self.stop_arp_task(identifier) }
        }
    }

    fn start_arp_task(&self, identifier: String, message: Message) {
        let channel = if let Arp {channel,..} = message { channel } else {1};
        if let Some((mut note_generator, mut velocity_generator, pulse_generator)) = build_requirements(message, self) {
            let (stop_sender, mut stop_receiver) = tokio::sync::oneshot::channel();
            tokio::task::spawn_blocking(move || {
                let mut last_note: Option<i8> = None;
                for _ in pulse_generator {
                    if let Some(note) = last_note { send_midi_off(note, channel); }
                    if let Ok(()) = stop_receiver.try_recv() { break; }

                    last_note = Some(note_generator.next().unwrap());
                    send_midi_on(last_note.unwrap(), velocity_generator.next().unwrap(), channel);
                }
            });
            self.arp_tasks.lock().unwrap().insert(identifier, stop_sender);
        }
    }

    fn stop_arp_task(&self, identifier: String) {
        if let Some(tx) = self.arp_tasks.lock().unwrap().remove(&identifier) {
            tx.send(()).expect("error when stop arp task");
        }
    }

}

fn send_midi_on(note: i8, velocity: i8, channel: i8) {
    if note < 0 { return; }
    let mut conn = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
    conn.midi_note_message_with_channel_number(note, velocity, 1, channel);
}

fn send_midi_off(note: i8, channel: i8) {
    if note < 0 { return; }
    let mut conn = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
    conn.midi_note_message_with_channel_number(note, 0, 0, channel);
}

fn build_requirements(message: Message, arp_handler: &ArpHandler) -> Option<(CircleContainer<i8>, CircleContainer<i8>, PulseGenerator)> {
    if let Arp { note, velocity, method, rate, swing_pct,
        up_note_cnt, velocity_automation, dynamic_pct, bpm, ..} = message {
        // duration per beat
        let beat_dur = Duration::from_secs_f64(60 as f64 / bpm as f64);
        // duration once arp (no swing)
        let once_arp_dur = beat_dur.mul_f64(arp_handler.rate_scales[rate as usize]);
        let velocity_automation_span = arp_handler.velocity_automation_span[rate as usize];

        let note_generator = build_note_generator(note, method, up_note_cnt);
        let velocity_generator = build_velocity_generator(velocity, velocity_automation, dynamic_pct,  velocity_automation_span);
        let pulse_generator = build_pulse_generator(once_arp_dur, swing_pct);

        Some((note_generator, velocity_generator, pulse_generator))
    } else {
        None
    }
}

// 从0..to进行计数，并存储到Vec中，过程中可以应用一个函数
fn count_to_vec<F>(to: i8, mut f: F) -> Vec<i8>
where F: FnMut(i8) -> i8, {
    let mut vec = Vec::with_capacity(to as usize);
    for i in 0..to {
        vec.push(f(i));
    }
    vec
}

// count_to，计数到达一半后，开始转为向下计数
fn count_to_vec_up_down<F>(to: i8, mut f: F) -> Vec<i8>
    where F: FnMut(i8) -> i8, {
    let mid = to / 2;
    let even = to % 2 == 0;
    count_to_vec(to, |i| {
        if to / (i + 1) > 1 {
            f(i)
        } else {
            let offset = if even { 1 } else { 0 };
            let i = mid - (offset + i - mid);
            f(i)
        }
    })
}

fn build_velocity_generator(base_velocity: i8, velocity_automation: i8, dynamic_pct: i16, velocity_automation_span: i8) -> CircleContainer<i8> {
    /**
     * dynamic_pct为0~200，代表着力度的另一端，我们假设它是another_velocity，则力度生成器产生的力度将在base_velocity到another_velocity的范围内
     *      当dynamic_pct小于100时，力度生成器的范围是[another_velocity, base_velocity]
     *      当dynamic_pct大于100时，力度生成器的范围是[base_velocity, another_velocity]
     *      当dynamic_pct等于100时，谁前谁后无所谓，此时`base_velocity == another_velocity`，力度生成器将生成恒定的值
     * 特别注意，因为MIDI协议限制音符力度最大值为127，所以another_velocity必须限制在127以下
     */
    let another_velocity = (base_velocity as f64 * (dynamic_pct as f64 / 100f64)) as i16;
    let another_velocity = if another_velocity > 127 { 127i8 } else { another_velocity as i8 };

    let min_velo = min(base_velocity, another_velocity);
    let max_velo = max(base_velocity, another_velocity);
    let step = (max_velo - min_velo) / velocity_automation_span;

    let mut rng = rand::thread_rng();

    let velocity_vec = match velocity_automation {
        VELOCITY_UP => {
            count_to_vec(velocity_automation_span, |i| min_velo + i * step)
        }
        VELOCITY_DOWN => {
            count_to_vec(velocity_automation_span, |i| max_velo - i * step)
        }
        VELOCITY_UP_DOWN => {
            count_to_vec_up_down(velocity_automation_span, |i| min_velo + i * step * 2)
        }
        VELOCITY_DOWN_UP => {
            count_to_vec_up_down(velocity_automation_span, |i| max_velo - i * step * 2)
        }
        VELOCITY_RANDOM => {
            count_to_vec(velocity_automation_span, |_| rng.gen_range(min_velo..=max_velo))
        }
        VELOCITY_STEP => {
            count_to_vec(velocity_automation_span, |i| if i % 2 == 0 { max_velo } else { min_velo })
        }
        _ => {
            count_to_vec(velocity_automation_span, |_| base_velocity)
        }
    };
    CircleContainer::new(velocity_vec)
}

fn build_note_generator(base_note: i8, method: i8, up_note_cnt: i8) -> CircleContainer<i8> {
    let note_offsets = match method {
        METHOD_UP => {
            count_to_vec(up_note_cnt, |i| i * 12)
        }
        METHOD_DOWN => {
            count_to_vec(up_note_cnt, |i| -(i * 12))
        }
        METHOD_UPDOWN => {
            count_to_vec_up_down(up_note_cnt, |i| i * 12)
        }
        METHOD_DOWNUP => {
            count_to_vec_up_down(up_note_cnt, |i| -(i * 12))
        }
        METHOD_3CHORD => {
            // 3和弦之间，音符间隔是0，4，7，将它们包装到CircleContainer里，以循环读取
            // 可我们期待的序列是 0, 4, 7, 12, 16, 19 ... ，所以需要乘以系数 i / 3 * 12
            // 其它和弦一致
            let mut c = CircleContainer::new(vec![0, 4, 7]);
            count_to_vec(up_note_cnt, |i| (i / 3 * 12) + c.next().unwrap())
        }
        METHOD_7CHORD => {
            let mut c = CircleContainer::new(vec![0, 4, 7, 11]);
            count_to_vec(up_note_cnt, |i| (i / 4) * 12 + c.next().unwrap())
        }
        METHOD_3MINCHORD => {
            let mut c = CircleContainer::new(vec![0, 3, 7]);
            count_to_vec(up_note_cnt, |i| (i / 3) * 12 + c.next().unwrap())
        }
        METHOD_7MINCHORD => {
            let mut c = CircleContainer::new(vec![0, 3, 7, 11]);
            count_to_vec(up_note_cnt, |i| (i / 4) * 12 + c.next().unwrap())
        }
        _ => {
            Vec::from_iter(0..up_note_cnt)
        }
    };
    CircleContainer::new(
        note_offsets.iter().map(|x| base_note + x).collect()
    )
}

fn build_pulse_generator(ticktime_in_given_bpm: Duration, swing_pct: i8) -> PulseGenerator {
    let swing_pct = swing_pct as f64 / 100f64;
    let swing_dly = ticktime_in_given_bpm.mul_f64(swing_pct);
    PulseGenerator::new(
        vec![ticktime_in_given_bpm + swing_dly, ticktime_in_given_bpm - swing_dly]
    )
}

#[cfg(test)]
mod test_arp_calcu_function {
    use crate::arp_handler::{build_note_generator, METHOD_DOWN, METHOD_DOWNUP, METHOD_UP, METHOD_UPDOWN};

    #[test]
    fn test_method_up_odd() {
        let mut c = build_note_generator(0, METHOD_UP, 3);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(12));
        assert_eq!(c.next(), Some(24));
    }

    #[test]
    fn test_method_up_even() {
        let mut c = build_note_generator(0, METHOD_UP, 4);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(12));
        assert_eq!(c.next(), Some(24));
        assert_eq!(c.next(), Some(36));
    }

    #[test]
    fn test_method_down_odd() {
        let mut c = build_note_generator(0, METHOD_DOWN, 3);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(-12));
        assert_eq!(c.next(), Some(-24));
    }

    #[test]
    fn test_method_down_even() {
        let mut c = build_note_generator(0, METHOD_DOWN, 4);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(-12));
        assert_eq!(c.next(), Some(-24));
        assert_eq!(c.next(), Some(-36));
    }

    #[test]
    fn test_method_up_down_odd() {
        let mut c = build_note_generator(0, METHOD_UPDOWN, 3);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(12));
        assert_eq!(c.next(), Some(0));
    }

    #[test]
    fn test_method_up_down_even() {
        let mut c = build_note_generator(0, METHOD_UPDOWN, 4);
        assert_eq!(c.next(), Some(0));
        assert_eq!(c.next(), Some(12));
        assert_eq!(c.next(), Some(12));
        assert_eq!(c.next(), Some(0));
    }
    #[test]
    fn test_only_one_note() {
        let mut c = build_note_generator(0, METHOD_UP, 1);
        assert_eq!(c.next(), Some(0));

        let mut c = build_note_generator(0, METHOD_DOWN, 1);
        assert_eq!(c.next(), Some(0));

        let mut c = build_note_generator(0, METHOD_UPDOWN, 1);
        assert_eq!(c.next(), Some(0));
    }
}

#[cfg(test)]
mod test_count_to_vec {
    use crate::arp_handler::{count_to_vec, count_to_vec_up_down};

    #[test]
    fn test_count_to_vec() {
        let new_vec = count_to_vec(3, |i| i * 2);
        assert_eq!(new_vec, vec![0, 2, 4]);
    }

    #[test]
    fn test_count_to_vec_up_down_with_odd_counts() {
        let new_vec = count_to_vec_up_down(3, |i| i * 2);
        assert_eq!(new_vec, vec![0, 2, 0]);
    }

    #[test]
    fn test_count_to_vec_with_even_counts() {
        let new_vec = count_to_vec_up_down(4, |i| i * 2);
        assert_eq!(new_vec, vec![0, 2, 2, 0]);
    }
}

const METHOD_NO_METHOD: i8 = 0;
const METHOD_UP: i8 = 1;
const METHOD_DOWN: i8 = 2;
const METHOD_UPDOWN: i8 = 3;
const METHOD_DOWNUP: i8 = 4;
const METHOD_3CHORD: i8 = 5;
const METHOD_7CHORD: i8 = 6;
const METHOD_3MINCHORD: i8 = 7;
const METHOD_7MINCHORD: i8 = 8;

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
        velocity_automation_span: {
            let mut va_span: Vec<i8> = vec![0i8; 20];
            va_span[RATE_1_1 as usize] = 1;
            va_span[RATE_1_1_T as usize] = 1;
            va_span[RATE_1_2 as usize] = 2;
            va_span[RATE_1_2_D as usize] = 1;
            va_span[RATE_1_2_T as usize] = 3;
            va_span[RATE_1_4 as usize] = 4;
            va_span[RATE_1_4_D as usize] = 2;
            va_span[RATE_1_4_T as usize] = 6;
            va_span[RATE_1_8 as usize] = 8;
            va_span[RATE_1_8_D as usize] = 5;
            va_span[RATE_1_8_T as usize] = 12;
            va_span[RATE_1_16 as usize] = 16;
            va_span[RATE_1_16_D as usize] = 10;
            va_span[RATE_1_16_T as usize] = 24;
            va_span[RATE_1_32 as usize] = 32;
            va_span[RATE_1_32_D as usize] = 21;
            va_span[RATE_1_32_T as usize] = 48;
            va_span[RATE_1_64 as usize] = 64;
            va_span[RATE_1_64_D as usize] = 42;
            va_span[RATE_1_64_T as usize] = 96;
            va_span
        },
        rate_scales: {
            let mut rate_scale: Vec<f64> = vec![0.0f64; 20];
            rate_scale[RATE_1_1 as usize] = 4f64;
            rate_scale[RATE_1_1_T as usize] = rate_scale[RATE_1_1 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_2 as usize] = 2f64;
            rate_scale[RATE_1_2_D as usize] = rate_scale[RATE_1_2 as usize] * 1.5f64;
            rate_scale[RATE_1_2_T as usize] = rate_scale[RATE_1_2 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_4 as usize] = 1f64;
            rate_scale[RATE_1_4_D as usize] = rate_scale[RATE_1_4 as usize] * 1.5f64;
            rate_scale[RATE_1_4_T as usize] = rate_scale[RATE_1_4 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_8 as usize] = 0.5f64;
            rate_scale[RATE_1_8_D as usize] = rate_scale[RATE_1_8 as usize] * 1.5f64;
            rate_scale[RATE_1_8_T as usize] = rate_scale[RATE_1_8 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_16 as usize] = 0.25f64;
            rate_scale[RATE_1_16_D as usize] = rate_scale[RATE_1_16 as usize] * 1.5f64;
            rate_scale[RATE_1_16_T as usize] = rate_scale[RATE_1_16 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_32 as usize] = 0.125f64;
            rate_scale[RATE_1_32_D as usize] = rate_scale[RATE_1_32 as usize] * 1.5f64;
            rate_scale[RATE_1_32_T as usize] = rate_scale[RATE_1_32 as usize] * 2f64 / 3f64;
            rate_scale[RATE_1_64 as usize] = 0.0625f64;
            rate_scale[RATE_1_64_D as usize] = rate_scale[RATE_1_64 as usize] * 1.5f64;
            rate_scale[RATE_1_64_T as usize] = rate_scale[RATE_1_64 as usize] * 2f64 / 3f64;
            rate_scale
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
