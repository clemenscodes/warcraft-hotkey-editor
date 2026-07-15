pub mod components;
mod model;
mod presentation;
mod view;

pub use view::CategoryFilterView;

use components::hidden_category_chip::HiddenCategoryChip;
use components::shown_category_chip::ShownCategoryChip;
use dioxus::prelude::*;
use model::CategoryFilterModel;
use presentation::{CategoryFilterDispatch, use_category_filter};
use tw_macro::assert_component;

#[component]
pub fn CategoryFilter(props: CategoryFilterModel) -> Element {
    let CategoryFilterDispatch { is_active, model } = use_category_filter(props.kind);
    let label = model.label();
    let onclick = model.onclick();
    match is_active {
        true => rsx! {
            ShownCategoryChip {
                label,
                onclick,
            }
        },
        false => rsx! {
            HiddenCategoryChip {
                label,
                onclick,
            }
        },
    }
}

assert_component!(CategoryFilter);
