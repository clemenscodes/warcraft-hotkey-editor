mod model;
mod view;

pub use view::SearchFieldButtonView;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use tw_macro::assert_component;

use model::SearchFieldButtonModel;

#[component]
pub fn SearchFieldButton(props: SearchFieldButtonModel) -> Element {
    let label = props.label;
    let active = props.is_active;
    let onclick = props.on_select;
    rsx! {
        ToggleButton {
            label,
            active,
            onclick,
        }
    }
}

assert_component!(SearchFieldButton);
