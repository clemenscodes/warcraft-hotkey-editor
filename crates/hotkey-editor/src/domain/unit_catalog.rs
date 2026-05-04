use warcraft_api::{Race, UnitKind, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::domain::unit_kind::UnitKindHelpers;
use crate::domain::unit_mode::UnitMode;

pub(crate) struct CatalogEntry {
    pub(crate) unit_id: String,
    pub(crate) warcraft_object: &'static WarcraftObject,
    pub(crate) unit_kind: UnitKind,
}

pub(crate) struct UnitCatalog;

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
    pub(crate) fn entries_for(
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
                    // Check all names, not just the first — some units have alternate display names.
                    let names_lower: String = warcraft_object
                        .names()
                        .iter()
                        .map(|n| n.to_ascii_lowercase())
                        .collect::<Vec<_>>()
                        .join(" ");
                    // Match if the name/id contains the query OR the query contains the name/id as
                    // a word — the latter lets "peasant worker" find a unit named just "Peasant".
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
            left_priority.cmp(&right_priority).then_with(|| {
                let left_name = left_entry
                    .warcraft_object
                    .names()
                    .first()
                    .copied()
                    .unwrap_or("");
                let right_name = right_entry
                    .warcraft_object
                    .names()
                    .first()
                    .copied()
                    .unwrap_or("");
                left_name
                    .cmp(right_name)
                    .then_with(|| left_entry.unit_id.cmp(&right_entry.unit_id))
            })
        });

        entries
    }
}
