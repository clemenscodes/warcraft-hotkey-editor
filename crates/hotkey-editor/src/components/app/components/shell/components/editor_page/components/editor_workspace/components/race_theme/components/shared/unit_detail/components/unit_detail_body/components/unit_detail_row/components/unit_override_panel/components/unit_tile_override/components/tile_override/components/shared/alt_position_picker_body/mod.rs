pub mod components;
mod props;
mod style;

use components::alt_position_picker_explainer::AltPositionPickerExplainer;
use components::alt_position_picker_grid_anchor::AltPositionPickerGridAnchor;
use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::AltPositionPickerBodyProps;

/// The scroll body of a position-picker dialog: the instruction explainer above the
/// embedded command grid, centered in the dialog's own scroll region. Shared by the
/// off-state and upgraded-form pickers.
#[component]
pub fn AltPositionPickerBody(props: AltPositionPickerBodyProps) -> Element {
    let AltPositionPickerBodyProps {
        explainer_text,
        grid_config,
    } = props;
    rsx! {
        div {
            class: CLASS,
            AltPositionPickerExplainer { text: explainer_text }
            AltPositionPickerGridAnchor { grid_config }
        }
    }
}

assert_component!(AltPositionPickerBody);
