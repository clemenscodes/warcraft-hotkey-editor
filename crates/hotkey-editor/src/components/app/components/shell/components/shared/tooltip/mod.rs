mod logic;
mod props;
mod state;
mod style;

use dioxus::prelude::*;
use logic::TooltipPresentation;
pub use props::TooltipProps;
pub use state::{TooltipAnchor, TooltipPlacement};
use style::CLASS;
use tw_macro::assert_component;
assert_component!(Tooltip);

/// A hover/focus tooltip bubble, shown above or below its trigger. Rendered as a
/// child of a `group/tooltip relative` trigger, it reveals on the trigger's hover
/// or keyboard focus. Renders nothing when its text is empty, so a caption with no
/// conflict shows no bubble. The one shared tooltip for the system-hotkey dialogs
/// (the system-hotkeys list, the inventory grid, and the on-screen key picker.)
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    let presentation = TooltipPresentation::from(&props);
    if presentation.is_empty {
        return rsx! {};
    }
    rsx! {
        span {
            class: CLASS,
            "data-placement": presentation.placement,
            "data-anchor": presentation.anchor,
            {presentation.text}
        }
    }
}
