pub mod components;
mod hooks;
mod props;
mod style;
use crate::components::app::components::shell::components::shared::carriers_dialog_host::CarriersDialogHost;
use components::carrier_badge::CarrierBadge;
use components::fight_icon::FightIcon;
use dioxus::prelude::*;
use hooks::{AbilityIconView, use_ability_icon};
pub use props::AbilityIconProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(AbilityIcon);

/// One ability icon with a carrier-count badge; clicking opens this ability's carriers
/// dialog. The icon owns the open state and mounts the dialog's host beneath itself, so
/// no ancestor knows the dialog exists.
#[component]
pub fn AbilityIcon(props: AbilityIconProps) -> Element {
    let AbilityIconView {
        open_state,
        icon,
        badge,
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
            FightIcon { ..icon }
            CarrierBadge { ..badge }
        }
        CarriersDialogHost { open_state }
    }
}
