use std::borrow::Borrow;

use warcraft_api::{WarcraftObject, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

pub(crate) struct ObjectLookup;

impl ObjectLookup {
    pub(crate) fn by_id(needle_id: &str) -> Option<&'static WarcraftObject> {
        let database_map = WARCRAFT_DATABASE.db();
        let direct_lookup: Option<&WarcraftObject> = database_map.get(needle_id);
        if direct_lookup.is_some() {
            return direct_lookup;
        }
        for (object_id, warcraft_object) in database_map.iter() {
            let id_value: &str = object_id.borrow();
            if id_value.eq_ignore_ascii_case(needle_id) {
                return Some(warcraft_object);
            }
        }
        None
    }

    pub(crate) fn has_icon(object_id: &str) -> bool {
        let Some(warcraft_object) = Self::by_id(object_id) else {
            return false;
        };
        warcraft_object
            .icons()
            .iter()
            .any(|icon_path| !icon_path.trim().is_empty())
    }

    pub(crate) fn is_passive_ability(object_id: &str) -> bool {
        let Some(warcraft_object) = Self::by_id(object_id) else {
            return false;
        };
        warcraft_object
            .icons()
            .first()
            .copied()
            .map(|icon_path| {
                icon_path
                    .to_ascii_lowercase()
                    .starts_with("passivebuttons/")
            })
            .unwrap_or(false)
    }

    /// Game-mechanic class from `units/abilitydata.slk`'s `code` column —
    /// e.g. `Apit` for Purchase Item, `Aave` for Avenger Form. Returns
    /// `None` when the object isn't an ability or has no code recorded.
    pub(crate) fn ability_code(object_id: &str) -> Option<&'static str> {
        let warcraft_object = Self::by_id(object_id)?;
        match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.code(),
            _ => None,
        }
    }

    /// For one-way morph abilities, the unit id this ability transforms its
    /// caster into. Used to suppress the morph trigger on the unit it
    /// morphs *into* (e.g. Avenger Form on the Destroyer).
    pub(crate) fn morph_target_unit(object_id: &str) -> Option<&'static str> {
        let warcraft_object = Self::by_id(object_id)?;
        match warcraft_object.meta() {
            WarcraftObjectMeta::Ability(meta) => meta.morph_target_unit().map(|id| id.value()),
            _ => None,
        }
    }

    /// First forward morph target (i.e. morph ability whose `morph_target` is
    /// a *different* unit than the host). Returns `None` when the unit has no
    /// such ability, or when every morph ability on it self-loops (e.g. the
    /// burrowed `ucrm` whose Abur points back at `ucrm`).
    pub(crate) fn forward_morph_target_for_unit(host_unit_id: &str) -> Option<&'static str> {
        let host = Self::by_id(host_unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = host.meta() else {
            return None;
        };
        for ability in unit_meta.abilities().iter().chain(unit_meta.hero_abilities().iter()) {
            if let Some(target_id) = Self::morph_target_unit(ability.value())
                && !target_id.eq_ignore_ascii_case(host_unit_id)
            {
                return Some(target_id);
            }
        }
        None
    }

    /// True iff `ability_id` on `host_unit_id` is the caster-form copy of a
    /// mechanic that "really" lives on the host's morph target. The classic
    /// case is the Druid of the Claw: the caster form's `abilList` includes
    /// `Aroa` (code `Aroa`), but `Aroa` is the bear-form Roar — the bear form
    /// `edcm` carries `Ara2` (also code `Aroa`). The caster-form button is
    /// dead weight because in the actual game Roar is only usable after the
    /// druid morphs. Match the in-game command card by suppressing it.
    ///
    /// Mirror cases (Druid of the Talon → Storm Crow, Demon Hunter →
    /// Metamorphosis, Avenger Form on Destroyer, etc.) follow the same
    /// "same code on the morph target via a different ability id" pattern.
    pub(crate) fn ability_belongs_to_alt_form(
        ability_id: &str,
        host_unit_id: &str,
    ) -> bool {
        let Some(target_id) = Self::forward_morph_target_for_unit(host_unit_id) else {
            return false;
        };
        let Some(our_code) = Self::ability_code(ability_id) else {
            return false;
        };
        let Some(target) = Self::by_id(target_id) else {
            return false;
        };
        let WarcraftObjectMeta::Unit(target_meta) = target.meta() else {
            return false;
        };
        target_meta
            .abilities()
            .iter()
            .chain(target_meta.hero_abilities().iter())
            .any(|target_ability| {
                if target_ability.value().eq_ignore_ascii_case(ability_id) {
                    return false;
                }
                Self::ability_code(target_ability.value())
                    .map(|target_code| target_code.eq_ignore_ascii_case(our_code))
                    .unwrap_or(false)
            })
    }
}
