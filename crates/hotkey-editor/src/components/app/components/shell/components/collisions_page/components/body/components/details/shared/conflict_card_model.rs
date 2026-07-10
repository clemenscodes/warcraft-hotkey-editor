use super::conflict_marker_view::ConflictMarker;
use super::conflict_pair_row::AbilityPair;
use warcraft_api::WarcraftObjectId;

/// One clashing ability as domain data: its display name, object id, icon, and the unit
/// its icon deep-links to. Threaded into the pair row and the multi stack, which build
/// each `ConflictAbility` leaf from these fields.
#[derive(Clone, PartialEq)]
pub(crate) struct ConflictAbilityData {
    pub(crate) name: String,
    pub(crate) ability_id: WarcraftObjectId,
    pub(crate) icon_url: Option<String>,
    pub(crate) unit_id: WarcraftObjectId,
}

/// A conflict detail card's shaped body, shared by the hotkey and position cards: the
/// role-label caption plus the pair-vs-multi presentation of the clashing abilities.
/// Exactly one of the two layouts renders; the other guards itself away. A two-ability
/// clash flanks the marker (pair row); any other count stacks above it (multi stack).
/// Each card supplies only its own marker and shaped abilities; the split is shared here.
#[derive(Clone, PartialEq)]
pub(crate) struct ConflictCardModel {
    pub(crate) role_label: String,
    pub(crate) pair: Option<AbilityPair>,
    pub(crate) multi: Vec<ConflictAbilityData>,
    pub(crate) marker: ConflictMarker,
}

/// Which clash presentation a card uses: an exact pair (two abilities) or a multi-stack
/// (any other count). Exactly one field is populated.
struct ClashLayout {
    pair: Option<AbilityPair>,
    multi: Vec<ConflictAbilityData>,
}

impl ConflictCardModel {
    /// Splits the shaped abilities around the clash marker into the pair-vs-multi
    /// presentation: exactly two abilities become a pair row flanking the marker, any
    /// other count a stack above it.
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
