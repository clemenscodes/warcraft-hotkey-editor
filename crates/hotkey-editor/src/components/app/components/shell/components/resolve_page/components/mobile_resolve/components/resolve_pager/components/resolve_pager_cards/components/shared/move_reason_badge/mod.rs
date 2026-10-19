pub mod components;
mod model;
mod view;

pub use view::MoveReasonBadgeView;
mod style;
use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;
use components::reason_badges::fight_reason_badge::FightReasonBadge;
use components::reason_badges::gap_pull_reason_badge::GapPullReasonBadge;
use components::reason_badges::spill_reason_badge::SpillReasonBadge;
use components::reason_badges::stuck_reason_badge::StuckReasonBadge;
use components::reason_badges::swap_reason_badge::SwapReasonBadge;
use dioxus::prelude::*;
use model::MoveReasonBadgeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MoveReasonBadge(props: MoveReasonBadgeModel) -> Element {
    let kind = props.kind;
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {
                match kind {
                    ReasonKind::Fight => rsx! {
                        FightReasonBadge {
                            label,
                        }
                    },
                    ReasonKind::GapPull => rsx! {
                        GapPullReasonBadge {
                            label,
                        }
                    },
                    ReasonKind::Spill => rsx! {
                        SpillReasonBadge {
                            label,
                        }
                    },
                    ReasonKind::Swap => rsx! {
                        SwapReasonBadge {
                            label,
                        }
                    },
                    ReasonKind::Stuck => rsx! {
                        StuckReasonBadge {
                            label,
                        }
                    },
                }
            }
        }
    }
}

assert_component!(MoveReasonBadge);
