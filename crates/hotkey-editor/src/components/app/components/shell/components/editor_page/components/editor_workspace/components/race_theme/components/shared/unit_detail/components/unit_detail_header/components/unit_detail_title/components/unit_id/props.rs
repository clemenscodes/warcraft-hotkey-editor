use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The unit's database id, shown as a monospace caption.
#[derive(Props, Clone, PartialEq)]
pub struct UnitIdProps {
    pub unit_id: WarcraftObjectId,
}
