use dioxus::html::s;
use midi_control::Channel;
use crate::message::Message;
use crate::midi_connect::GLOBAL_CTL_CONNECTOR;


fn send_on_and_off(note: i8, velocity: i8) {
    GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, velocity, 1);
    GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, velocity, 0);
}

fn map_num_to_channel(num: i8) -> Channel {
    match num {
        1 => Channel::Ch1,
        2 => Channel::Ch2,
        3 => Channel::Ch3,
        4 => Channel::Ch4,
        5 => Channel::Ch5,
        6 => Channel::Ch6,
        7 => Channel::Ch7,
        8 => Channel::Ch8,
        9 => Channel::Ch9,
        10 => Channel::Ch10,
        11 => Channel::Ch11,
        12 => Channel::Ch12,
        13 => Channel::Ch13,
        14 => Channel::Ch14,
        15 => Channel::Ch15,
        16 => Channel::Ch16,
        _ => Channel::Ch1
    }
}

pub fn handle_track_message(msg: Message) {
    // 第nth个轨道，设置状态为state，如果状态时FADER_VALUE_CHANEGD，设置value
    if let Message::TrackMessage { nth, state, value } = msg {
        match state {
            STATE_FADER_UP => send_on_and_off(TRACK_FADER_TOUCH_NOTE_OFFSET + nth - 1, 0),
            STATE_FADER_DOWN => send_on_and_off(TRACK_FADER_TOUCH_NOTE_OFFSET + nth - 1, 127),
            STATE_FADER_VALUE_CHANGED => GLOBAL_CTL_CONNECTOR.lock().unwrap().pitch_wheel_message_with_channel(value, map_num_to_channel(nth)),
            STATE_SOLO_ON | STATE_SOLO_OFF => send_on_and_off(TRACK_SOLO_NOTE_OFFSET + nth - 1, 127),
            STATE_MUTE_ON | STATE_MUTE_OFF => send_on_and_off(TRACK_MUTE_NOTE_OFFSET + nth - 1, 127),
            STATE_REC_ON | STATE_REC_OFF => send_on_and_off(TRACK_REC_NOTE_OFFSET + nth - 1, 127),
            _ => {
                log::error!("cannot handle track message since state is invaild {}", state);
            }
        }
    }
}


const STATE_FADER_UP: i8 = 0;
const STATE_FADER_DOWN: i8 = 1;
const STATE_FADER_VALUE_CHANGED: i8 = 2;
const STATE_SOLO_ON: i8 = 3;
const STATE_SOLO_OFF: i8 = 4;
const STATE_MUTE_ON: i8 = 5;
const STATE_MUTE_OFF: i8 = 6;
const STATE_REC_ON: i8 = 7;
const STATE_REC_OFF: i8 = 8;

const TRACK_FADER_TOUCH_NOTE_OFFSET: i8 = 104;
const TRACK_SOLO_NOTE_OFFSET: i8 = 8;
const TRACK_MUTE_NOTE_OFFSET: i8 = 16;
const TRACK_REC_NOTE_OFFSET: i8 = 0;