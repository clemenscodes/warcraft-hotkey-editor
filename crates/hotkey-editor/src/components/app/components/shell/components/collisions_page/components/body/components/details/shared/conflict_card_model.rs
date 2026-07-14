use super::conflict_marker::ConflictMarker;
use super::conflict_pair_row::AbilityPair;
use warcraft_api::WarcraftObjectId;

#[derive(Clone, PartialEq)]
pub(crate) struct ConflictAbilityData {
    pub(crate) name: String,
    pub(crate) ability_id: WarcraftObjectId,
    pub(crate) icon_url: Option<String>,
    pub(crate) unit_id: WarcraftObjectId,
}

#[derive(Clone, PartialEq)]
pub(crate) struct ConflictCardModel {
    pub(crate) role_label: String,
    pub(crate) pair: Option<AbilityPair>,
    pub(crate) multi: Vec<ConflictAbilityData>,
    pub(crate) marker: ConflictMarker,
}

struct ClashLayout {
    pair: Option<AbilityPair>,
    multi: Vec<ConflictAbilityData>,
}

impl ConflictCardModel {
    pub(crate) fn new(
        role_label: String,
        marker: ConflictMarker,
        abilities: Vec<ConflictAbilityData>,
    ) -> Self {
        let clash = if abilities.len() == 2 {
            let mut ability_iter = abilities.into_iter();
            let left = ability_iter.next().expect("checked len == 2");
            let right = ability_iter.next().expect("checked len == 2");
            let pair_marker = marker.clone();
            let ability_pair = AbilityPair::new(left, right, pair_marker);
            ClashLayout {
                pair: Some(ability_pair),
                multi: Vec::new(),
            }
        } else {
            ClashLayout {
                pair: None,
                multi: abilities,
            }
        };
        Self {
            role_label,
            pair: clash.pair,
            multi: clash.multi,
            marker,
        }
    }
}
