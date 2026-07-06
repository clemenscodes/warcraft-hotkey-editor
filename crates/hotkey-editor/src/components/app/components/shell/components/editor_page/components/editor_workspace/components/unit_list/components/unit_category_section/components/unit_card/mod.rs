pub mod components;
mod hooks;
mod props;
mod style;

use crate::services::focus::context::use_focus_coordinator;
use components::unit_card_icon::{UnitCardIcon, UnitCardIconProps};
use components::unit_card_info::{UnitCardInfo, UnitCardInfoProps};
use dioxus::prelude::*;
use hooks::use_unit_card;
pub use props::UnitCardProps;
use std::rc::Rc;
use tw_macro::assert_component;
assert_component!(UnitCard);

/// One selectable unit in the list: portrait plus name and id. Selecting it drives
/// the unit-detail panel.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let model = use_unit_card(&props);
    let icon = UnitCardIconProps::from(&props);
    let info = UnitCardInfoProps::from(&props);
    let is_selected = props.is_selected;
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    let unit_id = props.unit_id.clone();
    let selected_unit_id = props.selected_unit_id;
    // Register this card as the unit-card focus target exactly while it is the selected
    // unit — read from the selection signal, never from a `data-selected` DOM query.
    use_effect(move || {
        if selected_unit_id.read().as_deref() == Some(unit_id.as_str()) {
            let handle = mounted_handle.read().clone();
            focus.set_unit_card_handle(handle);
        }
    });
    rsx! {
        button {
            class: model.class,
            "data-unit-kind": model.kind_attr,
            "data-selected": is_selected,
            onmounted: move |event: Event<MountedData>| mounted_handle.set(Some(event.data())),
            onclick: model.on_click,
            onkeydown: model.on_keydown,
            UnitCardIcon { ..icon }
            UnitCardInfo { ..info }
        }
    }
}
