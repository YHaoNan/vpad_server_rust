use bytes::{Buf, BufMut, BytesMut};
use crate::message::Message;
use crate::message::Message::*;
use tokio_util::codec;
use crate::constants::*;
use crate::message_codec::MessageCodecError::{DecodeError, IOError};

pub struct MessageCodec;
impl MessageCodec {
    const MAX_SIZE: usize = 65535;
}

#[derive(Debug)]
pub enum MessageCodecError {
    IOError(std::io::Error),
    DecodeError(&'static str),
    EncodeError(&'static str)
}

impl From<std::io::Error> for  MessageCodecError {
    fn from(value: std::io::Error) -> Self {
        IOError(value)
    }
}
impl codec::Encoder<Message> for MessageCodec {
    type Error = MessageCodecError;

    fn encode(&mut self, message: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match message {
            HandShake {name, platform} => {
                let name = name.as_bytes();
                let platform = platform.as_bytes();

                dst.put_i16((name.len() + platform.len()) as i16);
                dst.put_i8(HANDSHAKE_OP);
                dst.put_string(name);
                dst.put_string(platform);
            }
            _ => {

            }
        };
        Ok(())
    }
}

impl codec::Decoder for MessageCodec {
    type Item = Message;
    type Error = MessageCodecError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() == 0 {
            return Ok(None);
        }
        if src.len() < 2 {
            return Err(DecodeError("Incompleted Message"));
        }

        let first_two = &src[0..2];
        let remaind_bytes_cnt: i16 = first_two[0] as i16;
        let remaind_bytes_cnt: i16 = remaind_bytes_cnt << 8 | first_two[1] as i16;

        // 如果消息不完整
        if remaind_bytes_cnt as usize > src.len() - 2 {
            return Err(DecodeError("Incompleted Message"));
        }
        // 跳过前两个字节
        src.advance(2);

        // 分割当前消息和下一条消息
        let mut remaind_bytes = src.split_to(remaind_bytes_cnt as usize);
        log::info!("this_message_len:{}, remaind:{}", remaind_bytes.len(), src.len());

        // 获取操作码
        let op = remaind_bytes.get_i8();

        Ok(match op {
            HANDSHAKE_OP => {
                Some(HandShake {
                    name: remaind_bytes.get_string(),
                    platform: remaind_bytes.get_string()
                })
            }
            MIDI_OP => {
                Some(Midi {
                    note: remaind_bytes.get_i8(),
                    velocity: remaind_bytes.get_i8(),
                    state: remaind_bytes.get_i8()
                })
            }
            ARP_OP => {
                Some(Arp {
                    note: remaind_bytes.get_i8(),
                    velocity: remaind_bytes.get_i8(),
                    state: remaind_bytes.get_i8(),
                    method: remaind_bytes.get_i8(),
                    rate: remaind_bytes.get_i8(),
                    swing_pct: remaind_bytes.get_i8(),
                    up_note_cnt: remaind_bytes.get_i8(),
                    velocity_automation: remaind_bytes.get_i8(),
                    dynamic_pct: remaind_bytes.get_i16(),
                    bpm: remaind_bytes.get_i16()
                })
            }
            CHORD_OP => {
                Some(Chord {
                    note: remaind_bytes.get_i8(),
                    velocity: remaind_bytes.get_i8(),
                    state: remaind_bytes.get_i8(),
                    chord_type: remaind_bytes.get_i8(),
                    chord_level: remaind_bytes.get_i8(),
                    transpose: remaind_bytes.get_i8(),
                    arp_delay: remaind_bytes.get_i8(),
                    bpm: remaind_bytes.get_i16()
                })
            }
            PITCHWHEEL_OP => {
                Some(PitchWheel {
                    pos: remaind_bytes.get_i8(),
                    prev_pos: remaind_bytes.get_i8()
                })
            }
            CC_OP => {
                Some(CC {
                    channel: remaind_bytes.get_i8(),
                    value: remaind_bytes.get_i8()
                })
            }
            CONTROL_OP => {
                Some(ControlMessage {
                    operation: remaind_bytes.get_i8(),
                    state: remaind_bytes.get_i8(),
                    auto_close: remaind_bytes.get_i8()
                })
            }
            TRACK_OP => {
                Some(TrackMessage {
                    nth: remaind_bytes.get_i8(),
                    state: remaind_bytes.get_i8(),
                    value: remaind_bytes.get_i8()
                })
            }
            _ => {
                log::error!("Got an unsupportted message op {}", op);
                return Err(DecodeError("Unsupportted Message"))
            }
        })
    }
}

trait GetString {
    fn get_string(&mut self) -> String;
}
trait PutString {
    fn put_string(&mut self, string: &[u8]);
}

impl GetString for BytesMut {
    fn get_string(&mut self) -> String {
        let len = self.get_i8();
        let mut vec = Vec::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(self.get_u8());
        }
        String::from_utf8(vec).unwrap()
    }

}

impl PutString for BytesMut {
    fn put_string(&mut self, string: &[u8]) {
        self.put_i8(string.len() as i8);
        for u8 in string {
            self.put_u8(*u8);
        }
    }
}
