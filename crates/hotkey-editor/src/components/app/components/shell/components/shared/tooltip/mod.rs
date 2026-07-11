pub mod components;
mod model;
mod view;

pub use view::TooltipView;
mod state;

use components::above_center_tooltip::AboveCenterTooltip;
use components::above_left_tooltip::AboveLeftTooltip;
use components::above_right_tooltip::AboveRightTooltip;
use components::below_center_tooltip::BelowCenterTooltip;
use components::below_left_tooltip::BelowLeftTooltip;
use components::below_right_tooltip::BelowRightTooltip;
use dioxus::prelude::*;
use model::TooltipModel;
pub use state::{TooltipAnchor, TooltipPlacement};
use tw_macro::assert_component;

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
pub fn Tooltip(props: TooltipModel) -> Element {
    if props.text.is_empty() {
        return rsx! {};
    }
    match props.placement {
        TooltipPlacement::Above => match props.anchor {
            TooltipAnchor::Left => {
                let text = props.text.clone();
                rsx! {
                    AboveLeftTooltip { text }
                }
            }
            TooltipAnchor::Center => {
                let text = props.text.clone();
                rsx! {
                    AboveCenterTooltip { text }
                }
            }
            TooltipAnchor::Right => {
                let text = props.text.clone();
                rsx! {
                    AboveRightTooltip { text }
                }
            }
        },
        TooltipPlacement::Below => match props.anchor {
            TooltipAnchor::Left => {
                let text = props.text.clone();
                rsx! {
                    BelowLeftTooltip { text }
                }
            }
            TooltipAnchor::Center => {
                let text = props.text.clone();
                rsx! {
                    BelowCenterTooltip { text }
                }
            }
            TooltipAnchor::Right => {
                let text = props.text.clone();
                rsx! {
                    BelowRightTooltip { text }
                }
            }
        },
    }
}

assert_component!(Tooltip);
