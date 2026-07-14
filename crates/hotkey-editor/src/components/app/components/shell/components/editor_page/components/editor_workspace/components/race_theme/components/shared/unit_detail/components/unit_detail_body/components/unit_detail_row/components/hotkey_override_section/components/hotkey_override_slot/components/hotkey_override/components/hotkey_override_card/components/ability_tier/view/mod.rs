use warcraft_api::WarcraftObjectId;

/// The published `View` contract mirroring [`AbilityTierModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AbilityTierView {
    pub object_id: WarcraftObjectId,
    pub active_tier_index: usize,
    pub total_tier_count: usize,
    pub tier_label_text: String,
}

impl ddd::View for AbilityTierView {}
