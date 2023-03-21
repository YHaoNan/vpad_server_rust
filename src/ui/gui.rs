use dioxus::prelude::*;
use dioxus_desktop::{Config, PhysicalSize, WindowBuilder};
use crate::public::fetch_midi_ports;
use crate::ui::error::ErrorPanel;
use crate::ui::main::MainUI;


pub fn launch() {
    dioxus_desktop::launch_cfg(App,
        Config::new().with_window(
            WindowBuilder::new().with_inner_size(
                PhysicalSize { width: 300, height: 500 }
            )
        )
    )
}


#[derive(Clone)]
pub struct AppData {
    pub midi_ports: Vec<String>
}

fn App(cx: Scope) -> Element {
    if let Ok(ports) = fetch_midi_ports() {
        use_shared_state_provider(cx, || AppData {midi_ports: ports});
        cx.render(rsx! {
            MainUI {}
        })
    } else {
        cx.render(rsx! {
            ErrorPanel { errmsg: "无法获取到MIDI设备信息".into() }
        })
    }
}
