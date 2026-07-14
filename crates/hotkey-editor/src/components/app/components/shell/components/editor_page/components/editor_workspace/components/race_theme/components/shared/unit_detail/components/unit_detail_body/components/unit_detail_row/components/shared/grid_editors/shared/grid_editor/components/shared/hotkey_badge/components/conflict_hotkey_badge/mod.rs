mod model;
mod view;

pub use view::ConflictHotkeyBadgeView;
mod style;

use dioxus::prelude::*;
use model::ConflictHotkeyBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictHotkeyBadge(props: ConflictHotkeyBadgeModel) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ConflictHotkeyBadge);
