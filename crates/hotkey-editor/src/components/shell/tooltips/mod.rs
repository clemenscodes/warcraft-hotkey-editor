use dioxus::prelude::*;

const TOOLTIP_TOUCH_SCRIPT: Asset = asset!("/assets/tooltip-touch.js");

/// Mounts the touch/long-press tooltip behavior. The tooltip styling itself is a
/// global concern and lives in the design layer (`tailwind.input.css`), so this only
/// injects the supporting script.
#[component]
pub fn TooltipMount() -> Element {
    rsx! {
        document::Script { src: TOOLTIP_TOUCH_SCRIPT, r#type: "module" }
    }
}
