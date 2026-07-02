use crate::components::views::resolve_page::logic::CarriersDialogData;
use dioxus::prelude::*;

/// One ability icon with its carrier-count badge; clicking opens the carriers
/// dialog. The winner of a Fight is ringed gold.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityIconProps {
    #[props(into)]
    pub name: String,
    pub icon_url: Option<String>,
    pub carrier_count: usize,
    pub carrier_unit_ids: Vec<String>,
    pub is_winner: bool,
    pub carriers_dialog: Signal<Option<CarriersDialogData>>,
}
