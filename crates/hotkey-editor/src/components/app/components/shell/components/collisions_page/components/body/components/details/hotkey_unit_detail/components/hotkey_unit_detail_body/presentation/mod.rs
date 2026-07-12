use super::model::HotkeyUnitDetailBodyModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

/// The prompt shown in the hotkey detail pane before a unit is selected.
pub(super) const EMPTY_PROMPT: &str = "Select a unit to inspect.";

/// Resolves the selected unit's view, or `None` when nothing is selected. The selection
/// is read from context by the caller and passed in; the filled pane shapes the header
/// and cards from the returned domain view.
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
