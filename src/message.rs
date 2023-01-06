use std::borrow::BorrowMut;
use std::str::Bytes;
use std::sync::{Arc, Mutex};

use bytes::{Buf, BufMut, BytesMut};
use crate::arp_handler::GLOBAL_ARP_HANDLER;
use crate::constants::*;
use crate::midi_connect::{GLOBAL_MIDI_CONNECTOR, MidiConnector};
use crate::message::Message::*;
use crate::pitch_wheel;
use crate::server::VPadMessageContext;


#[derive(Debug)]
pub enum Message {
    HandShake {
        name: String,
        platform: String,
    },
    Midi {
        note: i8,
        velocity: i8,
        state: i8
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
        bpm: i16
    },
    PitchWheel {
        pos: i8,
        prev_pos: i8
    },
    CC {
        channel: i8,
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
            Midi {note, velocity, state} => {
                let mut midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
                midi_connector.borrow_mut().midi_note_message(note, velocity, state);
                None
            },
            Arp { note, .. } => {
                let identifier = format!("{}:{} on {}", ctx.addr.ip().to_string(), ctx.addr.port().to_string(), &note);
                GLOBAL_ARP_HANDLER.handle(identifier, self);
                None
            }
            PitchWheel { pos, prev_pos } => {
                pitch_wheel::move_to_smoothly(prev_pos, pos);
                None
            }
            CC { channel, value } => {
                let mut midi_connector = GLOBAL_MIDI_CONNECTOR.lock().unwrap();
                midi_connector.borrow_mut().cc_message(channel, value);
                None
            }
            _ => {
                None
            }
        }
    }
}

