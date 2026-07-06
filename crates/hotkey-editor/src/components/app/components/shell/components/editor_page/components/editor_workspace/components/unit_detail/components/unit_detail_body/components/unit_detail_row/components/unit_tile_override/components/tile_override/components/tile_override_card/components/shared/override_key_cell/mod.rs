mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::OverrideKeyCellProps;

assert_component!(OverrideKeyCell);

/// The hotkey-capture button shown in the override panel header (and the alt/upgrade
/// sections).
#[component]
pub fn OverrideKeyCell(props: OverrideKeyCellProps) -> Element {
    let label = props.label;
    let is_editing = props.is_editing;
    let is_special = props.is_special;
    let title = props.title;
    let on_activate = props.on_activate;
    let handle_click = move |_| on_activate.call(());
    rsx! {
        button {
            class: CLASS,
            "data-editing": is_editing,
            "data-special": is_special,
            title,
            onclick: handle_click,
            {label}
        }
    }
}
