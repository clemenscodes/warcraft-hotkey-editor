mod model;
mod view;

pub use view::NormalKeyChipView;
mod style;

use crate::components::app::components::shell::components::shared::tooltip::Tooltip;
use dioxus::prelude::*;
use model::NormalKeyChipModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NormalKeyChip(props: NormalKeyChipModel) -> Element {
    let label = props.label;
    let onclick = props.onclick;
    let text = props.tooltip_text;
    let placement = props.tooltip_placement;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            {label}
            Tooltip {
                text,
                placement,
            }
        }
    }
}

assert_component!(NormalKeyChip);
