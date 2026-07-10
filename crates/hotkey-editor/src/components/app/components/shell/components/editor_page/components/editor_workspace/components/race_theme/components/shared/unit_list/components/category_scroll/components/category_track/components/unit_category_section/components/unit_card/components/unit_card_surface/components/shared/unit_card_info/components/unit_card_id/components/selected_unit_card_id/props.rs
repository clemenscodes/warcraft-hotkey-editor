use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit id the selected code element renders. Its race accent is read from the
/// theme container's `--race-accent`, so no race value is threaded in.
#[derive(Props, Clone, PartialEq)]
pub struct SelectedUnitCardIdProps {
    pub unit_id: WarcraftObjectId,
}
