pub mod components;
mod props;
mod style;

use components::alt_state_controls::{AltStateControls, AltStateControlsProps};
use components::alt_state_header_label_column::{
    AltStateHeaderLabelColumn, AltStateHeaderLabelColumnProps,
};
use dioxus::prelude::*;
pub use props::AltStateHeaderProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AltStateHeader);

/// The off-state block's top row: the label column beside the editable controls.
#[component]
pub fn AltStateHeader(props: AltStateHeaderProps) -> Element {
    let label_column = AltStateHeaderLabelColumnProps::from(&props);
    let controls = AltStateControlsProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            AltStateHeaderLabelColumn { ..label_column }
            AltStateControls { ..controls }
        }
    }
}
