use warcraft_api::{Race, UnitKind, UnitMeta};

use crate::domain::unit_catalog::UnitCatalog;
use crate::domain::unit_mode::UnitMode;

pub(crate) struct UnitKindHelpers;

impl UnitKindHelpers {
    pub(crate) fn effective_kind(unit_meta: &UnitMeta) -> UnitKind {
        if unit_meta.is_special() && unit_meta.unit_kind() == UnitKind::Worker {
            return UnitKind::Soldier;
        }
        unit_meta.unit_kind()
    }

    pub(crate) fn category_label(unit_kind: UnitKind) -> &'static str {
        match unit_kind {
            UnitKind::Hero => "Heroes",
            UnitKind::Soldier => "Units",
            UnitKind::Worker => "Workers",
            UnitKind::Building => "Buildings",
        }
    }

    pub(crate) fn category_priority(unit_kind: UnitKind) -> u8 {
        match unit_kind {
            UnitKind::Hero => 0,
            UnitKind::Soldier => 1,
            UnitKind::Worker => 2,
            UnitKind::Building => 3,
        }
    }

    pub(crate) fn passes_filter(mode: UnitMode, unit_meta: &UnitMeta) -> bool {
        if unit_meta.is_hidden_in_editor() {
            return false;
        }
        match mode {
            UnitMode::Melee => !unit_meta.is_campaign() && unit_meta.is_in_editor(),
            UnitMode::Campaign => unit_meta.is_campaign(),
        }
    }

    pub(crate) fn default_unit_id_for(race: Race, mode: UnitMode) -> Option<String> {
        UnitCatalog::entries_for(race, mode, None, None)
            .into_iter()
            .next()
            .map(|entry| entry.unit_id)
    }
}
