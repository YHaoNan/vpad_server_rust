use dioxus::prelude::*;
use crate::public::{fetch_midi_ports};

pub fn launch() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    if let Ok(ports) = fetch_midi_ports() {
        cx.render(rsx!(
            div {
                h5 { "乐器MIDI接口" },
                midi_device_selector { midi_ports: &ports },
                h5 { "控制MIDI接口" },
                midi_device_selector { midi_ports: &ports },
            }
        ))
    } else {
        cx.render(rsx!(
            span { "初始化MIDI设备失败" }
        ))
    }
}

#[derive(PartialEq, Props)]
struct MidiDeviceSelectorProps<'a> {
    midi_ports: &'a Vec<String>
}

fn midi_device_selector<'a>(cx: Scope<'a, MidiDeviceSelectorProps<'a>>) -> Element {
    cx.render(rsx!(
        select {
            for port in cx.props.midi_ports {
                option { "{port}" }
            }
        },
    ))
}
