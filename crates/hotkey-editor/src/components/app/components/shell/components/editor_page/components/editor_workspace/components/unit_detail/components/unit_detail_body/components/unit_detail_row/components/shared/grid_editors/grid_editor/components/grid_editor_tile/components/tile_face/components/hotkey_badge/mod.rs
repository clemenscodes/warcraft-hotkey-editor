mod props;
mod state;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::HotkeyBadgeProps;
pub use state::HotkeyBadgeState;
assert_component!(HotkeyBadge);

#[component]
pub fn HotkeyBadge(props: HotkeyBadgeProps) -> Element {
    let class = style::class(props.state);
    let label = props.letter.display_label();
    rsx! {
        span { class, {label} }
    }
}
