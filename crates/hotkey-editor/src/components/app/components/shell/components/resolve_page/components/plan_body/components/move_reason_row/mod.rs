pub mod components;
mod props;
mod style;
use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use components::reason_badges::fight_badge::FightBadge;
use components::reason_badges::gap_pull_badge::GapPullBadge;
use components::reason_badges::spill_badge::SpillBadge;
use components::reason_badges::stuck_badge::StuckBadge;
use components::reason_badges::swap_badge::SwapBadge;
use dioxus::prelude::*;
pub use props::MoveReasonRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveReasonRow);

/// The reason-badge row atop a move card. It routes the move's kind to the
/// matching per-kind badge wrapper, forwarding the domain label text.
#[component]
pub fn MoveReasonRow(props: MoveReasonRowProps) -> Element {
    let kind = props.kind;
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {
                match kind {
                    ReasonKind::Fight => rsx! { FightBadge { label } },
                    ReasonKind::GapPull => rsx! { GapPullBadge { label } },
                    ReasonKind::Spill => rsx! { SpillBadge { label } },
                    ReasonKind::Swap => rsx! { SwapBadge { label } },
                    ReasonKind::Stuck => rsx! { StuckBadge { label } },
                }
            }
        }
    }
}
