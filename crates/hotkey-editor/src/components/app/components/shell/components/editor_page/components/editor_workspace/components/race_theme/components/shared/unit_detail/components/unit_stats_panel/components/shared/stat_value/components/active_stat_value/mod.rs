mod model;
mod view;

pub use view::ActiveStatValueView;
mod style;

use dioxus::prelude::*;
use model::ActiveStatValueModel;
use style::CLASS;
use tw_macro::assert_component;

/// The active (non-muted) value look: primary text, medium weight. Rendered by the
/// [`StatValue`](super::super::StatValue) dispatcher when the figure is not muted.
#[component]
pub fn ActiveStatValue(props: ActiveStatValueModel) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(ActiveStatValue);
