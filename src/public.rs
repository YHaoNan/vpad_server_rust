use crate::midi_connect::{GLOBAL_MIDI_CONNECTOR, Result};


pub fn fetch_midi_ports() -> Result<Vec<String>> {
    GLOBAL_MIDI_CONNECTOR.lock().unwrap().port_list()
}
