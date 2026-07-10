mod props;
mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::components::shared::hotkey_badge::{
    HotkeyBadge, HotkeyBadgeProps,
};

pub use props::FollowerBadgeProps;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FollowerBadge(props: FollowerBadgeProps) -> Element {
    let badge = HotkeyBadgeProps::from(&props);
    rsx! {
        div { class: CLASS,
            HotkeyBadge { ..badge }
        }
    }
}

assert_component!(FollowerBadge);
