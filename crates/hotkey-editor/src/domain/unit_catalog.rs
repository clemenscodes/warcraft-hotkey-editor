use std::collections::HashSet;

use warcraft_api::{Race, UnitKind, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

use crate::domain::object_lookup::ObjectLookup;
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
    /// Walks `WARCRAFT_DATABASE`, applies race/mode/kind/search filters, sorts
    /// by category priority then display name, and dedupes by `(kind, name)`
    /// — but keeps both forms of a same-name pair when one morphs into the
    /// other (e.g. Druid of the Claw → Bear Form). Internal-id variants that
    /// don't morph between each other (Tinker `Nrob`/`Ntin`, Carrion Beetle
    /// levels `ucs1`/`ucs2`/`ucs3`, Alchemist `Nal2`/`Nal3`/`Nalc`/`Nalm`)
    /// collapse to a single entry.
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
                left_name.cmp(right_name)
            })
        });

        // Compute the set of unit ids that share a (kind, name) with another
        // unit AND are linked to it by a morph ability — those need to survive
        // dedup as distinct entries. Everything else collapses to the first
        // occurrence of its (kind, name).
        let morph_linked_ids: HashSet<&'static str> = compute_morph_linked_ids(&entries);

        let mut seen_keys: HashSet<(UnitKind, &'static str)> = HashSet::new();
        entries.retain(|entry| {
            let name = entry.warcraft_object.names().first().copied().unwrap_or("");
            if morph_linked_ids.contains(entry.unit_id.as_str()) {
                return true;
            }
            seen_keys.insert((entry.unit_kind, name))
        });

        entries
    }
}

/// Returns the set of unit ids that share a `(kind, name)` with at least one
/// other unit in `entries` AND are connected to it by a morph ability (in
/// either direction). These survive `(kind, name)` dedup so a player browsing
/// "Druid of the Claw" sees both the caster and bear forms, while Tinker
/// `Nrob`/`Ntin` (no morph link, just two duplicate IDs) collapses.
fn compute_morph_linked_ids(entries: &[CatalogEntry]) -> HashSet<&'static str> {
    use std::collections::HashMap;

    // Bucket entries by (kind, name); only buckets with >= 2 units can produce
    // intra-bucket morph links worth preserving.
    let mut name_buckets: HashMap<(UnitKind, &str), Vec<&CatalogEntry>> = HashMap::new();
    for entry in entries {
        let name = entry.warcraft_object.names().first().copied().unwrap_or("");
        if name.is_empty() {
            continue;
        }
        name_buckets.entry((entry.unit_kind, name)).or_default().push(entry);
    }

    let mut linked: HashSet<&'static str> = HashSet::new();
    for bucket in name_buckets.values() {
        if bucket.len() < 2 {
            continue;
        }
        let bucket_ids: HashSet<&str> =
            bucket.iter().map(|entry| entry.unit_id.as_str()).collect();

        for entry in bucket {
            let WarcraftObjectMeta::Unit(unit_meta) = entry.warcraft_object.meta() else {
                continue;
            };
            for ability in unit_meta.abilities().iter().chain(unit_meta.hero_abilities().iter()) {
                let Some(target_id) = ObjectLookup::morph_target_unit(ability.value()) else {
                    continue;
                };
                if target_id.eq_ignore_ascii_case(&entry.unit_id) {
                    // Self-morph (alt-state toggle like Burrow/Defend) — not a
                    // form link to another bucket member.
                    continue;
                }
                if !bucket_ids.iter().any(|id| id.eq_ignore_ascii_case(target_id)) {
                    continue;
                }
                // Promote the source unit's id and the target unit's id to
                // 'static via the database — both need to survive dedup.
                if let Some(source_static_id) = static_unit_id(&entry.unit_id) {
                    linked.insert(source_static_id);
                }
                if let Some(target_static_id) = static_unit_id(target_id) {
                    linked.insert(target_static_id);
                }
            }
        }
    }
    linked
}

fn static_unit_id(unit_id: &str) -> Option<&'static str> {
    WARCRAFT_DATABASE
        .iter()
        .find(|(object_id, _)| object_id.value().eq_ignore_ascii_case(unit_id))
        .map(|(object_id, _)| object_id.value())
}
