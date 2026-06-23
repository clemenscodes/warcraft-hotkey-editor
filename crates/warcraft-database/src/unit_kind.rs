use warcraft_api::{Race, UnitKind, UnitMeta};

use crate::unit_catalog::{CatalogVisibility, SearchField, UnitCatalog};
use crate::unit_mode::UnitMode;

pub struct UnitKindHelpers;

impl UnitKindHelpers {
    pub fn effective_kind(unit_meta: &UnitMeta) -> UnitKind {
        if unit_meta.is_special() && unit_meta.unit_kind() == UnitKind::Worker {
            return UnitKind::Soldier;
        }
        unit_meta.unit_kind()
    }

    pub fn category_label(unit_kind: UnitKind) -> &'static str {
        match unit_kind {
            UnitKind::Hero => "Heroes",
            UnitKind::Soldier => "Units",
            UnitKind::Worker => "Workers",
            UnitKind::Building => "Buildings",
        }
    }

    pub fn category_priority(unit_kind: UnitKind) -> u8 {
        match unit_kind {
            UnitKind::Hero => 0,
            UnitKind::Building => 1,
            UnitKind::Worker => 2,
            UnitKind::Soldier => 3,
        }
    }

    pub fn search_sort_priority(unit_kind: UnitKind, is_campaign: bool) -> u8 {
        match (is_campaign, unit_kind) {
            (false, UnitKind::Hero) => 0,
            (false, UnitKind::Building) => 1,
            (false, UnitKind::Worker) => 2,
            (false, UnitKind::Soldier) => 3,
            (true, UnitKind::Hero) => 4,
            (true, UnitKind::Building) => 5,
            (true, UnitKind::Worker) => 6,
            (true, UnitKind::Soldier) => 7,
        }
    }

    /// `inEditor=1` in `unitui.slk` is Blizzard's flag for "show in the
    /// World Editor's unit picker". Tavern mercenaries with bindable
    /// abilities — Barbed Arachnathid (merc) `nanm` carrying Burrow,
    /// Watcher Ward `nwad`, Entangled Gold Mine `egol` — ship with
    /// `inEditor=0` because they aren't placed in the world editor, but
    /// they still need to surface in the hotkey editor's catalog.
    /// The downstream `has_visible_ability || has_production` check in
    /// `UnitCatalog::entries_for` still drops the placeholder rows
    /// (Barbed Arachnathid `nanb`, Crystal Arachnathid `nanc`, Warrior
    /// Arachnathid `nanw`) that have neither.
    pub fn passes_filter(mode: UnitMode, unit_meta: &UnitMeta) -> bool {
        if unit_meta.is_hidden_in_editor() {
            return false;
        }
        match mode {
            UnitMode::Melee => !unit_meta.is_campaign(),
            UnitMode::Campaign => unit_meta.is_campaign(),
        }
    }

    pub fn default_unit_id_for(race: Race, mode: UnitMode) -> Option<String> {
        let curated = CatalogVisibility::default();
        let first_entry = UnitCatalog::entries_for(
            Some(race),
            Some(mode),
            None,
            None,
            SearchField::UnitName,
            curated,
        )
        .into_iter()
        .next();
        first_entry.map(|entry| entry.unit_id().to_owned())
    }
}
