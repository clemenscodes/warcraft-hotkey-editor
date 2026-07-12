use super::view::AbilityTierView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// Tier-cycling footer inputs: the object being edited, the active/total tier counts,
/// and the caption. The stored per-object tier overrides it cycles are read from editor
/// context, so they are not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct AbilityTierModel {
    pub object_id: WarcraftObjectId,
    pub active_tier_index: usize,
    pub total_tier_count: usize,
    #[props(into)]
    pub tier_label_text: String,
}

impl From<&AbilityTierView> for AbilityTierModel {
    fn from(view: &AbilityTierView) -> Self {
        let AbilityTierView {
            object_id,
            active_tier_index,
            total_tier_count,
            tier_label_text,
        } = view.clone();
        Self {
            object_id,
            active_tier_index,
            total_tier_count,
            tier_label_text,
        }
    }
}

impl ddd::Model for AbilityTierModel {
    type View = AbilityTierView;
}
