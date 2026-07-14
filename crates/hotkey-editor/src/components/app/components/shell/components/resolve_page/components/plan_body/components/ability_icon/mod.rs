pub mod components;
mod model;
mod presentation;
mod view;

pub use view::AbilityIconView;

mod style;
use crate::components::app::components::shell::components::shared::carriers_dialog::CarriersDialog;
use components::carrier_badge::CarrierBadge;
use components::fight_icon::FightIcon;
use dioxus::prelude::*;
use model::AbilityIconModel;
use presentation::{AbilityIconPresentation, use_ability_icon};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn AbilityIcon(props: AbilityIconModel) -> Element {
    let AbilityIconPresentation {
        mut open_state,
        icon_src,
        icon_alt,
        count,
        is_winner,
        title,
        disabled,
        onclick,
    } = use_ability_icon(&props);
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            disabled,
            title,
            onclick,
            FightIcon {
                src: icon_src,
                alt: icon_alt,
            }
            CarrierBadge {
                count,
                is_winner,
            }
        }
        CarriersDialog {
            ability: open_state.read().clone(),
            on_close: Callback::new(move |()| open_state.set(None)),
        }
    }
}

assert_component!(AbilityIcon);
