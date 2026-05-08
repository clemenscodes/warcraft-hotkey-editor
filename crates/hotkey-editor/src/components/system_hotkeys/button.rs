use dioxus::prelude::*;

#[component]
pub(crate) fn SystemHotkeysButton(mut system_hotkeys_open: Signal<bool>) -> Element {
    let handle_click = move |_| {
        let next_value = !*system_hotkeys_open.read();
        system_hotkeys_open.set(next_value);
    };
    rsx! {
        button {
            class: "inline-flex items-center justify-center px-14 py-6 \
                rounded-lg border border-warcraft-blue text-warcraft-text-secondary \
                font-friz-quadrata text-[2rem] transition-all duration-[120ms] whitespace-nowrap \
                bg-[rgba(20,40,70,0.7)] [text-shadow:1px_1px_0_rgba(0,0,0,0.6)] \
                hover:border-warcraft-gold hover:text-warcraft-gold \
                hover:[box-shadow:0_0_12px_rgba(255,206,99,0.25)]",
            r#type: "button",
            onclick: handle_click,
            "System Hotkeys"
        }
    }
}
