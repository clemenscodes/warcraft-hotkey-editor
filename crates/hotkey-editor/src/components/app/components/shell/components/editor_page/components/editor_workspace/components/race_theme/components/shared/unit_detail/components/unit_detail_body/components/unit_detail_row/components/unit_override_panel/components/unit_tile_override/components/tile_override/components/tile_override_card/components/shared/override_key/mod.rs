pub mod components;
mod model;
mod view;

pub use view::OverrideKeyView;

use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycapState;
use components::normal_override_key::NormalOverrideKey;
use components::special_override_key::SpecialOverrideKey;
use dioxus::prelude::*;
use model::OverrideKeyModel;
use tw_macro::assert_component;

/// The hotkey-capture button shown in the override panel header (and the alt/upgrade
/// sections). A thin dispatcher: a multi-character special token (Esc, Mouse4) needs a
/// wider box than a single letter, so it renders `SpecialOverrideKey` xor
/// `NormalOverrideKey`. Each variant owns its own classed button root and calls
/// `on_activate` on click.
#[component]
pub fn OverrideKey(props: OverrideKeyModel) -> Element {
    let OverrideKeyModel {
        label,
        is_editing,
        is_special,
        title,
        on_activate,
    } = props;
    let state = if is_editing {
        EditableKeycapState::Editing
    } else {
        EditableKeycapState::Idle
    };
    if is_special {
        rsx! {
            SpecialOverrideKey { label, state, title, on_activate }
        }
    } else {
        rsx! {
            NormalOverrideKey { label, state, title, on_activate }
        }
    }
}

assert_component!(OverrideKey);
