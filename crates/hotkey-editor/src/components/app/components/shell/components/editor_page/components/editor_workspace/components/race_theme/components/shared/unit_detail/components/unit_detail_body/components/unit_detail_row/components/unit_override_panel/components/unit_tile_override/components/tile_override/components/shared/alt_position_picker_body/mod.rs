pub mod components;
mod props;
mod style;

use components::alt_position_picker_explainer::{
    AltPositionPickerExplainer, AltPositionPickerExplainerProps,
};
use components::alt_position_picker_grid_anchor::{
    AltPositionPickerGridAnchor, AltPositionPickerGridAnchorProps,
};
use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::AltPositionPickerBodyProps;

/// The scroll body of a position-picker dialog: the instruction explainer above the
/// embedded command grid, centered in the dialog's own scroll region. Shared by the
/// off-state and upgraded-form pickers.
#[component]
pub fn AltPositionPickerBody(props: AltPositionPickerBodyProps) -> Element {
    let explainer = AltPositionPickerExplainerProps::from(&props);
    let anchor = AltPositionPickerGridAnchorProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            AltPositionPickerExplainer { ..explainer }
            AltPositionPickerGridAnchor { ..anchor }
        }
    }
}

assert_component!(AltPositionPickerBody);
