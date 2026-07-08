mod props;
mod style;
use crate::components::app::components::shell::components::shared::panel_card::{
    PanelCard, PanelCardProps,
};
use dioxus::prelude::*;
pub use props::ConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictCard);

/// A collision conflict card: its own identity shell around the shared
/// `PanelCard` surface in its centered variant.
#[component]
pub fn ConflictCard(props: ConflictCardProps) -> Element {
    let panel = PanelCardProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            PanelCard { ..panel }
        }
    }
}
