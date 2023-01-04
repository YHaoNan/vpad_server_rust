use std::borrow::BorrowMut;
use std::string::ToString;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use midi_control::MidiMessageSend;
use midir::{ConnectError, InitError, MidiOutput, MidiOutputConnection};
use crate::{midi_connect::MidiConnectorError::PortNotFoundError, message::Message};

type Result<T> = std::result::Result<T, MidiConnectorError>;

pub struct MidiConnector {
    name: String,
    connection: Option<MidiOutputConnection>
}

impl MidiConnector {
    pub fn new(name: String) -> MidiConnector {
        MidiConnector {
            name,
            connection: None
        }
    }

    fn open_output() -> Result<MidiOutput> {
        Ok(MidiOutput::new("client")?)
    }

    /// 获取并返回当前port列表
    pub fn port_list(&mut self) -> Result<Vec<String>> {
        let output = MidiConnector::open_output()?;
        let count = output.port_count();
        let mut port_name_list: Vec<String> = Vec::with_capacity(count);
        for port in output.ports() {
            if let Ok(port_name) = output.port_name(&port) {
                port_name_list.push(port_name);
            }
        }

        Ok(port_name_list)
    }

    /// 连接port列表中的第i个port
    pub fn connect_port(&mut self, port_name: String) -> Result<()> {
        let output = MidiConnector::open_output()?;
        for port in output.ports() {
            if let Ok(this_port_name) = output.port_name(&port) {
                if port_name == this_port_name {
                    self.connection = Some(output.connect(&port, &this_port_name)?);
                    return Ok(())
                }
            }
        }

        Err(PortNotFoundError)
    }
    
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    pub fn midi_note_message(&mut self, note: i8, velocity: i8, state: i8) {
        let message = if state == 0 {
            midi_control::note_off(midi_control::Channel::Ch1, note as u8, velocity as u8)
        } else {
            midi_control::note_on(midi_control::Channel::Ch1, note as u8, velocity as u8)
        };
        self.connection.as_mut().unwrap().send_message(message);
    }

    pub fn pitch_wheel_message(&mut self, pos: i8) {
        let pos = pos as u16;
        let pos = pos * 128;
        self.connection.as_mut().unwrap().send_message(
            midi_control::pitch_bend(midi_control::Channel::Ch1, pos)
        );
    }

}

lazy_static! {
    pub static ref GLOBAL_MIDI_CONNECTOR: Mutex<MidiConnector> = Mutex::new(
        MidiConnector::new("GLOBAL_MIDI_CONNECTOR#1".to_string())
    );
}

#[derive(Debug)]
pub enum MidiConnectorError {
    InitError(InitError),
    ConnectError(ConnectError<MidiOutput>),
    PortNotFoundError
}
impl From<InitError> for MidiConnectorError {
    fn from(value: InitError) -> Self {
        MidiConnectorError::InitError(value)
    }
}

impl From<ConnectError<MidiOutput>> for MidiConnectorError {
    fn from(value: ConnectError<MidiOutput>) -> Self {
        MidiConnectorError::ConnectError(value)
    }
}

// ============== TEST ===============
#[cfg(test)]
mod midi_connect_test {
    use crate::midi_connect::MidiConnector;

    #[test]
    fn test_new() {
        let conn = MidiConnector::new("TestConnector".to_string());
        assert_eq!(conn.name, "TestConnector");
    }

    #[test]
    fn test_port_list() {
        let mut conn = MidiConnector::new("TestConnector".to_string());
        let ports = conn.port_list().unwrap();
        for port in ports {
            println!("{port}");
        }
    }

    #[test]
    fn test_connect_port() {
        let mut conn = MidiConnector::new("TestConnector".to_string());
        if let Ok(_) = conn.connect_port("loopMIDI Port".to_string()) {
            assert!(conn.is_connected());
        }
    }
}