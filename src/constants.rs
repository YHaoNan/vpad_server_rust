use std::string::ToString;
use crate::message::Message;

pub const HANDSHAKE_OP: i8 = 1;
pub const MIDI_OP: i8 = 2;
pub const ARP_OP: i8 = 3;


pub const SERVER_NAME: &str = "VPadServer";
pub const SERVER_PLATFORM: &str = "Windows x86 Rust";