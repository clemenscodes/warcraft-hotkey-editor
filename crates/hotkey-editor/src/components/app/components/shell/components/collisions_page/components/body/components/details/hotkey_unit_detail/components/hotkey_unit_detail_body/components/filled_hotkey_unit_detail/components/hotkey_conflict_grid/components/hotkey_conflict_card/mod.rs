mod model;
mod presentation;
mod view;

pub use view::HotkeyConflictCardView;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_model::ConflictCardModel;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_panel::ConflictPanel;
use dioxus::prelude::*;
use model::HotkeyConflictCardModel;
use style::CLASS;
use tw_macro::assert_component;

/// One shared-hotkey conflict card: the e2e-coupled `.conflict-card` shell wrapping the
/// conflict panel. It shapes the card model and hands the panel the shaped card.
#[component]
pub fn HotkeyConflictCard(props: HotkeyConflictCardModel) -> Element {
    let model = ConflictCardModel::from(&props);
    rsx! {
        div {
            class: CLASS,
            ConflictPanel {
                model,
            }
        }
    }
}

assert_component!(HotkeyConflictCard);
