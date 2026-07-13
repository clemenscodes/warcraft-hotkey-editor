mod model;
mod view;

pub use view::SystemHotkeysListEntryLabelView;
mod style;

use dioxus::prelude::*;
use model::SystemHotkeysListEntryLabelModel;
use style::CLASS;
use tw_macro::assert_component;

/// The binding name shown in a system-hotkey list row.
#[component]
pub fn SystemHotkeysListEntryLabel(props: SystemHotkeysListEntryLabelModel) -> Element {
    let text = props.text;
    rsx! {
        span { class: CLASS, {text} }
    }
}

assert_component!(SystemHotkeysListEntryLabel);
