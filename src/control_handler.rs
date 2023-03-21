use std::collections::HashMap;
use std::iter::Map;
use dioxus::html::u;
use lazy_static::lazy_static;
use crate::message::Message::ControlMessage;
use crate::message::Message;
use crate::midi_connect::GLOBAL_CTL_CONNECTOR;
use crate::midi_note_to_number::*;

pub fn handle_control_msg(daw: DawType, message: Message) {
    if let ControlMessage {operation, state, auto_close} = message {
        if operation > 14 {
            log::error!("cannot execute control message, because operation code is out of bounds");
        } else {
            let note = get_note_by_type(&daw, operation);
            if state == OP_STATE_ON {
                GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, 127, 1);
                GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, 127, 0);
            }
            if state == OP_STATE_OFF || auto_close == 1 {
                GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, 0, 1);
                GLOBAL_CTL_CONNECTOR.lock().unwrap().midi_note_message(note, 0, 0);
            }
        }
    }
}

fn get_note_by_type(daw: &DawType, operation: i8) -> i8{
    let vec = MP_DAW_TO_CTL_NOTE.get(daw).unwrap();
    vec[operation as usize]
}

#[derive(Eq, Hash, PartialEq)]
pub enum DawType {
    McuDefault,
    FLStudio, StudioOne, Protools, Reaper, AbletonLive, Cubase, AdobeAudition, CakeWalk, Logic
}

const OP_PLAY: i8 = 0i8;                     // 播放
const OP_STOP: i8 = 1i8;                     // 停止
const OP_RECORD: i8 = 2i8;                   // 录制
const OP_UNDO: i8 = 3i8;                     // UNDO
const OP_REDO: i8 = 4i8;                     // REDO
const OP_LOOP: i8 = 5i8;                     // LOOP
const OP_SAVE: i8 = 6i8;                     // SAVE
const OP_ZOOM: i8 = 7i8;                     // ZOOM
const OP_CURSOR_L: i8 = 8i8;                 // CURSOR_L
const OP_CURSOR_R: i8 = 9i8;                 // CURSOR_R
const OP_CURSOR_U: i8 = 10i8;
const OP_CURSOR_D: i8 = 11i8;
const OP_CLICK: i8 = 12i8;                   // Toggle Tempo
const OP_TRACK_BANK_LEFT: i8 = 13i8;         // Left trak
const OP_TRACK_BANK_RIGHT: i8 = 14i8;

const OP_STATE_ON: i8 = 1;
const OP_STATE_OFF: i8 = 0;

fn init_map() -> HashMap<DawType, Vec<i8>> {
    let mut map: HashMap<DawType, Vec<i8>> = HashMap::new();
    map.insert(DawType::McuDefault,     vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::FLStudio,       vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::StudioOne,      vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::Protools,       vec![A_SHARP_6, A_6, B_6, A_5, G_5, D_6, G_SHARP_5, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::Reaper,         vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::Cubase,         vec![A_SHARP_6, A_6, B_6, A_SHARP_4, B_4, -1, C_5, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::AdobeAudition,  vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::CakeWalk,       vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::Logic,          vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map.insert(DawType::AbletonLive,    vec![A_SHARP_6, A_6, B_6, E_5, G_5, D_6, -1, D_SHARP_7, C_SHARP_7, E_7, C_7, F_7, -1, A_SHARP_2, B_2]);
    map
}

lazy_static! {
    // 从DawType到控制note的映射，若通过查询得到的note小于0，证明在该Daw中不支持此功能，目前知道的不支持列表
    //  AbletonLive   ==   Save、Click
    //  Cubase        ==   Loop、Click
    static ref MP_DAW_TO_CTL_NOTE: HashMap<DawType, Vec<i8>> = {
        init_map()
    };
}

