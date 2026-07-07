use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;
use dioxus::prelude::*;

/// The two abilities flanking the shared-key badge in a pair clash.
#[derive(Clone, PartialEq)]
pub struct AbilityPair {
    pub(super) left: ConflictAbilityProps,
    pub(super) right: ConflictAbilityProps,
}

impl AbilityPair {
    pub fn new(left: ConflictAbilityProps, right: ConflictAbilityProps) -> Self {
        Self { left, right }
    }
}

/// The pair-clash row: two abilities flanking the badge, or nothing when the clash
/// is not a two-ability pair.
#[derive(Props, Clone, PartialEq)]
pub struct HotkeyPairRowProps {
    pub pair: Option<AbilityPair>,
    #[props(into)]
    pub hotkey_label: String,
}
