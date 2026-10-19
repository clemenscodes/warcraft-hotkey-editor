pub mod components;
mod model;
mod view;

pub use view::CollisionListScrollView;
mod style;

use crate::components::app::components::shell::components::shared::drag_scroll::{
    DragScrollBindings, use_drag_scroll,
};
use components::collision_list_track::CollisionListTrack;
use dioxus::prelude::*;
use model::CollisionListScrollModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CollisionListScroll(props: CollisionListScrollModel) -> Element {
    let cards = props.cards;
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
            CollisionListTrack {
                cards,
            }
        }
    }
}

assert_component!(CollisionListScroll);
