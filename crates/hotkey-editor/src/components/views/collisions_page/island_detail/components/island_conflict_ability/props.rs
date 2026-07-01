use super::super::super::CarrierDialogData;
use dioxus::prelude::*;

/// One ability of an island conflict: a big icon and name that open the carriers
/// dialog, plus an optional "+N more" link when the ability is carried by more
/// units than the one shown.
#[derive(Props, Clone, PartialEq)]
pub struct IslandConflictAbilityProps {
    #[props(into)]
    pub ability_name: String,
    #[props(into)]
    pub ability_id: String,
    pub icon_url: Option<String>,
    pub extra_count: usize,
    pub carrier_unit_ids: Vec<String>,
    pub carrier_dialog: Signal<Option<CarrierDialogData>>,
}
