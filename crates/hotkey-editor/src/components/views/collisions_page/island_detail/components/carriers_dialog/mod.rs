pub mod components;
mod logic;
mod props;

use crate::components::dialogs::dialog::Dialog;
use components::carrier_card::CarrierCard;
use components::carriers_grid::CarriersGrid;
use dioxus::prelude::*;
use logic::cards;
pub use props::CarriersDialogProps;

/// Lists every unit that carries the shared ability in a scrollable grid; closing
/// the dialog clears the state that summoned it.
#[component]
pub fn CarriersDialog(props: CarriersDialogProps) -> Element {
    let mut carrier_dialog = props.carrier_dialog;
    let title = props.dialog_data.ability_name().to_owned();
    let open = use_signal(|| true);
    let cards = cards(&props);
    use_effect(move || {
        if !open() {
            carrier_dialog.set(None);
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
