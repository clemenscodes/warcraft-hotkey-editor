//! Database-level regression tests for the balance-overlay extraction
//! rework. These pin the data shape that downstream UI tests depend on,
//! and fail fast (no `dx serve` boot, no browser) when the extractor
//! drifts.
//!
//! Three failure modes these tests catch:
//!
//! 1. **Balance overlay SLK reads stop unioning ability lists.**
//!    `unit_abilities.slk` and `abilitydata.slk` must be read across the
//!    base and every `_balance/<variant>.w3mod:units/` overlay so abilities
//!    that only appear in a variant (Shadow Strike on Maiden of Pain in
//!    `custom_v1`) still land on the unit, and case-mismatched references
//!    (Earth Borer's `Acvs` vs the canonical `ACvs`) merge into one entry.
//!
//! 2. **Balance overlay `.txt` reads leak into the base preset.**
//!    `unitfunc.txt`, `abilityfunc.txt`, etc. are alternative gameplay
//!    presets, not strict supersets — `_balance/melee_v0`'s Goblin
//!    Merchant publishes a different `Sellitems=` list than the base. The
//!    rules for these files explicitly *do not* match overlay paths.
//!
//! 3. **`notused_*.slk` siblings get included.** The overlay dirs ship
//!    abandoned pre-rebalance SLK tables alongside the live ones with a
//!    different column layout (`notused_unitui.slk`'s `inEditor` is
//!    column 10, the live one's is column 9). Reading them collapses good
//!    rows to bogus flags under the "first wins" merge, and the Destroyer
//!    loses four of its five abilities to a corrupted transform filter.

#[cfg(test)]
mod tests {
    use crate::{CatalogVisibility, SearchField, UnitCatalog, UnitMode, WARCRAFT_DATABASE};
    use warcraft_api::{Race, WarcraftObjectMeta};

