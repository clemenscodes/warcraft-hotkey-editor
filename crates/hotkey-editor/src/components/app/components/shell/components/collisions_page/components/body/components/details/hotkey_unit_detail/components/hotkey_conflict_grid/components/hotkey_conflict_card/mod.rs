mod components;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use components::hotkey_conflict_panel::{HotkeyConflictPanel, HotkeyConflictPanelProps};
use dioxus::prelude::*;
pub use props::HotkeyConflictCardProps;
use style::CONFLICT_CARD;
use tw_macro::assert_component;
assert_component!(HotkeyConflictCard);

/// One shared-hotkey conflict card: the e2e-coupled `.conflict-card` shell wrapping the
/// conflict panel. It shapes the card model and hands the panel its caption and the two
/// clash layouts.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    let panel = HotkeyConflictPanelProps {
        caption: model.caption,
        pair_row: model.pair_row,
        multi_stack: model.multi_stack,
    };
    rsx! {
        div {
            class: CONFLICT_CARD,
            HotkeyConflictPanel { ..panel }
        }
    }
}
