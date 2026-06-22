use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use warcraft_api::{Race, UnitKind, WarcraftObject, WarcraftObjectKind, WarcraftObjectMeta};

use crate::WARCRAFT_DATABASE;
use crate::unit_kind::UnitKindHelpers;
use crate::unit_mode::UnitMode;
use crate::variant_groups::VariantUnits;

/// What a search query is matched against. The sidebar exposes this as a
/// toggle: search units by their own name/id (default), or by the abilities
/// they carry — the latter answers "which units have this ability?" (issue #30,
/// the collision-resolution lookup).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchField {
    #[default]
    UnitName,
    Ability,
}

/// Unit ids that some shop offers for sale (anything appearing in a
/// `sell_units` list). A purchasable unit such as the Ogre Mauler `nogm`
/// carries no abilities and no production of its own — it only ever
/// appears as a buy-button on a Mercenary Camp — so the catalog needs
/// this reverse lookup to keep it instead of mistaking it for a dead
/// placeholder. Built once; the database is static.
static SOLD_UNIT_IDS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    let mut sold_unit_ids = HashSet::new();
    for (_seller_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            continue;
        };
        for sold_unit_id in unit_meta.sell_units() {
            let sold_unit_value = sold_unit_id.value();
            sold_unit_ids.insert(sold_unit_value);
        }
    }
    sold_unit_ids
});

/// Per-unit lowercase search text built from the names and ids of the abilities
/// the unit carries on a command button (`abilities()` + `hero_abilities()`,
/// kept only when the ability has a `default_button_position` — the same
/// "button-positioned" notion as the catalog's `has_visible_ability` gate, so a
/// unit that produces a haystack always survives that gate). Lets the search
/// match a unit by the abilities it carries (issue #30). The button-positioned
/// list is a deliberate approximation of the exact command card, which lives in
/// `warcraft-keybinds` and cannot be reached from here. Built once; the database
/// is static, so per-keystroke matching is one lookup plus a `contains`.
static UNIT_ABILITY_HAYSTACK: LazyLock<HashMap<&'static str, String>> = LazyLock::new(|| {
    let mut unit_ability_haystack: HashMap<&'static str, String> = HashMap::new();
    for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            continue;
        };
        let ability_ids = unit_meta
            .abilities()
            .iter()
            .chain(unit_meta.hero_abilities().iter());
        let mut haystack = String::new();
        for ability_id in ability_ids {
            let ability_id_value = ability_id.value();
            let Some(ability_object) = WARCRAFT_DATABASE.by_id(ability_id_value) else {
                continue;
            };
            let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
                continue;
            };
            if ability_meta.default_button_position().is_none() {
                continue;
            }
            for ability_name in ability_object.names() {
                let ability_name_lower = ability_name.to_ascii_lowercase();
                haystack.push(' ');
                haystack.push_str(&ability_name_lower);
            }
            let ability_id_lower = ability_id_value.to_ascii_lowercase();
            haystack.push(' ');
            haystack.push_str(&ability_id_lower);
        }
        if !haystack.is_empty() {
            haystack.push(' ');
            let unit_id_value = object_id.value();
            unit_ability_haystack.insert(unit_id_value, haystack);
        }
    }
    unit_ability_haystack
});

