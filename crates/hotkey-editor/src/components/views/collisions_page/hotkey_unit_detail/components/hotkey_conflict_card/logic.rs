use super::props::HotkeyConflictCardProps;
use crate::components::views::collisions_page::conflict_ability::ConflictAbilityProps;

/// The two abilities flanking the badge in a pair clash.
pub(super) struct AbilityPair {
    pub(super) left: ConflictAbilityProps,
    pub(super) right: ConflictAbilityProps,
}

/// The card's shaped view: the shared key, the caption, and either the two-sided
/// pair or the multi-way ability list.
pub(super) struct HotkeyConflictCardModel {
    pub(super) hotkey_label: String,
    pub(super) role_label: String,
    pub(super) pair: Option<AbilityPair>,
    pub(super) multi: Vec<ConflictAbilityProps>,
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
        Self {
            hotkey_label,
            role_label,
            pair,
            multi,
        }
    }
}
