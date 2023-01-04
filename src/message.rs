use std::borrow::BorrowMut;
use std::process::id;
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
    }
}


impl Message {
    pub fn parse(byte_buf: &mut BytesMut) -> Option<Message> {
        let content_bytes = byte_buf.get_i16();
        let op = byte_buf.get_i8();
        println!("op => {op}, content_bytes => {content_bytes}");
        match op {
            HANDSHAKE_OP => {
                Some(HandShake {
                    name: byte_buf.get_string(),
                    platform: byte_buf.get_string()
                })
            },
            MIDI_OP => {
                Some(Midi {
                    note: byte_buf.get_i8(),
                    velocity: byte_buf.get_i8(),
                    state: byte_buf.get_i8()
                })
            }
            ARP_OP => {
                Some(Arp {
                    note: byte_buf.get_i8(),
                    velocity: byte_buf.get_i8(),
                    state: byte_buf.get_i8(),
                    method: byte_buf.get_i8(),
                    rate: byte_buf.get_i8(),
                    swing_pct: byte_buf.get_i8(),
                    up_note_cnt: byte_buf.get_i8(),
                    velocity_automation: byte_buf.get_i8(),
                    dynamic_pct: byte_buf.get_i16(),
                    bpm: byte_buf.get_i16()
                })
            }
            PITCHWHEEL_OP => {
                Some(PitchWheel {
                    pos: byte_buf.get_i8(),
                    prev_pos: byte_buf.get_i8()
                })
            }
            _ => {
                None
            }
        }
    }

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
            _ => {
                None
            }
        }
    }

    pub fn to_buf(&self) -> impl Buf {
        let mut buf = BytesMut::new();
        match self {
            HandShake {name, platform} => {
                let name = name.as_bytes();
                let platform = platform.as_bytes();

                buf.put_i16((name.len() + platform.len()) as i16);
                buf.put_i8(HANDSHAKE_OP);
                buf.put_string(name);
                buf.put_string(platform);
            }
            _ => { /* nothing to do */ }
        }
        buf
    }
}


trait StringLikeBuf {
    fn get_string(&mut self) -> String;
    fn put_string(&mut self, string: &[u8]);
}

// Provide BytesMut.set_string以及BytesMut.get_string
impl StringLikeBuf for BytesMut {
    fn get_string(&mut self) -> String {
        let len = self.get_i8();
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(self.get_u8());
        }
        String::from_utf8(vec).unwrap()
    }

    fn put_string(&mut self, string: &[u8]) {
        self.put_i8(string.len() as i8);
        for u8 in string {
            self.put_u8(*u8);
        }
    }
}