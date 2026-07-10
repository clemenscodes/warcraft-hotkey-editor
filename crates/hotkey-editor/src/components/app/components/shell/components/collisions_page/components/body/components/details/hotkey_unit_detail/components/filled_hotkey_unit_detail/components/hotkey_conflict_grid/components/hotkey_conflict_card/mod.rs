mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::{
    ConflictPanel, ConflictPanelProps,
};
use dioxus::prelude::*;
pub use props::HotkeyConflictCardProps;
use style::CLASS;
use tw_macro::assert_component;

/// One shared-hotkey conflict card: the e2e-coupled `.conflict-card` shell wrapping the
/// conflict panel. It shapes the card model and hands the panel its caption and the two
/// clash layouts.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    let panel = ConflictPanelProps::from(model);
    rsx! {
        div {
            class: CLASS,
            ConflictPanel { ..panel }
        }
    }
}

assert_component!(HotkeyConflictCard);
