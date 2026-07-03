use dioxus::prelude::*;
use warcraft_api::{Race, UnitKind};

/// One mobile category tab: its kind, whether it is the active category, the race
/// whose accent colour it wears when active or hovered, and the signal it writes on
/// tap.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabProps {
    pub kind: UnitKind,
    pub is_active: bool,
    pub race: Race,
    pub active_category: Signal<UnitKind>,
}
