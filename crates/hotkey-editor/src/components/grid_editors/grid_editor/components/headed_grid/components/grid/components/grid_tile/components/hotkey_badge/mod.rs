mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;

pub use props::HotkeyBadgeProps;
pub use state::HotkeyBadgeState;

assert_component!(HotkeyBadge);

#[component]
pub fn HotkeyBadge(props: HotkeyBadgeProps) -> Element {
    let class = style::class(props.state);
    let label = props.letter.display_label();
    rsx! {
        span {
            class,
            {label}
        }
    }
}
