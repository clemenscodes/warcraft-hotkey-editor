use super::view::SelectedUnitCardIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id the selected code element renders. Its race accent is read from the
/// theme container's `--race-color`, so no race value is threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardIdProps {
    pub unit_id: WarcraftObjectId,
}

impl From<&SelectedUnitCardIdView> for SelectedUnitCardIdProps {
    fn from(view: &SelectedUnitCardIdView) -> Self {
        let SelectedUnitCardIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Props for SelectedUnitCardIdProps {
    type View = SelectedUnitCardIdView;
}
