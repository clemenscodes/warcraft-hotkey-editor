mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;

use crate::components::grid_editors::grid_editor::components::grid_editor_tile::components::tile_face::components::hotkey_badge::{
    HotkeyBadge, HotkeyBadgeProps,
};

pub use props::FollowerBadgeProps;
use style::CLASS;
assert_component!(FollowerBadge);

#[component]
pub fn FollowerBadge(props: FollowerBadgeProps) -> Element {
    let badge = HotkeyBadgeProps::from(&props);
    rsx! {
        div { class: CLASS,
            HotkeyBadge { ..badge }
        }
    }
}
