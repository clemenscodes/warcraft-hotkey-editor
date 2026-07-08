pub mod components;
mod hooks;
mod props;
mod style;

use crate::services::focus::context::use_focus_coordinator;
use components::unit_card_surface::{UnitCardSurface, UnitCardSurfaceProps};
use dioxus::prelude::*;
use hooks::use_unit_card;
pub use props::UnitCardProps;
use std::rc::Rc;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitCard);

/// One selectable unit in the list: portrait plus name and id. Selecting it drives
/// the unit-detail panel. A thin identity wrapper that owns the card's placement box
/// and per-kind carousel filter and nests its own `UnitCardSurface` button for the
/// look, the generic `--race-color` accent, and the select handlers.
#[component]
pub fn UnitCard(props: UnitCardProps) -> Element {
    let model = use_unit_card(&props);
    let is_selected = props.is_selected;
    let icon_path = props.icon_path.clone();
    let display_name = props.display_name.clone();
    let unit_id = props.unit_id.clone();
    let race = props.race;
    let focus = use_focus_coordinator();
    let mut mounted_handle = use_signal(|| None::<Rc<MountedData>>);
    let unit_id_for_focus = props.unit_id.clone();
    let selected_unit_id = props.selected_unit_id;
    // Register this card as the unit-card focus target exactly while it is the selected
    // unit — read from the selection signal, never from a `data-selected` DOM query.
    use_effect(move || {
        if selected_unit_id.read().as_deref() == Some(unit_id_for_focus.as_str()) {
            let handle = mounted_handle.read().clone();
            focus.set_unit_card_handle(handle);
        }
    });
    let on_mounted = EventHandler::new(move |event: Event<MountedData>| {
        mounted_handle.set(Some(event.data()));
    });
    let surface = UnitCardSurfaceProps {
        icon_path,
        display_name,
        unit_id,
        race,
        is_selected,
        onclick: model.on_click,
        onkeydown: model.on_keydown,
        onmounted: on_mounted,
    };
    rsx! {
        div {
            class: CLASS,
            "data-unit-kind": model.kind_attr,
            UnitCardSurface { ..surface }
        }
    }
}
