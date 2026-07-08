mod props;
mod style;
use crate::components::app::components::shell::components::shared::panel_card::{
    PanelCard, PanelCardProps,
};
use dioxus::prelude::*;
pub use props::MoveCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(MoveCard);

/// A plan move card: its own identity shell around the shared `PanelCard`
/// surface. `is_stuck` picks the orc-tinted variant for unresolved abilities.
#[component]
pub fn MoveCard(props: MoveCardProps) -> Element {
    let is_stuck = props.is_stuck;
    let panel = PanelCardProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            "data-stuck": is_stuck,
            PanelCard { ..panel }
        }
    }
}