/// Per-building sort key encoding in-game availability: `chain_base_gold_cost *
/// 1000 + upgrade_depth`. A building "upgrades into" another when the target
/// building's id appears in its `researches` list (the `R…`-prefixed research
/// ids are tech, not buildings). Walking up to the chain base gives every member
/// of a main-building chain (Town Hall → Keep → Castle) the same base cost, so
/// they group together and order by depth; whole chains then order by the base's
/// gold cost. Built once; the database is static.
static BUILDING_RANKS: LazyLock<HashMap<&'static str, u32>> = LazyLock::new(|| {
    let mut gold_cost: HashMap<&'static str, u32> = HashMap::new();
    let mut upgrade_parent: HashMap<&'static str, &'static str> = HashMap::new();
    for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            continue;
        };
        if unit_meta.unit_kind() != UnitKind::Building {
            continue;
        }
        let source_id = object_id.value();
        gold_cost.insert(source_id, unit_meta.gold_cost());
        for research_id in unit_meta.researches() {
            let target_id = research_id.value();
            let target_is_building = WARCRAFT_DATABASE.by_id(target_id).is_some_and(|object| {
                matches!(
                    object.meta(),
                    WarcraftObjectMeta::Unit(target_meta)
                        if target_meta.unit_kind() == UnitKind::Building
                )
            });
            if target_is_building {
                upgrade_parent.insert(target_id, source_id);
            }
        }
    }
    let mut ranks: HashMap<&'static str, u32> = HashMap::new();
    for building_id in gold_cost.keys().copied() {
        let mut base_id = building_id;
        let mut upgrade_depth: u32 = 0;
        while let Some(parent_id) = upgrade_parent.get(base_id).copied() {
            base_id = parent_id;
            upgrade_depth += 1;
            if upgrade_depth > 16 {
                break;
            }
        }
        let base_gold_cost = gold_cost.get(base_id).copied().unwrap_or(0);
        let key = base_gold_cost
            .saturating_mul(1000)
            .saturating_add(upgrade_depth);
        ranks.insert(building_id, key);
    }
    ranks
});

/// The within-category sort key (lower lists first). Buildings use the
/// upgrade-chain + gold-cost rank; everything else uses the unit's tech tier.
fn availability_key(entry: &CatalogEntry) -> u32 {
    if entry.unit_kind == UnitKind::Building {
        return BUILDING_RANKS
            .get(entry.unit_id.as_str())
            .copied()
            .unwrap_or(0);
    }
    match entry.warcraft_object.meta() {
        WarcraftObjectMeta::Unit(unit_meta) => unit_meta.level(),
        _ => 0,
    }
}

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

    /// Builds the entry for a canonical variant unit looked up fresh from the
    /// database. Used when a weaker variant collapses onto its strongest
    /// sibling, which may not itself have matched the active filter or query.
    fn canonical_entry(unit_id: &'static str) -> Option<Self> {
        let warcraft_object = WARCRAFT_DATABASE.by_id(unit_id)?;
        let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
            return None;
        };
        let effective_kind = UnitKindHelpers::effective_kind(unit_meta);
        let unit_id_string = unit_id.to_string();
        let entry = Self {
            unit_id: unit_id_string,
            warcraft_object,
            unit_kind: effective_kind,
        };
        Some(entry)
    }
}

pub struct UnitCatalog;

