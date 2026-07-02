use dioxus::prelude::*;
use warcraft_api::UnitKind;

/// One mobile category tab: its kind, whether it is the active category, and the
/// signal it writes on tap.
#[derive(Props, Clone, PartialEq)]
pub struct MobileCategoryTabProps {
    pub kind: UnitKind,
    pub is_active: bool,
    pub active_category: Signal<UnitKind>,
}
