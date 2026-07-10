use dioxus::prelude::*;
use dioxus_gallery::Story;
use hotkey_editor::components::app::components::shell::components::shared::carriers_dialog_host::components::carriers_dialog::components::carriers_dialog_panel::components::carriers_dialog_body::components::carriers_grid::components::carrier_card::{
    CarrierCard, CarrierCardProps,
};
use hotkey_editor::components::app::components::shell::components::shared::carriers_dialog_host::components::carriers_dialog::components::carriers_dialog_panel::components::carriers_dialog_body::CarriersDialogBody;

use super::editor_mount::EditorMount;
use crate::stories::fixtures;

pub fn stories() -> Vec<Story> {
    vec![
        Story::single("Carriers", "CarrierCard", carrier_card_default),
        Story::single("Carriers", "CarriersDialogBody", carriers_dialog_body),
    ]
}

fn sample_card() -> CarrierCardProps {
    let unit_id = fixtures::sample_unit_id();
    let icon_url = Some(fixtures::sample_icon_url());
    let name = "Footman".to_string();
    CarrierCardProps {
        unit_id,
        icon_url,
        name,
    }
}

fn carrier_card_default() -> Element {
    let card = sample_card();
    rsx! {
        EditorMount {
            CarrierCard { ..card }
        }
    }
}

fn carriers_dialog_body() -> Element {
    let hero_id = fixtures::sample_hero_id();
    let hero_icon = Some(fixtures::sample_icon_url());
    let hero_card = CarrierCardProps {
        unit_id: hero_id,
        icon_url: hero_icon,
        name: "Archmage".to_string(),
    };
    let cards = vec![sample_card(), hero_card];
    rsx! {
        EditorMount {
            CarriersDialogBody { cards }
        }
    }
}
