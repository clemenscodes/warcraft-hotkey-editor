mod model;
mod view;

pub use view::NormalHotkeyBadgeView;
mod style;

use dioxus::prelude::*;
use model::NormalHotkeyBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NormalHotkeyBadge(props: NormalHotkeyBadgeModel) -> Element {
    let label = props.letter.display_label();
    rsx! {
        span {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(NormalHotkeyBadge);
