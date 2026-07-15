pub mod components;
mod model;
mod view;

pub use view::CatalogVisibilitySwitchView;
mod style;

use components::switch_item_label::SwitchItemLabel;
use components::switch_toggle::SwitchToggle;
use dioxus::prelude::*;
use model::CatalogVisibilitySwitchModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn CatalogVisibilitySwitch(props: CatalogVisibilitySwitchModel) -> Element {
    let label = props.label;
    let is_on = props.is_active;
    let title = Some(props.title);
    let onclick = props.on_toggle;
    rsx! {
        div {
            class: CLASS,
            SwitchItemLabel { text: label }
            SwitchToggle {
                is_on,
                title,
                aria_label: label,
                onclick,
            }
        }
    }
}

assert_component!(CatalogVisibilitySwitch);
