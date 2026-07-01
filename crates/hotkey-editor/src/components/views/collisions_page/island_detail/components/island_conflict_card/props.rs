use crate::components::views::collisions_page::{CarrierDialogData, ConflictView};
use crate::services::navigation::view_navigation::ViewNavigationContext;
use dioxus::prelude::*;

/// One island conflict: the affected unit, its two clashing abilities, the carrier
/// dialog signal the abilities open, and the navigation context.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictCardProps {
    pub conflict: ConflictView,
    pub carrier_dialog: Signal<Option<CarrierDialogData>>,
    pub view_navigation: ViewNavigationContext,
}