impl UnitCatalog {
    /// The single source of truth for "which units belong in a list view".
    /// Walks `WARCRAFT_DATABASE`, applies race/mode/kind/search filters, and
    /// sorts by category priority then display name.
    ///
    /// Variant groups collapse to a single entry (see `VariantUnits`): leveled
    /// summon tiers (Feral Spirit `osw1`/`osw2`/`osw3`) fold into their
    /// strongest member, upgrade-swaps (Headhunter `ohun` → Berserker `otbk`)
    /// into the upgraded unit, and a hero's duplicate campaign/form ids
    /// (Alchemist `Nal2`/`Nal3`/`Nalm`, Tinker `Nrob`) into the produced
    /// (trained/sold) hero. Tiers and swaps come straight from the game data;
    /// heroes group by shared name with the produced id as the data-driven
    /// canonical. A query that matches a weaker variant (searching "Headhunter")
    /// surfaces the canonical (Berserker) rather than vanishing.
    pub fn entries_for(
        race_filter: Option<Race>,
        mode_filter: Option<UnitMode>,
        kind_filter: Option<UnitKind>,
        search_query: Option<&str>,
        search_field: SearchField,
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

        struct QueryMatch {
            is_direct: bool,
            is_fuzzy: bool,
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
                let has_production = !unit_meta.trains().is_empty()
                    || !unit_meta.builds().is_empty()
                    || !unit_meta.researches().is_empty()
                    || !unit_meta.sell_items().is_empty()
                    || !unit_meta.sell_units().is_empty();
                let mut all_abilities = unit_meta
                    .abilities()
                    .iter()
                    .chain(unit_meta.hero_abilities().iter());
                let has_visible_ability = all_abilities.any(|ability_id| {
                    let ability_id_str = ability_id.value();
                    let Some(ability_object) = WARCRAFT_DATABASE.by_id(ability_id_str) else {
                        return false;
                    };
                    let WarcraftObjectMeta::Ability(ability_meta) = ability_object.meta() else {
                        return false;
                    };
                    let button_position = ability_meta.default_button_position();
                    button_position.is_some()
                });
                // Purchasable units (e.g. mercenaries) carry no abilities
                // and no production of their own — they are only ever sold
                // at a shop, which renders their buy-button and exposes a
                // rebindable hotkey. Keep them; ability-less placeholders
                // that no shop sells stay filtered out.
                let object_id_value = object_id.value();
                let is_purchasable = SOLD_UNIT_IDS.contains(object_id_value);
                if !has_production && !has_visible_ability && !is_purchasable {
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
                    let query_match = match search_field {
                        SearchField::UnitName => {
                            let id_lower = unit_id_string.to_ascii_lowercase();
                            // Check all names — some units have alternate display
                            // names.
                            let names_lower: String = warcraft_object
                                .names()
                                .iter()
                                .map(|name| name.to_ascii_lowercase())
                                .collect::<Vec<_>>()
                                .join(" ");
                            // Direct: name/id contains the query, or a query token
                            // (whole word, ≥3 chars) exactly matches a name word.
                            let is_direct = names_lower.contains(query)
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
                            // Fuzzy fallback: every char in the query appears in
                            // order in the name. Only surfaced when no direct match
                            // exists anywhere.
                            let is_fuzzy = is_subsequence(query, &names_lower);
                            QueryMatch {
                                is_direct,
                                is_fuzzy,
                            }
                        }
                        SearchField::Ability => {
                            // Match the names/ids of the unit's button-positioned
                            // abilities. No fuzzy fallback: subsequence over the
                            // concatenated haystack would match almost anything.
                            let ability_haystack = UNIT_ABILITY_HAYSTACK
                                .get(unit_id_string.as_str())
                                .map(String::as_str)
                                .unwrap_or("");
                            let is_direct = ability_haystack.contains(query)
                                || query
                                    .split_whitespace()
                                    .filter(|token| token.len() >= 3)
                                    .any(|token| {
                                        ability_haystack
                                            .split_whitespace()
                                            .any(|ability_word| ability_word == token)
                                    });
                            QueryMatch {
                                is_direct,
                                is_fuzzy: false,
                            }
                        }
                    };
                    if !query_match.is_direct && !query_match.is_fuzzy {
                        return None;
                    }
                    !query_match.is_direct
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
        let visible_entries = candidates
            .into_iter()
            .filter(|candidate| !has_direct_match || !candidate.fuzzy_only)
            .map(|candidate| candidate.entry);

        // Collapse each variant group to its canonical (strongest) member so a
        // group surfaces as one entry. A weaker variant is replaced by its
        // canonical — looked up fresh, since the canonical may not have matched
        // the query itself — and deduped so the canonical appears once even
        // when several siblings (or the canonical itself) reach this point.
        let mut seen_display_ids: HashSet<String> = HashSet::new();
        let mut entries: Vec<CatalogEntry> = Vec::new();
        for entry in visible_entries {
            let canonical_lookup = VariantUnits::canonical_for(&entry.unit_id);
            if let Some(canonical) = canonical_lookup
                && entry.unit_id != canonical
            {
                let canonical_id = canonical.to_string();
                if !seen_display_ids.insert(canonical_id) {
                    continue;
                }
                if let Some(canonical_entry) = CatalogEntry::canonical_entry(canonical) {
                    entries.push(canonical_entry);
                }
            } else {
                let entry_id = entry.unit_id.clone();
                if !seen_display_ids.insert(entry_id) {
                    continue;
                }
                entries.push(entry);
            }
        }

        let is_search = mode_filter.is_none();
        entries.sort_by(|left_entry, right_entry| {
            let left_object = left_entry.warcraft_object;
            let right_object = right_entry.warcraft_object;
            let left_name = left_object.names().first().copied().unwrap_or("");
            let right_name = right_object.names().first().copied().unwrap_or("");
            // Within a category, order by in-game availability rather than
            // alphabetically: units by their tech tier (`level`), buildings by
            // the upgrade-chain + gold-cost rank (see `availability_key`).
            let left_key = availability_key(left_entry);
            let right_key = availability_key(right_entry);
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
                .then_with(|| left_key.cmp(&right_key))
                .then_with(|| left_name.cmp(right_name))
                .then_with(|| left_entry.unit_id.cmp(&right_entry.unit_id))
        });

        entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn melee_ids(race: Race) -> Vec<String> {
        UnitCatalog::entries_for(
            Some(race),
            Some(UnitMode::Melee),
            None,
            None,
            SearchField::UnitName,
        )
        .iter()
        .map(|entry| entry.unit_id().to_string())
        .collect()
    }

    /// An upgrade-swap browses as the upgraded unit only: Headhunter `ohun`
    /// folds into Berserker `otbk`, Siege Engine `hmtt` into the barrage-capable
    /// `hrtt`. The weaker form never appears as its own list entry.
    #[test]
    fn browse_collapses_upgrade_swaps_to_upgraded_unit() {
        let orc_ids = melee_ids(Race::Orc);
        assert!(orc_ids.iter().any(|id| id == "otbk"), "Berserker must list");
        assert!(
            !orc_ids.iter().any(|id| id == "ohun"),
            "Headhunter must be hidden"
        );

        let human_ids = melee_ids(Race::Human);
        assert!(
            human_ids.iter().any(|id| id == "hrtt"),
            "Barrage Siege Engine must list"
        );
        assert!(
            !human_ids.iter().any(|id| id == "hmtt"),
            "base Siege Engine must be hidden"
        );
    }

    /// Leveled summon tiers browse as their strongest tier only. Feral Spirit
    /// wolves `osw1`/`osw2`/`osw3` collapse to `osw3` (the user's reference
    /// case); the Neutral Spiderlings `osp1..osp4` collapse to `osp4`.
    #[test]
    fn browse_collapses_summon_tiers_to_strongest() {
        let orc_ids = melee_ids(Race::Orc);
        assert!(
            orc_ids.iter().any(|id| id == "osw3"),
            "strongest wolf must list"
        );
        assert!(
            !orc_ids.iter().any(|id| id == "osw1" || id == "osw2"),
            "weaker wolves hidden"
        );
        assert!(
            orc_ids.iter().any(|id| id == "osp4"),
            "strongest spiderling must list"
        );
        assert!(
            !orc_ids
                .iter()
                .any(|id| id == "osp1" || id == "osp2" || id == "osp3"),
            "weaker spiderlings hidden",
        );
    }

    /// Heroes with duplicate campaign/form ids list once, as the produced hero:
    /// the neutral Alchemist shows `Nalc` (not `Nal2`/`Nal3`/`Nalm`) and the
    /// Tinker shows `Ntin` (not `Nrob`); the Human Paladin shows `Hpal` (not its
    /// campaign variants like `Huth`).
    #[test]
    fn heroes_collapse_to_a_single_catalog_entry() {
        let neutral_ids = melee_ids(Race::Neutral);
        assert!(
            neutral_ids.iter().any(|id| id == "Nalc"),
            "Alchemist Nalc must list"
        );
        assert!(
            !neutral_ids
                .iter()
                .any(|id| id == "Nal2" || id == "Nal3" || id == "Nalm"),
            "Alchemist variants must be hidden",
        );
        assert!(
            neutral_ids.iter().any(|id| id == "Ntin"),
            "Tinker Ntin must list"
        );
        assert!(
            !neutral_ids.iter().any(|id| id == "Nrob"),
            "Tinker Robo form must be hidden"
        );

        let human_ids = melee_ids(Race::Human);
        assert!(
            human_ids.iter().any(|id| id == "Hpal"),
            "Paladin Hpal must list"
        );
        assert!(
            !human_ids.iter().any(|id| id == "Huth"),
            "Paladin variant Huth must be hidden"
        );
    }

    /// A query that matches a weaker variant which carries its own card must
    /// surface the canonical rather than the weaker form: searching the
    /// Spiderling tier `osp1` surfaces `osp4`, never `osp1` itself.
    #[test]
    fn search_for_weaker_variant_surfaces_canonical() {
        let entries =
            UnitCatalog::entries_for(None, None, None, Some("osp1"), SearchField::UnitName);
        let ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        assert!(
            ids.contains(&"osp4"),
            "searching osp1 must surface canonical osp4"
        );
        assert!(
            !ids.contains(&"osp1"),
            "the weaker variant osp1 must not appear"
        );
    }

    /// Both Burrow carriers must surface in Neutral/Melee:
    /// - `nbnb` (Burrowed Barbed Arachnathid) carries Abu5 in its
    ///   `unitabilities.slk` row and has `inEditor=1`.
    /// - `nanm` (Barbed Arachnathid merc) also carries Abu5 but ships
    ///   with `inEditor=0` — the merc form lives in tavern data, not the
    ///   World Editor's picker. The relaxed `passes_filter` in
    ///   `unit_kind.rs` lets it through anyway because the hotkey editor
    ///   needs to bind Burrow on it.
    ///
    /// Regression guard for the "missing arachnathid units" report.
    #[test]
    fn neutral_melee_includes_burrow_carriers() {
        let entries = UnitCatalog::entries_for(
            Some(Race::Neutral),
            Some(UnitMode::Melee),
            None,
            None,
            SearchField::UnitName,
        );
        let entry_ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        for required_id in ["nbnb", "nanm"] {
            assert!(
                entry_ids.contains(&required_id),
                "Neutral/Melee catalog missing {required_id} (Burrow carrier)",
            );
        }
    }

    /// Purchasable units must be findable in search. `nogm` (Ogre
    /// Mauler) has no abilities and no production — it is only ever sold
    /// at a Mercenary Camp — but it carries its own buy-button slot via
    /// `default_button_position` (column 3, row 0). The catalog must keep
    /// such units so the hotkey on their shop slot can be rebound.
    ///
    /// Regression guard for the "can't find Ogre Mauler in search" report.
    #[test]
    fn search_includes_purchasable_units() {
        let entries =
            UnitCatalog::entries_for(None, None, None, Some("Ogre Mauler"), SearchField::UnitName);
        let entry_ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        assert!(
            entry_ids.contains(&"nogm"),
            "search for 'Ogre Mauler' missing nogm (purchasable mercenary unit)",
        );
    }

    /// Within a category, units list by in-game tech tier (`unitbalance.slk`
    /// `level`), not alphabetically: Footman (level 2) < Rifleman (level 3) <
    /// Knight (level 4), even though alphabetically Knight sorts first. Covers
    /// the "order by in-game availability" requirement.
    #[test]
    fn soldiers_sort_by_in_game_availability_level() {
        let entries = UnitCatalog::entries_for(
            Some(Race::Human),
            Some(UnitMode::Melee),
            Some(UnitKind::Soldier),
            None,
            SearchField::UnitName,
        );
        let position = |unit_id: &str| {
            entries
                .iter()
                .position(|entry| entry.unit_id() == unit_id)
                .unwrap_or_else(|| panic!("Human/Melee soldiers missing {unit_id}"))
        };
        let footman = position("hfoo");
        let rifleman = position("hrif");
        let knight = position("hkni");
        assert!(
            footman < rifleman && rifleman < knight,
            "expected Footman({footman}) < Rifleman({rifleman}) < Knight({knight}) by tech tier",
        );
    }

    /// Every race's three-tier main-building chain stays grouped and in tier
    /// order (adjacent), driven purely by the upgrade graph in the data — not a
    /// human-specific rule. Human Town Hall→Keep→Castle, Orc Great Hall→
    /// Stronghold→Fortress, Night Elf Tree of Life→Ages→Eternity, Undead
    /// Necropolis→Halls of the Dead→Black Citadel.
    #[test]
    fn main_hall_upgrade_chains_group_for_all_races() {
        let chains = [
            (Race::Human, ["htow", "hkee", "hcas"]),
            (Race::Orc, ["ogre", "ostr", "ofrt"]),
            (Race::Nightelf, ["etol", "etoa", "etoe"]),
            (Race::Undead, ["unpl", "unp1", "unp2"]),
        ];
        for (race, chain) in chains {
            let entries = UnitCatalog::entries_for(
                Some(race),
                Some(UnitMode::Melee),
                Some(UnitKind::Building),
                None,
                SearchField::UnitName,
            );
            let position = |unit_id: &str| {
                entries
                    .iter()
                    .position(|entry| entry.unit_id() == unit_id)
                    .unwrap_or_else(|| panic!("{race:?} buildings missing {unit_id}"))
            };
            let tier_one = position(chain[0]);
            let tier_two = position(chain[1]);
            let tier_three = position(chain[2]);
            assert_eq!(
                tier_two,
                tier_one + 1,
                "{race:?}: {} must immediately follow {}",
                chain[1],
                chain[0],
            );
            assert_eq!(
                tier_three,
                tier_two + 1,
                "{race:?}: {} must immediately follow {}",
                chain[2],
                chain[1],
            );
        }
    }

    /// Campaign-flagged units must NOT bleed into Melee mode. After
    /// dropping the `inEditor` requirement we still rely on `is_campaign`
    /// to keep map-script-only rows like Naga / Draenei / Blood Elf out
    /// of the Melee tab.
    #[test]
    fn neutral_melee_excludes_campaign_only_units() {
        let entries = UnitCatalog::entries_for(
            Some(Race::Neutral),
            Some(UnitMode::Melee),
            None,
            None,
            SearchField::UnitName,
        );
        let entry_ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        for campaign_id in ["nmyr", "nnsw", "ndrl", "nbel"] {
            assert!(
                !entry_ids.contains(&campaign_id),
                "Neutral/Melee catalog leaked campaign unit {campaign_id}",
            );
        }
    }

    /// Issue #30: searching by ability surfaces every unit that carries it,
    /// matched by ability name *or* id. Slow `ACsw` is carried by Kobold
    /// Geomancer `nkog`, Mur'gul Snarecaster `nmsn`, and Watery Minion `nsns`
    /// (all show it on a command button); the Footman has no Slow.
    #[test]
    fn ability_search_lists_carrier_units_by_name_and_id() {
        for query in ["Slow", "ACsw"] {
            let entries =
                UnitCatalog::entries_for(None, None, None, Some(query), SearchField::Ability);
            let entry_ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
            for carrier_id in ["nkog", "nmsn", "nsns"] {
                assert!(
                    entry_ids.contains(&carrier_id),
                    "ability search '{query}' missing Slow carrier {carrier_id}",
                );
            }
            assert!(
                !entry_ids.contains(&"hfoo"),
                "ability search '{query}' wrongly included the Footman (no Slow)",
            );
        }
    }

    /// The toggle is exclusive: in unit-name mode an ability name must not pull
    /// in its carriers (no unit is named "Slow"), while a real unit name still
    /// resolves.
    #[test]
    fn unit_name_search_ignores_abilities() {
        let by_ability_name =
            UnitCatalog::entries_for(None, None, None, Some("Slow"), SearchField::UnitName);
        let ability_name_ids: Vec<&str> = by_ability_name
            .iter()
            .map(|entry| entry.unit_id())
            .collect();
        assert!(
            !ability_name_ids.contains(&"nkog"),
            "unit-name search 'Slow' must not surface ability carriers",
        );
        let by_unit_name =
            UnitCatalog::entries_for(None, None, None, Some("Footman"), SearchField::UnitName);
        let unit_name_ids: Vec<&str> = by_unit_name.iter().map(|entry| entry.unit_id()).collect();
        assert!(
            unit_name_ids.contains(&"hfoo"),
            "unit-name search 'Footman' should still find the Footman",
        );
    }
}
