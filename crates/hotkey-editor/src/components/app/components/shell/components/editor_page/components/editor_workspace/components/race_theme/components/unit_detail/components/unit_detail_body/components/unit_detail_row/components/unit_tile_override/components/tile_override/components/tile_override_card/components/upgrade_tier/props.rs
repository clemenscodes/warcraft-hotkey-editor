use dioxus::prelude::*;
use std::collections::HashMap;
use warcraft_api::WarcraftObjectId;

/// Tier-cycling footer inputs: the object being edited, the active/total tier counts,
/// the caption, and the stored per-object tier overrides.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeTierProps {
    pub object_id: WarcraftObjectId,
    pub active_tier_index: usize,
    pub total_tier_count: usize,
    #[props(into)]
    pub tier_label_text: String,
    pub tier_overrides: Signal<HashMap<String, usize>>,
}
