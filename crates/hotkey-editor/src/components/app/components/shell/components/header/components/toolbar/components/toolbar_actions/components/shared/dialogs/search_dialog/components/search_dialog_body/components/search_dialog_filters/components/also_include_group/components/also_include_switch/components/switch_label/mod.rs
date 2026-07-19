mod model;
mod view;

pub use view::SwitchLabelView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::shared::info_popover::InfoPopover;
use dioxus::prelude::*;
use model::SwitchLabelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn SwitchLabel(props: SwitchLabelModel) -> Element {
    let text = props.text;
    let popover_text = props.popover_text;
    rsx! {
        span {
            class: CLASS,
            {text}
            InfoPopover {
                text: popover_text,
            }
        }
    }
}

assert_component!(SwitchLabel);
