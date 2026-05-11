use warcraft_api::{Race, UnitKind, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};

use crate::WARCRAFT_DATABASE;
use crate::unit_kind::UnitKindHelpers;
use crate::unit_mode::UnitMode;

pub struct CatalogEntry {
    unit_id: String,
    warcraft_object: &'static WarcraftObject,
    unit_kind: UnitKind,
}

impl CatalogEntry {
    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn warcraft_object(&self) -> &'static WarcraftObject {
        self.warcraft_object
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }
}

pub struct UnitCatalog;

impl UnitCatalog {
    /// The single source of truth for "which units belong in a list view".
    /// Walks `WARCRAFT_DATABASE`, applies race/mode/kind/search filters, and
    /// sorts by category priority then display name. Does *not* dedupe by
    /// name — same-name internal-id variants (Demon Hunter `Eevi`/`Eevm`/
    /// `Eidm`/`Eill`/`Eilm`, Alchemist `Nal2`/`Nal3`/`Nalc`/`Nalm`, Tinker
    /// `Ntin`/`Nrob`, Druid of the Claw `edoc`/`edcm`, Carrion Beetle
    /// `ucs1`/`ucs2`/`ucs3`) all surface as distinct entries with their unit
    /// id visible. The game ships these IDs deliberately (campaign variants,
    /// metamorphosis forms, level-summon variants) and any heuristic that
    /// tries to pick a canonical one is going to be wrong somewhere.
    pub fn entries_for(
        race: Race,
        mode: UnitMode,
        kind_filter: Option<UnitKind>,
        search_query: Option<&str>,
    ) -> Vec<CatalogEntry> {
        let lowercase_query = search_query
            .map(|raw_query| raw_query.trim().to_ascii_lowercase())
            .filter(|trimmed| !trimmed.is_empty());

        let mut entries: Vec<CatalogEntry> = WARCRAFT_DATABASE
            .iter()
            .filter_map(|(object_id, warcraft_object)| {
                if warcraft_object.kind() != WarcraftObjectKind::Unit {
                    return None;
                }
                if warcraft_object.race() != Some(race) {
                    return None;
                }
                let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                    return None;
                };
                if !UnitKindHelpers::passes_filter(mode, unit_meta) {
                    return None;
                }
                let effective_kind = UnitKindHelpers::effective_kind(unit_meta);
                if let Some(required_kind) = kind_filter
                    && effective_kind != required_kind
                {
                    return None;
                }
                let unit_id_string = object_id.value().to_string();
                if let Some(query) = lowercase_query.as_deref() {
                    let id_lower = unit_id_string.to_ascii_lowercase();
                    // Check all names — some units have alternate display names.
                    let names_lower: String = warcraft_object
                        .names()
                        .iter()
                        .map(|name| name.to_ascii_lowercase())
                        .collect::<Vec<_>>()
                        .join(" ");
                    // Match if name/id contains the query OR query contains name/id as a word —
                    // the latter lets "peasant worker" find a unit named just "Peasant".
                    let matches_id = id_lower.contains(query) || query.contains(id_lower.as_str());
                    let matches_name = names_lower.contains(query)
                        || names_lower
                            .split_whitespace()
                            .any(|word| query.contains(word) && word.len() >= 3);
                    if !matches_id && !matches_name {
                        return None;
                    }
                }
                let display_name = warcraft_object.names().first().copied().unwrap_or("");
                if display_name.is_empty() {
                    return None;
                }
                Some(CatalogEntry {
                    unit_id: unit_id_string,
                    warcraft_object,
                    unit_kind: effective_kind,
                })
            })
            .collect();

        entries.sort_by(|left_entry, right_entry| {
            let left_priority = UnitKindHelpers::category_priority(left_entry.unit_kind);
            let right_priority = UnitKindHelpers::category_priority(right_entry.unit_kind);
            let left_object = left_entry.warcraft_object;
            let right_object = right_entry.warcraft_object;
            let left_name = left_object.names().first().copied().unwrap_or("");
            let right_name = right_object.names().first().copied().unwrap_or("");
            left_priority
                .cmp(&right_priority)
                .then_with(|| left_name.cmp(right_name))
                .then_with(|| left_entry.unit_id.cmp(&right_entry.unit_id))
        });

        entries
    }
}
