mod data;
mod model;
mod presentation;
mod style;
mod view;

pub use view::SearchButtonView;

use crate::components::app::components::shell::components::header::components::toolbar::components::shared::toolbar_button_surface::ToolbarButtonSurface;
use dioxus::prelude::*;
use model::SearchButtonModel;
use presentation::SearchButtonPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SearchButton(props: SearchButtonModel) -> Element {
    let SearchButtonPresentation {
        icon,
        aria_label,
        aria_haspopup,
        aria_expanded,
        onclick,
    } = SearchButtonPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            ToolbarButtonSurface {
                icon,
                aria_label,
                aria_haspopup,
                aria_expanded,
                onclick,
            }
        }
    }
}

assert_component!(SearchButton);
