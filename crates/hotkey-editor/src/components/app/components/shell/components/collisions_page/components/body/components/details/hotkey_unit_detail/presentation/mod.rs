use super::model::HotkeyUnitDetailModel;
use crate::components::app::components::shell::components::collisions_page::presentation::HotkeyUnitView;
use dioxus::prelude::*;

/// Resolves the selected unit's view, or `None` when nothing is selected. The selection
/// is read from context by the caller and passed in; the filled pane shapes the header
/// and cards from the returned domain view.
pub(super) fn selected(
    props: &HotkeyUnitDetailModel,
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
