pub mod components;
mod model;
mod view;

pub use view::CategoryScrollView;
mod style;

use crate::components::app::components::shell::components::shared::drag_scroll::{
    DragScrollBindings, use_drag_scroll,
};
use components::category_track::CategoryTrack;
use dioxus::prelude::*;
use model::CategoryScrollModel;
use style::CLASS;
use tw_macro::assert_component;
use warcraft_api::UnitKind;

/// The stable list key for a category. Two lists render categories — the aside
/// and the mobile search dialog — so the key derivation lives with the list.
pub fn unit_kind_key(kind: UnitKind) -> &'static str {
    match kind {
        UnitKind::Hero => "hero",
        UnitKind::Soldier => "soldier",
        UnitKind::Worker => "worker",
        UnitKind::Building => "building",
    }
}

#[component]
pub fn CategoryScroll(props: CategoryScrollModel) -> Element {
    let groups = props.groups;
    let DragScrollBindings {
        onmounted,
        onpointerdown,
        onpointermove,
        onpointerup,
        onpointercancel,
        onlostpointercapture,
    } = use_drag_scroll();
    rsx! {
        div {
            class: CLASS,
            onmounted,
            onpointerdown,
            onpointermove,
            onpointerup,
            onpointercancel,
            onlostpointercapture,
            CategoryTrack {
                groups,
            }
        }
    }
}

assert_component!(CategoryScroll);
