//! Named, composable domain rules that decide how an ability behaves on a
//! unit's command card. Each rule is a [`ddd::Specification`] — a first-class
//! predicate that can be combined with `.and()`/`.or()`/`.not()` — replacing
//! the bare boolean helper functions this logic used to be. Every rule is a
//! stateless unit struct; the candidate it tests carries all the inputs,
//! including the game database when a rule needs to consult it.

use ddd::DomainLayer;
use ddd::Layered;
use ddd::Specification;
use warcraft_api::WarcraftDatabase;
use warcraft_api::WarcraftObjectId;
use warcraft_database::BuildingTraits;
use warcraft_database::UNIT_UPGRADE_SWAPS;

const ROOTED_ONLY_ABILITY_CODES: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Apit"), WarcraftObjectId::new("Aall")];

const ROOTED_ONLY_ABILITY_IDS: &[WarcraftObjectId] =
    &[WarcraftObjectId::new("Anei"), WarcraftObjectId::new("Aent")];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
struct HiddenUnitAbility {
    unit_id: WarcraftObjectId,
    ability_id: WarcraftObjectId,
}

const HIDDEN_UNIT_ABILITIES: &[HiddenUnitAbility] = &[
    HiddenUnitAbility {
        unit_id: WarcraftObjectId::new("hphx"),
        ability_id: WarcraftObjectId::new("Apxf"),
    },
    HiddenUnitAbility {
        unit_id: WarcraftObjectId::new("egol"),
        ability_id: WarcraftObjectId::new("Aenc"),
    },
];

/// An ability considered on a specific unit's command card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AbilityOnUnit<'a> {
    unit_id: &'a str,
    ability_id: &'a str,
}

impl<'a> AbilityOnUnit<'a> {
    pub fn new(unit_id: &'a str, ability_id: &'a str) -> Self {
        Self {
            unit_id,
            ability_id,
        }
    }
}

/// A pair of units tested for being the same trainable button at two tech tiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnitPair<'a> {
    first_unit_id: &'a str,
    second_unit_id: &'a str,
}

impl<'a> UnitPair<'a> {
    pub fn new(first_unit_id: &'a str, second_unit_id: &'a str) -> Self {
        Self {
            first_unit_id,
            second_unit_id,
        }
    }
}

/// A morph ability considered against the host unit it might revert to, together
/// with the database that answers the morph question.
#[derive(Clone, Copy, Debug)]
pub struct MorphAgainstHost<'a> {
    database: &'a WarcraftDatabase,
    ability_id: &'a str,
    host_unit_id: &'a str,
}

impl<'a> MorphAgainstHost<'a> {
    pub fn new(database: &'a WarcraftDatabase, ability_id: &'a str, host_unit_id: &'a str) -> Self {
        Self {
            database,
            ability_id,
            host_unit_id,
        }
    }
}

/// An ability looked up in the database to decide whether it is rooted-only.
#[derive(Clone, Copy, Debug)]
pub struct AbilityInDatabase<'a> {
    database: &'a WarcraftDatabase,
    ability_id: &'a str,
}

impl<'a> AbilityInDatabase<'a> {
    pub fn new(database: &'a WarcraftDatabase, ability_id: &'a str) -> Self {
        Self {
            database,
            ability_id,
        }
    }
}

/// The ability is deliberately hidden from the given unit's command card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct HiddenAbility;

impl Layered for HiddenAbility {
    type Layer = DomainLayer;
}

impl Specification<AbilityOnUnit<'_>> for HiddenAbility {
    fn is_satisfied_by(&self, candidate: &AbilityOnUnit<'_>) -> bool {
        HIDDEN_UNIT_ABILITIES.iter().any(|hidden| {
            hidden
                .unit_id
                .value()
                .eq_ignore_ascii_case(candidate.unit_id)
                && hidden
                    .ability_id
                    .value()
                    .eq_ignore_ascii_case(candidate.ability_id)
        })
    }
}

/// The two units are the same trainable button at different tech levels — a
/// genuine upgrade swap such as Headhunter → Berserker. Two distinct units that
/// merely share a default button cell are *not* a swap and each keep their slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FormUpgradeSwap;

impl Layered for FormUpgradeSwap {
    type Layer = DomainLayer;
}

