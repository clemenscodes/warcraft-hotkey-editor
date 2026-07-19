pub mod components;
mod model;
mod view;

pub use view::AlsoIncludeSwitchView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::shared::filter_switch::FilterSwitch;
use components::switch_label::SwitchLabel;
use dioxus::prelude::*;
use model::AlsoIncludeSwitchModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AlsoIncludeSwitch(props: AlsoIncludeSwitchModel) -> Element {
    let label = props.label;
    let popover_text = props.popover_text;
    let is_on = props.is_on;
    let onclick = props.onclick;
    rsx! {
        div {
            class: CLASS,
            SwitchLabel {
                text: label,
                popover_text,
            }
            FilterSwitch {
                is_on,
                onclick,
            }
        }
    }
}

assert_component!(AlsoIncludeSwitch);
