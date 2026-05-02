use dioxus::prelude::*;

#[component]
pub(crate) fn SystemHotkeysButton(mut system_hotkeys_open: Signal<bool>) -> Element {
    rsx! {
        button {
            class: "btn-warcraft-secondary",
            r#type: "button",
            onclick: move |_| {
                let next_value = !*system_hotkeys_open.read();
                system_hotkeys_open.set(next_value);
            },
            "System Hotkeys"
        }
    }
}
