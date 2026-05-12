use warcraft_api::{Race, UnitKind, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};

use crate::WARCRAFT_DATABASE;
use crate::unit_kind::UnitKindHelpers;
use crate::unit_mode::UnitMode;

fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut haystack_chars = haystack.chars();
    'outer: for needle_char in needle.chars() {
        loop {
            match haystack_chars.next() {
                Some(haystack_char) if haystack_char == needle_char => continue 'outer,
                Some(_) => continue,
                None => return false,
            }
        }
    }
    true
}

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
        race_filter: Option<Race>,
        mode_filter: Option<UnitMode>,
        kind_filter: Option<UnitKind>,
        search_query: Option<&str>,
    ) -> Vec<CatalogEntry> {
        let lowercase_query = search_query
            .map(|raw_query| raw_query.trim_start().to_ascii_lowercase())
            .filter(|trimmed| !trimmed.trim().is_empty());

        // Each candidate is tagged: fuzzy_only=true when it matched only via
        // subsequence. If any direct match exists we suppress all fuzzy-only
        // hits, so "water" shows Water Elemental without Draenei Watcher noise,
        // while "ftma" (no direct hits) still falls through to fuzzy.
        struct Candidate {
            entry: CatalogEntry,
            fuzzy_only: bool,
        }

        let candidates: Vec<Candidate> = WARCRAFT_DATABASE
            .iter()
            .filter_map(|(object_id, warcraft_object)| {
                if warcraft_object.kind() != WarcraftObjectKind::Unit {
                    return None;
                }
                if let Some(race) = race_filter
                    && warcraft_object.race() != Some(race)
                {
                    return None;
                }
                let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                    return None;
                };
                let passes_mode = match mode_filter {
                    Some(mode) => UnitKindHelpers::passes_filter(mode, unit_meta),
                    None => {
                        UnitKindHelpers::passes_filter(UnitMode::Melee, unit_meta)
                            || UnitKindHelpers::passes_filter(UnitMode::Campaign, unit_meta)
                    }
                };
                if !passes_mode {
                    return None;
                }
                let effective_kind = UnitKindHelpers::effective_kind(unit_meta);
                if let Some(required_kind) = kind_filter
                    && effective_kind != required_kind
                {
                    return None;
                }
                let unit_id_string = object_id.value().to_string();
                let fuzzy_only = if let Some(query) = lowercase_query.as_deref() {
                    let id_lower = unit_id_string.to_ascii_lowercase();
                    // Check all names — some units have alternate display names.
                    let names_lower: String = warcraft_object
                        .names()
                        .iter()
                        .map(|name| name.to_ascii_lowercase())
                        .collect::<Vec<_>>()
                        .join(" ");
                    // Direct: name/id contains the query, or a query token (whole
                    // word, ≥3 chars) exactly matches a name word.
                    let matches_direct = names_lower.contains(query)
                        || id_lower.contains(query)
                        || query.contains(id_lower.as_str())
                        || query
                            .split_whitespace()
                            .filter(|token| token.len() >= 3)
                            .any(|token| {
                                names_lower
                                    .split_whitespace()
                                    .any(|name_word| name_word == token)
                            });
                    // Fuzzy fallback: every char in the query appears in order in
                    // the name. Only surfaced when no direct match exists anywhere.
                    let matches_fuzzy = is_subsequence(query, &names_lower);
                    if !matches_direct && !matches_fuzzy {
                        return None;
                    }
                    !matches_direct
                } else {
                    false
                };
                let display_name = warcraft_object.names().first().copied().unwrap_or("");
                if display_name.is_empty() {
                    return None;
                }
                let entry = CatalogEntry {
                    unit_id: unit_id_string,
                    warcraft_object,
                    unit_kind: effective_kind,
                };
                Some(Candidate { entry, fuzzy_only })
            })
            .collect();

        let has_direct_match = candidates.iter().any(|candidate| !candidate.fuzzy_only);
        let mut entries: Vec<CatalogEntry> = candidates
            .into_iter()
            .filter(|candidate| !has_direct_match || !candidate.fuzzy_only)
            .map(|candidate| candidate.entry)
            .collect();

        let is_search = mode_filter.is_none();
        entries.sort_by(|left_entry, right_entry| {
            let left_object = left_entry.warcraft_object;
            let right_object = right_entry.warcraft_object;
            let left_name = left_object.names().first().copied().unwrap_or("");
            let right_name = right_object.names().first().copied().unwrap_or("");
            let left_priority = if is_search {
                let left_campaign = match left_object.meta() {
                    WarcraftObjectMeta::Unit(unit_meta) => unit_meta.is_campaign(),
                    _ => false,
                };
                let left_kind = left_entry.unit_kind;
                UnitKindHelpers::search_sort_priority(left_kind, left_campaign)
            } else {
                UnitKindHelpers::category_priority(left_entry.unit_kind)
            };
            let right_priority = if is_search {
                let right_campaign = match right_object.meta() {
                    WarcraftObjectMeta::Unit(unit_meta) => unit_meta.is_campaign(),
                    _ => false,
                };
                let right_kind = right_entry.unit_kind;
                UnitKindHelpers::search_sort_priority(right_kind, right_campaign)
            } else {
                UnitKindHelpers::category_priority(right_entry.unit_kind)
            };
            left_priority
                .cmp(&right_priority)
                .then_with(|| left_name.cmp(right_name))
                .then_with(|| left_entry.unit_id.cmp(&right_entry.unit_id))
        });

        entries
    }
}
