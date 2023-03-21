use dioxus::prelude::*;
use crate::ui::gui::AppData;

#[allow(unused_variables)]
pub fn MainUI(cx: Scope) -> Element {
    let server_started = use_state(cx, || false);
    let choosen_inst_port = use_state(cx, || "VPadInstrument");
    let choosen_cont_port = use_state(cx, || "VPadDawControl");
    let choosen_daw = use_state(cx, || "live");

    cx.render(rsx! {
        h5 { "乐器MIDI设备" }
        MidiDeviceSelector {}
        h5 { "控制MIDI设备" }
        MidiDeviceSelector {}
        h5 { "目标DAW" }
        select { value: "{choosen_daw}",
            option { value: "default", "Default" }
            option { value: "live", "Ableton Live" }
            option { value: "flstudio", "FL Studio" }
            option { value: "studioone", "Studio One" }
            option { value: "reaper", "Reaper" }
        }
        p {
            button {
                onclick: move |_| server_started.set(!server_started.get()),
                if **server_started {
                    "关闭服务器"
                } else {
                    "开启服务器"
                }
            }
        }
    })
}

fn MidiDeviceSelector(cx: Scope) -> Element {
    let state = use_shared_state::<AppData>(cx).unwrap();
    let ports = &state.read().midi_ports;
    cx.render(rsx! {
        select {
            for portname in ports.iter() {
                option { key: "{portname}", value: "{portname}", "{portname}" }
            }
        }
    })
}