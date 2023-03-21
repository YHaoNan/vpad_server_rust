use dioxus::prelude::*;


#[inline_props]
pub fn ErrorPanel(cx: Scope, errmsg: String) -> Element {
    cx.render(rsx! {
        div {
            h2 { "Oops! Something went wrong..." }
            p { "{errmsg}" }
        }
    })
}