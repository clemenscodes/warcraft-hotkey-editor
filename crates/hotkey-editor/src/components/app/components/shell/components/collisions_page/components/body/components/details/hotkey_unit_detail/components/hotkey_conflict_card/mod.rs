mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::HotkeyConflictCardProps;
use style::{CONFLICT_CARD, PANEL};
use tw_macro::assert_component;
assert_component!(HotkeyConflictCard);

/// One shared-hotkey conflict card: the abilities flanking (or stacked under) the
/// shared key badge, captioned by the command card the clash lives on. It owns its own
/// card surface directly. Each of the two layouts renders itself away when it does not
/// apply.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardProps) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        div {
            class: CONFLICT_CARD,
            div {
                class: PANEL,
                ConflictCardCaption { ..model.caption }
                ConflictPairRow { ..model.pair_row }
                ConflictMultiStack { ..model.multi_stack }
            }
        }
    }
}