    fn unit_abilities(unit_id: &str) -> Vec<String> {
        let object = WARCRAFT_DATABASE
            .by_id(unit_id)
            .unwrap_or_else(|| panic!("unit {unit_id} missing from database"));
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("{unit_id} is not a Unit");
        };
        unit_meta
            .abilities()
            .iter()
            .map(|ability_id| ability_id.value().to_string())
            .collect()
    }

    fn unit_trains(unit_id: &str) -> Vec<String> {
        let object = WARCRAFT_DATABASE
            .by_id(unit_id)
            .unwrap_or_else(|| panic!("unit {unit_id} missing from database"));
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("{unit_id} is not a Unit");
        };
        unit_meta
            .trains()
            .iter()
            .map(|train_id| train_id.value().to_string())
            .collect()
    }

    fn unit_researches(unit_id: &str) -> Vec<String> {
        let object = WARCRAFT_DATABASE
            .by_id(unit_id)
            .unwrap_or_else(|| panic!("unit {unit_id} missing from database"));
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("{unit_id} is not a Unit");
        };
        unit_meta
            .researches()
            .iter()
            .map(|research_id| research_id.value().to_string())
            .collect()
    }

    fn unit_sell_items(unit_id: &str) -> Vec<String> {
        let object = WARCRAFT_DATABASE
            .by_id(unit_id)
            .unwrap_or_else(|| panic!("unit {unit_id} missing from database"));
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("{unit_id} is not a Unit");
        };
        unit_meta
            .sell_items()
            .iter()
            .map(|item_id| item_id.value().to_string())
            .collect()
    }

    fn contains_ignore_case(haystack: &[String], needle: &str) -> bool {
        haystack
            .iter()
            .any(|entry| entry.eq_ignore_ascii_case(needle))
    }

    // ----- Balance-overlay SLK union ------------------------------------------

    /// Maiden of Pain (ndqp) ships with `ACdr,ACss` in the base
    /// `unitabilities.slk` and the custom_v1 overlay. Shadow Strike (ACss)
    /// was missing from the catalog before the balance-overlay matcher
    /// was added and the union merge case-folded the two casings of the
    /// reference.
    #[test]
    fn maiden_of_pain_carries_shadow_strike_and_life_drain() {
        let abilities = unit_abilities("ndqp");
        assert!(
            contains_ignore_case(&abilities, "ACss"),
            "ndqp must carry Shadow Strike (ACss); got {abilities:?}",
        );
        assert!(
            contains_ignore_case(&abilities, "ACdr"),
            "ndqp must carry Life Drain (ACdr); got {abilities:?}",
        );
    }

    /// Earth-borer (nane): the base lists `Acvs` (lowercase v), custom_v1
    /// lists `ACvs,ACss`. The union must collapse Acvs/ACvs to one entry
    /// (case-insensitive merge) AND add Shadow Strike from the overlay.
    #[test]
    fn earth_borer_carries_envenomed_and_shadow_strike() {
        let abilities = unit_abilities("nane");
        assert!(
            contains_ignore_case(&abilities, "ACvs"),
            "nane must carry Envenomed Weapons (ACvs); got {abilities:?}",
        );
        assert!(
            contains_ignore_case(&abilities, "ACss"),
            "nane must carry Shadow Strike (ACss) from custom_v1; got {abilities:?}",
        );
        let acvs_entries: Vec<&String> = abilities
            .iter()
            .filter(|ability_id| ability_id.eq_ignore_ascii_case("ACvs"))
            .collect();
        assert_eq!(
            acvs_entries.len(),
            1,
            "Envenomed Weapons must appear exactly once (case-insensitive merge); got {abilities:?}",
        );
    }

    /// Burrowed form (nbnb) carries Burrow (Abu5). The base flag
    /// `inEditor=1` means it has always been visible — guard against the
    /// catalog filter accidentally cutting it.
    #[test]
    fn burrowed_barbed_arachnathid_carries_burrow() {
        let abilities = unit_abilities("nbnb");
        assert!(
            contains_ignore_case(&abilities, "Abu5"),
            "nbnb must carry Burrow (Abu5); got {abilities:?}",
        );
    }

    /// Mercenary form (nanm) ships with `inEditor=0` because it lives in
    /// tavern data, not the World Editor. The relaxed `passes_filter`
    /// keeps it in the Melee catalog so users can bind Burrow on it.
    #[test]
    fn barbed_arachnathid_merc_is_in_melee_catalog() {
        let entries = UnitCatalog::entries_for(
            Some(Race::Neutral),
            Some(UnitMode::Melee),
            None,
            None,
            SearchField::UnitName,
            CatalogVisibility::default(),
        );
        let ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        assert!(
            ids.contains(&"nanm"),
            "Barbed Arachnathid merc (nanm) must survive the Melee catalog filter",
        );
        let abilities = unit_abilities("nanm");
        assert!(
            contains_ignore_case(&abilities, "Abu5"),
            "nanm must carry Burrow (Abu5); got {abilities:?}",
        );
    }

    /// Shadow Strike (ACss) was missing entirely from `WARCRAFT_DATABASE`
    /// as an `Ability` object before the unit_abilities union surfaced it.
    /// The default button position lives in `neutralabilityfunc.txt`
    /// section `[ACss]` at column 2, row 2.
    #[test]
    fn shadow_strike_ability_has_default_button_position() {
        use warcraft_api::{ColumnIndex, GridCoordinate, RowIndex};
        let object = WARCRAFT_DATABASE
            .by_id("ACss")
            .expect("Shadow Strike (ACss) must exist as an Ability object");
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            panic!("ACss must be an Ability");
        };
        assert_eq!(
            ability_meta.default_button_position(),
            Some(GridCoordinate::new(ColumnIndex::Two, RowIndex::Two)),
            "Shadow Strike's default Buttonpos is `2,2` in neutralabilityfunc.txt",
        );
    }

    // ----- Base-only `.txt` reads ---------------------------------------------

    /// Human Town Hall (htow): base `humanunitfunc.txt` publishes
    /// `Researches=Rhpm` (Backpack) and `Upgrade=hkee` (Keep). The
    /// `_balance/custom_v0` overlay only has `Upgrade=hkee`. A union /
    /// first-wins merge across overlays would drop Backpack research
    /// entirely. Base-only matchers preserve both.
    #[test]
    fn human_town_hall_has_backpack_research_and_keep_upgrade() {
        let researches = unit_researches("htow");
        assert!(
            researches.contains(&"Rhpm".to_string()),
            "htow must research Backpack (Rhpm); got {researches:?}",
        );
        assert!(
            researches.contains(&"hkee".to_string()),
            "htow must upgrade to Keep (hkee); got {researches:?}",
        );
        let trains = unit_trains("htow");
        assert!(
            trains.contains(&"hpea".to_string()),
            "htow must train Peasant (hpea); got {trains:?}",
        );
    }

    /// Orc Barracks (obar): the base ships four trains and four
    /// researches. Variant overlays sometimes drop entries.
    #[test]
    fn orc_barracks_publishes_full_base_production() {
        let trains = unit_trains("obar");
        for required in ["ogru", "ohun", "otbk", "ocat"] {
            assert!(
                trains.iter().any(|train| train == required),
                "obar must train {required}; got {trains:?}",
            );
        }
        let researches = unit_researches("obar");
        for required in ["Robs", "Rotr", "Robk", "Robf"] {
            assert!(
                researches.iter().any(|research| research == required),
                "obar must research {required}; got {researches:?}",
            );
        }
    }

    /// Goblin Merchant (ngme): the base publishes 11 specific sell items.
    /// `_balance/melee_v0` publishes a different 8-item set. If the
    /// extractor were to union those, the command card (12 cells) would
    /// drop entries off the end — pinv (Potion of Invisibility) went
    /// missing in the regression.
    #[test]
    fn goblin_merchant_publishes_base_sell_items_without_overlay_leak() {
        let sell_items = unit_sell_items("ngme");
        let base = [
            "stwp", "bspd", "dust", "tret", "prvt", "cnob", "stel", "pnvl", "shea", "spro", "pinv",
        ];
        for required in base {
            assert!(
                sell_items.iter().any(|item| item == required),
                "ngme must publish base sell item {required}; got {sell_items:?}",
            );
        }
        // melee_v0-only items
        let overlay_only = ["phea", "pman", "wneg", "gemt"];
        for leak in overlay_only {
            assert!(
                !sell_items.iter().any(|item| item == leak),
                "ngme must NOT publish melee_v0-only sell item {leak}; got {sell_items:?}",
            );
        }
        // Base ships exactly 11 items. Anything else is preset pollution.
        assert_eq!(
            sell_items.len(),
            11,
            "ngme must publish exactly 11 sell items; got {sell_items:?}",
        );
    }

    // ----- `notused_*.slk` exclusion ------------------------------------------

    /// Destroyer (ubsp) carries five abilities in the base
    /// `unitabilities.slk`. Reading `notused_*.slk` (different column
    /// layout) corrupted the transform-target filter and stripped four
    /// of them. Anchor the full set.
    #[test]
    fn destroyer_carries_full_base_ability_set() {
        let abilities = unit_abilities("ubsp");
        for required in ["Advm", "Afak", "Aave", "Aabs", "ACmi"] {
            assert!(
                abilities.iter().any(|ability| ability == required),
                "ubsp must carry {required}; got {abilities:?}",
            );
        }
    }

    /// Owl Scout (nowl): live `unitui.slk` flags it `special=1, inEditor=1`.
    /// The notused `unitui.slk` ships a different column layout where the
    /// equivalent row resolves to `special=0, inEditor=0` if read through
    /// the live column names. If `is_war3_units_path` ever stops excluding
    /// `notused_*.slk`, nowl's flags collapse to `inEditor=false` (and
    /// vanish from the catalog) and `special=false`. Asserting both flags
    /// here catches that regression no matter which way the notused row
    /// gets misinterpreted.
    #[test]
    fn owl_scout_flags_come_from_live_unitui_slk() {
        let object = WARCRAFT_DATABASE
            .by_id("nowl")
            .expect("Owl Scout (nowl) must exist");
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("nowl must be a Unit");
        };
        assert!(
            unit_meta.is_in_editor(),
            "nowl must have inEditor=1 from the live unitui.slk; a notused_unitui.slk read would flip this to false",
        );
        assert!(
            unit_meta.is_special(),
            "nowl must have special=1 from the live unitui.slk; a notused_unitui.slk read would flip this to false",
        );
    }

    // ----- Catalog filter semantics -------------------------------------------

    /// Campaign-flagged units stay out of Melee. The catalog has to keep
    /// rejecting `is_campaign=true` rows after the inEditor relaxation.
    #[test]
    fn campaign_units_stay_out_of_melee() {
        for race in [
            Race::Human,
            Race::Nightelf,
            Race::Orc,
            Race::Undead,
            Race::Neutral,
        ] {
            let entries = UnitCatalog::entries_for(
                Some(race),
                Some(UnitMode::Melee),
                None,
                None,
                SearchField::UnitName,
                CatalogVisibility::default(),
            );
            for entry in &entries {
                let WarcraftObjectMeta::Unit(unit_meta) = entry.warcraft_object().meta() else {
                    continue;
                };
                assert!(
                    !unit_meta.is_campaign(),
                    "{race:?}/Melee leaked campaign unit {}",
                    entry.unit_id(),
                );
            }
        }
    }

    /// Ability-less placeholder rows (Crystal Arachnathid `nanc`, Warrior
    /// Arachnathid `nanw`, Barbed Arachnathid `nanb`) must stay filtered
    /// out — they have no bindable abilities and no production, so
    /// they'd be dead entries in the unit list.
    #[test]
    fn ability_less_placeholders_stay_filtered() {
        let entries = UnitCatalog::entries_for(
            Some(Race::Neutral),
            Some(UnitMode::Melee),
            None,
            None,
            SearchField::UnitName,
            CatalogVisibility::default(),
        );
        let ids: Vec<&str> = entries.iter().map(|entry| entry.unit_id()).collect();
        for placeholder in ["nanc", "nanw"] {
            assert!(
                !ids.contains(&placeholder),
                "{placeholder} must stay filtered (no abilities, no production)",
            );
        }
    }
}
