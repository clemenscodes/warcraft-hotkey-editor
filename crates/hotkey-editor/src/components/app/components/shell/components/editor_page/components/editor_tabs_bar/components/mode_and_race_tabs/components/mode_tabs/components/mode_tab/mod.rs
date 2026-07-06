mod props;
mod style;

use dioxus::prelude::*;
pub use props::ModeTabProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ModeTab);

/// One mode button (Melee or Campaign). Its active look is driven by the
/// `data-active` attribute; the label and handlers arrive as props.
#[component]
pub fn ModeTab(props: ModeTabProps) -> Element {
    let label = props.label;
    let active = props.active;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        button {
            class: CLASS,
            "data-active": active,
            onclick,
            onkeydown,
            {label}
        }
    }
}
