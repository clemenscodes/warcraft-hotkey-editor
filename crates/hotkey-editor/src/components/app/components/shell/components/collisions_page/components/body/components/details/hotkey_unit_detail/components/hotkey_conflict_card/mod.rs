mod logic;
mod props;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card::ConflictCard;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::HotkeyConflictCardProps;

/// One shared-hotkey conflict card: the abilities flanking (or stacked under) the
/// shared key badge, captioned by the command card the clash lives on. Each of the
/// two layouts renders itself away when it does not apply.
use tw_macro::assert_component;
assert_component!(HotkeyConflictCard);
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        ConflictCard {
            ConflictCardCaption { ..model.caption }
            ConflictPairRow { ..model.pair_row }
            ConflictMultiStack { ..model.multi_stack }
        }
    }
}
