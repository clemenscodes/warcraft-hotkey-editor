use super::model::HotkeyUnitDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

pub(super) const EMPTY_PROMPT: &str = "Select a unit to inspect.";

pub(super) fn selected(
    props: &HotkeyUnitDetailBodyModel,
    selected_unit: Signal<Option<String>>,
) -> Option<HotkeyUnitView> {
    let selected_key = selected_unit.read().clone();
    let key = selected_key?;
    let unit_view = props
        .units
        .iter()
        .find(|unit_view| unit_view.key() == key)?
        .clone();
    Some(unit_view)
}
