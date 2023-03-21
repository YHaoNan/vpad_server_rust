use crate::midi_connect::{MidiConnector, Result};


pub fn fetch_midi_ports() -> Result<Vec<String>> {
    MidiConnector::port_list()
}
