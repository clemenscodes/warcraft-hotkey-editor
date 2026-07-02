pub mod components;
mod logic;
mod props;

use crate::components::views::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use crate::components::views::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use components::hotkey_multi_stack::HotkeyMultiStack;
use components::hotkey_pair_row::HotkeyPairRow;
use dioxus::prelude::*;
use logic::HotkeyConflictCardModel;
pub use props::HotkeyConflictCardProps;

/// One shared-hotkey conflict card: the abilities flanking (or stacked under) the
/// shared key badge, captioned by the command card the clash lives on. Each of the
/// two layouts renders itself away when it does not apply.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let model = HotkeyConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            ConflictCardCaption { text: model.role_label }
            HotkeyPairRow { ..model.pair_row }
            HotkeyMultiStack { ..model.multi_stack }
        }
    }
}
