mod model;
mod view;

pub use view::PassiveHotkeyBadgeView;
mod style;

use dioxus::prelude::*;
use model::PassiveHotkeyBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PassiveHotkeyBadge(props: PassiveHotkeyBadgeModel) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(PassiveHotkeyBadge);
