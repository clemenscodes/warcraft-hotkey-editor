use super::view::UnitIdView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit's database id, shown as a monospace caption.
#[derive(Props, Clone, PartialEq)]
pub struct UnitIdProps {
    pub unit_id: WarcraftObjectId,
}

impl From<&UnitIdView> for UnitIdProps {
    fn from(view: &UnitIdView) -> Self {
        let UnitIdView { unit_id } = view.clone();
        Self { unit_id }
    }
}

impl ddd::Props for UnitIdProps {
    type View = UnitIdView;
}
