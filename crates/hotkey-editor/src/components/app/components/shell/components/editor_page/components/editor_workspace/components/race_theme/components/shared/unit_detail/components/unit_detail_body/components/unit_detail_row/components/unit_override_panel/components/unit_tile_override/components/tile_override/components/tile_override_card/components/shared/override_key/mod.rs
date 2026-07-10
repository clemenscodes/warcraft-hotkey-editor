pub mod components;
mod logic;
mod props;

use components::normal_override_key::{NormalOverrideKey, NormalOverrideKeyProps};
use components::special_override_key::{SpecialOverrideKey, SpecialOverrideKeyProps};
use dioxus::prelude::*;
pub use props::OverrideKeyProps;
use tw_macro::assert_component;

/// The hotkey-capture button shown in the override panel header (and the alt/upgrade
/// sections). A thin dispatcher: a multi-character special token (Esc, Mouse4) needs a
/// wider box than a single letter, so it renders `SpecialOverrideKey` xor
/// `NormalOverrideKey`. Each variant owns its own classed button root and calls
/// `on_activate` on click.
#[component]
pub fn OverrideKey(props: OverrideKeyProps) -> Element {
    let is_special = props.is_special;
    if is_special {
        let special = SpecialOverrideKeyProps::from(&props);
        rsx! {
            SpecialOverrideKey { ..special }
        }
    } else {
        let normal = NormalOverrideKeyProps::from(&props);
        rsx! {
            NormalOverrideKey { ..normal }
        }
    }
}

assert_component!(OverrideKey);
