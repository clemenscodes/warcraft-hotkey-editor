mod model;
mod view;

pub use view::AvailableKeyView;
mod style;

use super::shared::key_cap_label::KeyCapLabel;
use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use model::AvailableKeyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AvailableKey(props: AvailableKeyModel) -> Element {
    let AvailableKeyModel {
        label,
        disabled,
        onclick,
        tooltip_text,
        tooltip_placement,
        tooltip_anchor,
    } = props;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            onclick,
            KeyCapLabel {
                label,
            }
            Tooltip {
                text: tooltip_text,
                placement: tooltip_placement,
                anchor: tooltip_anchor,
            }
        }
    }
}

assert_component!(AvailableKey);
