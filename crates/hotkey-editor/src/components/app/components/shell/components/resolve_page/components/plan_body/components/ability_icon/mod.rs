pub mod components;
mod model;
mod presentation;
mod view;

pub use view::AbilityIconView;

mod style;
use crate::components::app::components::shell::components::shared::carriers_dialog_host::CarriersDialogHost;
use components::carrier_badge::CarrierBadge;
use components::fight_icon::FightIcon;
use dioxus::prelude::*;
use model::AbilityIconModel;
use presentation::{AbilityIconPresentation, use_ability_icon};
use style::CLASS;
use tw_macro::assert_component;

/// One ability icon with a carrier-count badge; clicking opens this ability's carriers
/// dialog. The icon owns the open state and mounts the dialog's host beneath itself, so
/// no ancestor knows the dialog exists.
#[component]
pub fn AbilityIcon(props: AbilityIconModel) -> Element {
    let AbilityIconPresentation {
        open_state,
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
            FightIcon { src: icon_src, alt: icon_alt }
            CarrierBadge { count, is_winner }
        }
        CarriersDialogHost { open_state }
    }
}

assert_component!(AbilityIcon);
