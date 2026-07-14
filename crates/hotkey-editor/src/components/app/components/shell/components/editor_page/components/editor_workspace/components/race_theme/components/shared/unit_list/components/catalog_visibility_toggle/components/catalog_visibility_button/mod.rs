mod model;
mod view;

pub use view::CatalogVisibilityButtonView;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use tw_macro::assert_component;

use model::CatalogVisibilityButtonModel;

#[component]
pub fn CatalogVisibilityButton(props: CatalogVisibilityButtonModel) -> Element {
    let label = props.label;
    let active = props.is_active;
    let title = Some(props.title);
    let onclick = props.on_toggle;
    rsx! {
        ToggleButton {
            label,
            active,
            title,
            onclick,
        }
    }
}

assert_component!(CatalogVisibilityButton);
