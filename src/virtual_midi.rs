use midir::{MidiOutput, MidiOutputConnection};
use midir::os::unix::{VirtualOutput};

#[cfg(unix)]
pub fn create_virtual_midi_port(midi_output: MidiOutput, port_name: String) -> Option<MidiOutputConnection> {
    if let Ok(conn) = midi_output.create_virtual(&port_name) {
        return Some(conn);
    } else {
        return None;
    }
}

#[cfg(target_os = "windows")]
pub fn create_virtual_midi_port(_: MidiOutput, _: String) -> Option<MidiOutputConnection> {
    return None;
}