impl Specification<UnitPair<'_>> for FormUpgradeSwap {
    fn is_satisfied_by(&self, candidate: &UnitPair<'_>) -> bool {
        for swap in UNIT_UPGRADE_SWAPS {
            let from_object_id = swap.from_unit_id();
            let to_object_id = swap.to_unit_id();
            let from_id = from_object_id.value();
            let to_id = to_object_id.value();
            let forward = from_id.eq_ignore_ascii_case(candidate.first_unit_id)
                && to_id.eq_ignore_ascii_case(candidate.second_unit_id);
            let backward = from_id.eq_ignore_ascii_case(candidate.second_unit_id)
                && to_id.eq_ignore_ascii_case(candidate.first_unit_id);
            if forward || backward {
                return true;
            }
        }
        false
    }
}

/// The morph ability collapses back into its host unit, so it must not claim its
/// own separate command slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RevertsToHost;

impl Layered for RevertsToHost {
    type Layer = DomainLayer;
}

impl Specification<MorphAgainstHost<'_>> for RevertsToHost {
    fn is_satisfied_by(&self, candidate: &MorphAgainstHost<'_>) -> bool {
        let ability_object = candidate.database.by_id(candidate.ability_id);
        let Some(target_id) = ability_object.and_then(|object| object.ability_morph_target_id())
        else {
            return false;
        };
        if !target_id.eq_ignore_ascii_case(candidate.host_unit_id) {
            return false;
        }
        !BuildingTraits::ability_has_alt_state(candidate.ability_id)
    }
}

/// The ability only exists on the unit's rooted form and must be dropped from an
/// uprooted or otherwise non-rooted command card.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RootedOnlyAbility;

impl Layered for RootedOnlyAbility {
    type Layer = DomainLayer;
}

impl Specification<AbilityInDatabase<'_>> for RootedOnlyAbility {
    fn is_satisfied_by(&self, candidate: &AbilityInDatabase<'_>) -> bool {
        if ROOTED_ONLY_ABILITY_IDS
            .iter()
            .any(|id| id.value().eq_ignore_ascii_case(candidate.ability_id))
        {
            return true;
        }
        let ability_object = candidate.database.by_id(candidate.ability_id);
        let Some(ability_code) = ability_object.and_then(|object| object.ability_code()) else {
            return false;
        };
        ROOTED_ONLY_ABILITY_CODES
            .iter()
            .any(|code| code.value().eq_ignore_ascii_case(ability_code))
    }
}

#[cfg(test)]
mod ddd_marker_tests {
    use super::AbilityInDatabase;
    use super::AbilityOnUnit;
    use super::FormUpgradeSwap;
    use super::HiddenAbility;
    use super::MorphAgainstHost;
    use super::RevertsToHost;
    use super::RootedOnlyAbility;
    use super::UnitPair;
    use crate::ddd_conformance::assert_specification;
    use ddd::Specification;
    use warcraft_database::WARCRAFT_DATABASE;

    #[test]
    fn ability_rules_are_specifications() {
        assert_specification::<AbilityOnUnit<'_>, HiddenAbility>();
        assert_specification::<UnitPair<'_>, FormUpgradeSwap>();
        assert_specification::<MorphAgainstHost<'_>, RevertsToHost>();
        assert_specification::<AbilityInDatabase<'_>, RootedOnlyAbility>();
    }

    #[test]
    fn hidden_ability_matches_only_the_listed_pairs() {
        let hidden = HiddenAbility;
        let phoenix_fire = AbilityOnUnit::new("hphx", "Apxf");
        let unrelated = AbilityOnUnit::new("hpea", "Apxf");
        assert!(hidden.is_satisfied_by(&phoenix_fire));
        assert!(!hidden.is_satisfied_by(&unrelated));
    }

    #[test]
    fn rooted_only_ability_flags_a_known_rooted_ability() {
        let rooted_only = RootedOnlyAbility;
        let rooted = AbilityInDatabase::new(&WARCRAFT_DATABASE, "Anei");
        let regular = AbilityInDatabase::new(&WARCRAFT_DATABASE, "AHbz");
        assert!(rooted_only.is_satisfied_by(&rooted));
        assert!(!rooted_only.is_satisfied_by(&regular));
    }
}
