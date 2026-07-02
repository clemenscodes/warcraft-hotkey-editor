use super::components::hotkey_multi_stack::HotkeyMultiStackProps;
use super::components::hotkey_pair_row::{AbilityPair, HotkeyPairRowProps};
use super::props::HotkeyConflictCardProps;
use crate::components::views::collisions_page::components::body::components::details::shared::conflict_ability::ConflictAbilityProps;

/// The card's shaped view: the caption plus the pair-row and multi-stack child
/// props (exactly one of which renders; the other guards itself away).
pub(super) struct HotkeyConflictCardModel {
    pub(super) role_label: String,
    pub(super) pair_row: HotkeyPairRowProps,
    pub(super) multi_stack: HotkeyMultiStackProps,
}

impl From<&HotkeyConflictCardProps> for HotkeyConflictCardModel {
    fn from(props: &HotkeyConflictCardProps) -> Self {
        let hotkey_label = props.conflict.hotkey_label().to_owned();
        let role_label = props.conflict.role_label().to_owned();
        let abilities: Vec<ConflictAbilityProps> = props
            .conflict
            .abilities()
            .iter()
            .map(|ability| ConflictAbilityProps {
                ability_name: ability.name().to_owned(),
                ability_id: ability.object_id().to_owned(),
                icon_url: ability.icon_url().map(str::to_owned),
                unit_id: props.unit_id.clone(),
                view_navigation: props.view_navigation,
            })
            .collect();
        let (pair, multi) = if abilities.len() == 2 {
            let mut iter = abilities.into_iter();
            let left = iter.next().expect("checked len == 2");
            let right = iter.next().expect("checked len == 2");
            let pair = AbilityPair { left, right };
            (Some(pair), Vec::new())
        } else {
            (None, abilities)
        };
        let pair_row = HotkeyPairRowProps {
            pair,
            hotkey_label: hotkey_label.clone(),
        };
        let multi_stack = HotkeyMultiStackProps {
            abilities: multi,
            hotkey_label,
        };
        Self {
            role_label,
            pair_row,
            multi_stack,
        }
    }
}
