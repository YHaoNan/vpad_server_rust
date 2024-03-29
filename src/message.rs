use std::borrow::BorrowMut;
use crate::arp_handler::GLOBAL_ARP_HANDLER;
use crate::chord_handler::GLOBAL_CHORD_HANDLER;
use crate::constants::*;
use crate::control_handler::{DawType, handle_control_msg};
use crate::midi_connect::{GLOBAL_MIDI_CONNECTOR};
use crate::message::Message::*;
use crate::pitch_wheel;
use crate::server::VPadMessageContext;
use crate::track_handler::handle_track_message;


#[derive(Debug)]
pub enum Message {
    HandShake {
        name: String,
        platform: String,
    },
    Midi {
        note: i8,
        velocity: i8,
        state: i8,
        channel: i8
    },
    Arp {
        note: i8,
        velocity: i8,
        state: i8,
        method: i8,
        rate: i8,
        swing_pct: i8,
        up_note_cnt: i8,
        velocity_automation: i8,
        dynamic_pct: i16,
        bpm: i16,
        channel: i8
    },
    Chord {
        note: i8,
        velocity: i8,
        state: i8,
        bpm: i16,
        chord_type: i8,
        chord_level: i8,
        transpose: i8,
        arp_delay: i8,
        channel: i8
    },
    PitchWheel {
        pos: i8,
        prev_pos: i8,
        channel: i8
    },
    CC {
        // 特别注意，channel实际上是CC通道，比如该消息代表控制CC64，那么channel就是64
        // channel2实际上是该消息所在的MIDI通道
        channel: i8,
        value: i8,
        channel2: i8
    },
    ControlMessage {
        operation: i8,
        state: i8,
        auto_close: i8
    },
    TrackMessage {
        nth: i8,
        state: i8,
        value: i8
    }
}


impl Message {
    pub fn handle_and_return<'a>(self, ctx: &'a VPadMessageContext) -> Option<Message> {
        match self {
            HandShake { .. } => {
                Some(HandShake {
                    name: SERVER_NAME.into(),
                    platform: SERVER_PLATFORM.into()
                })
            },
            Midi {note, velocity, state, channel} => {
                let mut midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
                midi_connector.borrow_mut().midi_note_message_with_channel_number(note, velocity, state, channel);
                None
            },
            Arp { note, .. } => {
                let identifier = format!("{}:{} on {}", ctx.addr.ip().to_string(), ctx.addr.port().to_string(), &note);
                GLOBAL_ARP_HANDLER.handle(identifier, self);
                None
            },
            Chord { note, .. } => {
                let identifier = format!("{}:{} on {}", ctx.addr.ip().to_string(), ctx.addr.port().to_string(), &note);
                GLOBAL_CHORD_HANDLER.handle(identifier, self);
                None
            },
            PitchWheel { pos, prev_pos, channel} => {
                pitch_wheel::move_to_smoothly(prev_pos, pos, channel);
                None
            },
            CC { channel, value, channel2 } => {
                let mut midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
                midi_connector.borrow_mut().cc_message_with_channel_number(channel, value, channel2);
                None
            },
            ControlMessage { .. } => {
                handle_control_msg(DawType::McuDefault, self);
                None
            },
            TrackMessage { .. } => {
                handle_track_message(self);
                None
            }
        }
    }
}

