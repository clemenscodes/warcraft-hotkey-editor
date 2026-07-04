pub mod components;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::Dialog;
use components::carrier_card::CarrierCard;
use components::carriers_grid::CarriersGrid;
use dioxus::prelude::*;
use logic::cards;
pub use props::CarriersDialogProps;

/// Lists every unit that carries an ability in a scrollable grid; closing the
/// dialog clears the state that summoned it.
#[component]
pub fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let mut carriers_dialog = props.carriers_dialog;
    let title = props.dialog_data.ability_name.clone();
    let open = use_signal(|| true);
    let cards = cards(&props);
    use_effect(move || {
        if !open() {
            carriers_dialog.set(None);
        }
    });
    rsx! {
        Dialog { open, title,
            CarriersGrid {
                for card in cards {
                    CarrierCard { ..card }
                }
            }
        }
    }
}
