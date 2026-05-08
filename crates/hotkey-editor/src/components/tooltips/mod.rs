use dioxus::prelude::*;

const TOOLTIP_STYLES: Asset = asset!("/src/components/tooltips/tooltips.css");
const TOOLTIP_TOUCH_SCRIPT: Asset = asset!("/assets/tooltip-touch.js");

#[component]
pub(crate) fn TooltipMount() -> Element {
    rsx! {
        document::Stylesheet { href: TOOLTIP_STYLES }
        document::Script { src: TOOLTIP_TOUCH_SCRIPT, r#type: "module" }
    }
}
