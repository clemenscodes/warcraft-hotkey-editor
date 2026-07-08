pub mod components;
mod logic;
mod props;
mod state;

use components::human_reason_badge::{HumanReasonBadge, HumanReasonBadgeProps};
use components::orc_reason_badge::{OrcReasonBadge, OrcReasonBadgeProps};
use components::success_reason_badge::{SuccessReasonBadge, SuccessReasonBadgeProps};
use components::undead_reason_badge::{UndeadReasonBadge, UndeadReasonBadgeProps};
use dioxus::prelude::*;
pub use props::ReasonBadgeProps;
pub use state::ReasonBadgeColor;
use tw_macro::assert_component;
assert_component!(ReasonBadge);

/// The reason badge: a colour-coded pill showing its label. A pure dispatcher: from
/// the colour it renders the matching per-colour look — orc, human, undead, or
/// success. Each look owns its own classed `span` root; the text arrives as a prop
/// and the per-kind wrapper picks the colour.
#[component]
pub fn ReasonBadge(props: ReasonBadgeProps) -> Element {
    match props.color {
        ReasonBadgeColor::Orc => {
            let badge = OrcReasonBadgeProps::from(&props);
            rsx! {
                OrcReasonBadge { ..badge }
            }
        }
        ReasonBadgeColor::Human => {
            let badge = HumanReasonBadgeProps::from(&props);
            rsx! {
                HumanReasonBadge { ..badge }
            }
        }
        ReasonBadgeColor::Undead => {
            let badge = UndeadReasonBadgeProps::from(&props);
            rsx! {
                UndeadReasonBadge { ..badge }
            }
        }
        ReasonBadgeColor::Success => {
            let badge = SuccessReasonBadgeProps::from(&props);
            rsx! {
                SuccessReasonBadge { ..badge }
            }
        }
    }
}
