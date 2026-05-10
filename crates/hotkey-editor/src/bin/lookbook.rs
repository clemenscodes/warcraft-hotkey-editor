use dioxus::prelude::*;
use lookbook::LookBook;
use lookbook_macros::preview;

#[preview]
fn button_preview(
    #[lookbook(default = "Click me")] label: String,
) -> Element {
    rsx!(
        button {
            style: "padding: 0.5rem 1rem; background: #ffce63; color: #050a1a; border: none; border-radius: 4px; cursor: pointer; font-size: 1rem;",
            "{label}"
        }
    )
}

#[component]
fn app() -> Element {
    rsx!(LookBook {
        home: |()| rsx!(
            div {
                style: "padding: 2rem; color: #e0d8c8;",
                h1 { "Warcraft III Hotkey Editor — Component Previews" }
                p { "Select a component from the sidebar to preview it." }
            }
        ),
        previews: [button_preview]
    })
}

fn main() {
    dioxus::launch(app);
}
