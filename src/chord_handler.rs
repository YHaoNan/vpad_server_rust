use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;
use lazy_static::lazy_static;
use crate::message::Message;
use crate::message::Message::Chord;
use crate::midi_connect::GLOBAL_MIDI_CONNECTOR;
use crate::pulse_generator::PulseGenerator;

pub struct ChordHandler {
    chord_tasks: Mutex<HashMap<String, tokio::sync::oneshot::Sender<()>>>,
}

impl ChordHandler {
    pub fn handle(&self, identifier: String, message: Message) {
        // if let Chord { note, velocity, state, chord_type, chord_level, transpose, arp_delay } = message {
        if let Chord { state,  .. } = message {
            if state == 1 {
                // 开启和弦任务
                self.start_chord_task(identifier, message);
            } else {
                // 1. 关闭和弦任务，避免还没被按下的和弦按键被按下
                // 2. 给和弦中所有要按下的按键发送midi off
                self.stop_chord_task(identifier, message);
            }
        }
    }

    fn start_chord_task(&self, identifier: String, message: Message) {
        if let Chord { note, velocity , bpm, chord_type, chord_level, transpose, arp_delay, channel, ..} = message {

            let mut note_offs = build_note_offsets(chord_type, chord_level);
            transpose_vec(&mut note_offs, transpose);
            println!("transported note offs : {:?}", &note_offs);

            let _beat_dur = 60f64 / bpm as f64;
            let _arp_finished_dur = _beat_dur * (arp_delay as f64 / 100f64);
            let _note_interval = _arp_finished_dur / note_offs.len() as f64;
            println!("_beat_dur {} , _arp_finished_dur {} , _note interval {}", _beat_dur, _arp_finished_dur, _note_interval);

            let pulse_generator = PulseGenerator::new(vec![Duration::from_secs_f64(_note_interval)]);

            let (stop_sender, mut stop_receiver) = tokio::sync::oneshot::channel();
            tokio::task::spawn_blocking(move || {
                for i in pulse_generator {
                    if let Ok(()) = stop_receiver.try_recv() { break; }
                    let i = i as usize;
                    send_midi_note_msg_once(note + note_offs[i], velocity, 1, channel);
                    if i == note_offs.len() - 1 {
                        break;
                    }
                }
            });

            self.chord_tasks.lock().unwrap().insert(identifier, stop_sender);
        }
    }

    fn stop_chord_task(&self, identifier: String, message: Message) {
        if let Chord { note, chord_type, chord_level, transpose, channel, ..} = message {
            if let Some(tx) = self.chord_tasks.lock().unwrap().remove(&identifier) {
                let _  = tx.send(());
            }
            let mut note_offs = build_note_offsets(chord_type, chord_level);
            transpose_vec(&mut note_offs, transpose);
            send_midi_off(note, note_offs, channel);
        }
    }
}

fn send_midi_note_msg_once(note: i8, velocity: i8, state: i8, channel: i8) {
    if note < 0 { return; }
    let mut conn = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
    conn.midi_note_message_with_channel_number(note, velocity, state, channel);
}
fn send_midi_on(note: i8, note_offs: Vec<i8>, velocity: i8, channel: i8) {
    for note_off in note_offs {
        send_midi_note_msg_once(note + note_off, velocity, 1, channel);
    }
}
fn send_midi_off(note: i8, note_offs: Vec<i8>, channel: i8) {
    for note_off in note_offs {
        send_midi_note_msg_once(note + note_off, 0, 0, channel);
    }
}

fn transpose_vec(note_offs: &mut Vec<i8>, transpose: i8) {
    for _ in 0..transpose {
        transpose_vec_once(note_offs);
    }
}
fn transpose_vec_once(note_offs: &mut Vec<i8>) {
    if note_offs.len() == 0 { return; }
    // 进行一次转置，把最上面的降低12，到最下面
    let lastidx = note_offs.len() - 1;
    let last = note_offs[lastidx];
    for i in (0..lastidx).rev() {
        note_offs[i + 1] = note_offs[i];
    }
    note_offs[0] = last - 12;
}


const CHORD_LEVEL_OFFS: [i8; 7] = [0, 4, 7, 11, 14, 17, 21];

fn build_note_offsets(chord_type: i8, chord_level: i8) -> Vec<i8> {
    let mut n = CHORD_LEVEL_OFFS.clone();
    match chord_type {
        CHORD_TYPE_MAJOR => {},
        CHORD_TYPE_MINOR => { n[1] -= 1; n[3] -= 1; },
        CHORD_TYPE_DOM   => { n[3] -= 1; },
        CHORD_TYPE_AUG   => { n[2] += 1; },
        CHORD_TYPE_DIM   => { n[1] -= 1; n[2] -= 1; },
        CHORD_TYPE_SUS2  => { n[1] -= 2; },
        CHORD_TYPE_SUS4  => { n[1] += 1; },
        _ => {},
    };

    let mut ret: Vec<i8> = Vec::new();
    for i in 0..(3+chord_level) as usize {
        ret.push(n[i]);
    }
    ret
}

lazy_static! {
    pub static ref GLOBAL_CHORD_HANDLER: ChordHandler = ChordHandler {
        chord_tasks: Mutex::new(HashMap::new())
    };
}

const CHORD_TYPE_MAJOR: i8 = 0;   // 大和弦
const CHORD_TYPE_MINOR: i8 = 1;   // 小和弦
const CHORD_TYPE_DOM: i8   = 2;   // 属和弦
const CHORD_TYPE_AUG: i8   = 3;   // 增和弦
const CHORD_TYPE_DIM: i8   = 4;   // 减和弦
const CHORD_TYPE_SUS2: i8  = 5;   // 挂2和弦
const CHORD_TYPE_SUS4: i8  = 6;   // 挂4和弦
const CHORD_TYPE_ADD6: i8  = 7;   // 加6和弦 在原始和弦上加高6音
const CHORD_TYPE_ADD9: i8  = 8;   // 加9和弦 在原始和弦上加高9音


const CHORD_LEVEL_3: i8    = 0;   // 3和弦
const CHORD_LEVEL_7: i8    = 1;   // 7和弦
const CHORD_LEVEL_9: i8    = 2;   // 9和弦
const CHORD_LEVEL_11: i8   = 3;   // 11和弦
const CHORD_LEVEL_13: i8   = 4;   // 13和弦