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

#[component]
pub fn CategoryScroll(props: CategoryScrollModel) -> Element {
    let sections = props.sections;
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
                sections,
            }
        }
    }
}

assert_component!(CategoryScroll);
