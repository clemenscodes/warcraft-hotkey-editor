mod props;
mod view;

pub use view::AltPositionPickerExplainerView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::AltPositionPickerExplainerProps;

/// The instruction line at the top of a position-picker dialog.
#[component]
pub fn AltPositionPickerExplainer(props: AltPositionPickerExplainerProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(AltPositionPickerExplainer);
