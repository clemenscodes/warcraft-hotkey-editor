use super::view::UpgradeTierView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// Tier-cycling footer inputs: the object being edited, the active/total tier counts,
/// and the caption. The stored per-object tier overrides it cycles are read from editor
/// context, so they are not a prop.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeTierModel {
    pub object_id: WarcraftObjectId,
    pub active_tier_index: usize,
    pub total_tier_count: usize,
    #[props(into)]
    pub tier_label_text: String,
}

impl From<&UpgradeTierView> for UpgradeTierModel {
    fn from(view: &UpgradeTierView) -> Self {
        let UpgradeTierView {
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

impl ddd::Model for UpgradeTierModel {
    type View = UpgradeTierView;
}
