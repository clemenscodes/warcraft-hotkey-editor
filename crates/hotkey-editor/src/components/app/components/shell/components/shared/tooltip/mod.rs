pub mod components;
mod logic;
mod props;
mod state;

use components::above_center_tooltip::{AboveCenterTooltip, AboveCenterTooltipProps};
use components::above_left_tooltip::{AboveLeftTooltip, AboveLeftTooltipProps};
use components::above_right_tooltip::{AboveRightTooltip, AboveRightTooltipProps};
use components::below_center_tooltip::{BelowCenterTooltip, BelowCenterTooltipProps};
use components::below_left_tooltip::{BelowLeftTooltip, BelowLeftTooltipProps};
use components::below_right_tooltip::{BelowRightTooltip, BelowRightTooltipProps};
use dioxus::prelude::*;
pub use props::TooltipProps;
pub use state::{TooltipAnchor, TooltipPlacement};
use tw_macro::assert_component;
assert_component!(Tooltip);

/// A hover/focus tooltip bubble, shown above or below its trigger. Rendered as a
/// child of a `group/tooltip relative` trigger, it reveals on the trigger's hover
/// or keyboard focus. Renders nothing when its text is empty, so a caption with no
/// conflict shows no bubble. The one shared tooltip for the system-hotkey dialogs
/// (the system-hotkeys list, the inventory grid, and the on-screen key picker.)
///
/// It carries no look of its own — it is the dispatcher that reads the static
/// placement and anchor and renders the matching positioned bubble. Each of the
/// six looks (above/below × left/center/right) owns its own bubble and positioning
/// across both `@supports` bands; the body only chooses which to render.
#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    if props.text.is_empty() {
        return rsx! {};
    }
    match props.placement {
        TooltipPlacement::Above => match props.anchor {
            TooltipAnchor::Left => {
                let leaf = AboveLeftTooltipProps::from(&props);
                rsx! {
                    AboveLeftTooltip { ..leaf }
                }
            }
            TooltipAnchor::Center => {
                let leaf = AboveCenterTooltipProps::from(&props);
                rsx! {
                    AboveCenterTooltip { ..leaf }
                }
            }
            TooltipAnchor::Right => {
                let leaf = AboveRightTooltipProps::from(&props);
                rsx! {
                    AboveRightTooltip { ..leaf }
                }
            }
        },
        TooltipPlacement::Below => match props.anchor {
            TooltipAnchor::Left => {
                let leaf = BelowLeftTooltipProps::from(&props);
                rsx! {
                    BelowLeftTooltip { ..leaf }
                }
            }
            TooltipAnchor::Center => {
                let leaf = BelowCenterTooltipProps::from(&props);
                rsx! {
                    BelowCenterTooltip { ..leaf }
                }
            }
            TooltipAnchor::Right => {
                let leaf = BelowRightTooltipProps::from(&props);
                rsx! {
                    BelowRightTooltip { ..leaf }
                }
            }
        },
    }
}
