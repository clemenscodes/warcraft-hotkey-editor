use super::conflict_ability::ConflictAbilityProps;
use super::conflict_card_caption::ConflictCardCaptionProps;
use super::conflict_marker_view::ConflictMarker;
use super::conflict_multi_stack::ConflictMultiStackProps;
use super::conflict_pair_row::{AbilityPair, ConflictPairRowProps};

/// A conflict detail card's shaped body, shared by the hotkey and position cards:
/// the role-label caption plus the pair-row and multi-stack child props. Exactly one
/// of the two layouts renders; the other guards itself away. A two-ability clash
/// flanks the marker (pair row); any other count stacks above it (multi stack). Each
/// card supplies only its own marker and shaped abilities; the split is shared here.
pub(crate) struct ConflictCardModel {
    pub(crate) caption: ConflictCardCaptionProps,
    pub(crate) pair_row: ConflictPairRowProps,
    pub(crate) multi_stack: ConflictMultiStackProps,
}

/// Which clash presentation a card uses: an exact pair (two abilities) or a
/// multi-stack (any other count). Exactly one field is populated.
struct ClashLayout {
    pair: Option<AbilityPair>,
    multi: Vec<ConflictAbilityProps>,
}

impl ConflictCardModel {
    /// Splits the shaped abilities around the clash marker into the pair-vs-multi
    /// presentation: exactly two abilities become a pair row flanking the marker,
    /// any other count a stack above it.
    pub(crate) fn new(
        role_label: String,
        marker: ConflictMarker,
        abilities: Vec<ConflictAbilityProps>,
    ) -> Self {
        let clash = if abilities.len() == 2 {
            let mut ability_iter = abilities.into_iter();
            let left = ability_iter.next().expect("checked len == 2");
            let right = ability_iter.next().expect("checked len == 2");
            let pair_marker = marker.clone();
            let pair = AbilityPair::new(left, right, pair_marker);
            ClashLayout {
                pair: Some(pair),
                multi: Vec::new(),
            }
        } else {
            ClashLayout {
                pair: None,
                multi: abilities,
            }
        };
        let pair_row = ConflictPairRowProps { pair: clash.pair };
        let multi_stack = ConflictMultiStackProps {
            abilities: clash.multi,
            marker,
        };
        let caption = ConflictCardCaptionProps { text: role_label };
        Self {
            caption,
            pair_row,
            multi_stack,
        }
    }
}
