//! Per-rule matcher + processor unit tests.
//!
//! Each rule has two responsibilities:
//!   (a) decide whether a CASC entry path is relevant (`matches`)
//!   (b) turn the byte payload into an `ExtractResult` (`process`)
//!
//! These tests exercise both against synthetic inputs — no game data, no
//! network, no CASC. They pin the matcher path predicates and the domain
//! mapping shape so a patch that changes the SLK schema or a file move inside
//! CASC fails loudly here before downstream crates rot.

use warcraft_api::{
    GridCoordinate, ItemClass, Race, UnitKind, WarcraftDatabase, WarcraftObjectMeta,
};
use warcraft_extractor::{
    ABILITY_DEFAULTS_EXTRACTION_RULE, ABILITY_METADATA_EXTRACTION_RULE,
    ABILITY_SKINS_EXTRACTION_RULE, CAMPAIGN_ABILITY_STRINGS_EXTRACTION_RULE,
    CAMPAIGN_UNIT_STRINGS_EXTRACTION_RULE, ExtractResult, HEROES_EXTRACTION_RULE,
    HUMAN_ABILITY_STRINGS_EXTRACTION_RULE, HUMAN_UNIT_STRINGS_EXTRACTION_RULE,
    HUMAN_UPGRADES_ART_EXTRACTION_RULE, HUMAN_UPGRADES_NAME_EXTRACTION_RULE,
    ITEM_ABILITY_STRINGS_EXTRACTION_RULE, ITEM_SKINS_EXTRACTION_RULE,
    ITEM_UNIT_STRINGS_EXTRACTION_RULE, ITEMS_EXTRACTION_RULE,
    NEUTRAL_ABILITY_STRINGS_EXTRACTION_RULE, NEUTRAL_UNIT_STRINGS_EXTRACTION_RULE,
    NIGHTELF_ABILITY_STRINGS_EXTRACTION_RULE, NIGHTELF_UNIT_STRINGS_EXTRACTION_RULE,
    NIGHTELF_UPGRADES_ART_EXTRACTION_RULE, ORC_ABILITY_STRINGS_EXTRACTION_RULE,
    ORC_UNIT_STRINGS_EXTRACTION_RULE, ORC_UPGRADES_ART_EXTRACTION_RULE,
    UNDEAD_ABILITY_STRINGS_EXTRACTION_RULE, UNDEAD_UNIT_STRINGS_EXTRACTION_RULE,
    UNDEAD_UPGRADES_ART_EXTRACTION_RULE, UNIT_ABILITIES_EXTRACTION_RULE,
    UNIT_SKINS_EXTRACTION_RULE, UNITS_EXTRACTION_RULE, WarcraftDataAggregation,
};

const HEROES_CASC_PATH: &str = "war3.w3mod:units/abilitydata.slk";
const UNITS_CASC_PATH: &str = "war3.w3mod:units/unitbalance.slk";
const ITEMS_CASC_PATH: &str = "war3.w3mod:units/itemdata.slk";
const ABILITY_SKINS_CASC_PATH: &str = "war3.w3mod:units/abilityskin.txt";
const ITEM_SKINS_CASC_PATH: &str = "war3.w3mod:units/itemfunc.txt";
const UNIT_SKINS_CASC_PATH: &str = "war3.w3mod:units/unitskin.txt";

mod heroes {
    use super::*;

    #[test]
    fn matcher_accepts_abilitydata_slk_path() {
        assert!(HEROES_EXTRACTION_RULE.matches(HEROES_CASC_PATH));
    }

    #[test]
    fn matcher_rejects_sibling_paths() {
        assert!(!HEROES_EXTRACTION_RULE.matches("war3.w3mod:units/unitbalance.slk"));
        assert!(!HEROES_EXTRACTION_RULE.matches("war3.w3mod:units/itemdata.slk"));
        assert!(!HEROES_EXTRACTION_RULE.matches("other.w3mod:units/abilitydata.slk"));
    }

    #[test]
    fn processor_extracts_hero_ability_with_race_and_levels() {
        let slk = "ID;P
C;X1;Y1;K\"alias\"
C;X2;Y1;K\"comments\"
C;X3;Y1;K\"hero\"
C;X4;Y1;K\"race\"
C;X5;Y1;K\"levels\"
C;X6;Y1;K\"Cool1\"
C;X7;Y1;K\"Cool2\"
C;X8;Y1;K\"Cool3\"
C;X9;Y1;K\"Cool4\"
C;X1;Y2;K\"AHbh\"
C;X2;Y2;K\"Paladin - Holy Light (some notes)\"
C;X3;Y2;K\"1\"
C;X4;Y2;K\"human\"
C;X5;Y2;K\"3\"
C;X6;Y2;K\"7.5\"
C;X7;Y2;K\"5.5\"
C;X8;Y2;K\"3.5\"
C;X9;Y2;K\"0\"
E
";
        let result = HEROES_EXTRACTION_RULE
            .process(HEROES_CASC_PATH, slk.as_bytes())
            .expect("heroes processor failed");
        let ExtractResult::Heroes(database) = result else {
            panic!("expected Heroes variant");
        };
        let paladin_abilities = database.get("Paladin").expect("Paladin key missing");
        assert_eq!(paladin_abilities.len(), 1);
        let ability = paladin_abilities.iter().next().unwrap();
        assert_eq!(ability.id(), "AHbh");
        assert_eq!(ability.ability(), "Holy Light");
        assert_eq!(ability.race(), Race::Human);
        assert_eq!(ability.max_level(), 3);
        assert!(!ability.is_ultimate());
        assert_eq!(ability.cooldowns(), [7500, 5500, 3500, 0]);
    }

    #[test]
    fn processor_marks_levels_other_than_three_as_ultimate_max_level_one() {
        let slk = "ID;P
C;X1;Y1;K\"alias\"
C;X2;Y1;K\"comments\"
C;X3;Y1;K\"hero\"
C;X4;Y1;K\"race\"
C;X5;Y1;K\"levels\"
C;X1;Y2;K\"AHav\"
C;X2;Y2;K\"Paladin - Avatar\"
C;X3;Y2;K\"1\"
C;X4;Y2;K\"human\"
C;X5;Y2;K\"1\"
E
";
        let result = HEROES_EXTRACTION_RULE
            .process(HEROES_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Heroes(database) = result else {
            unreachable!()
        };
        let ability = database.get("Paladin").unwrap().iter().next().unwrap();
        assert_eq!(ability.max_level(), 1);
        assert!(ability.is_ultimate());
    }

    #[test]
    fn processor_skips_non_hero_rows() {
        let slk = "ID;P
C;X1;Y1;K\"alias\"
C;X2;Y1;K\"comments\"
C;X3;Y1;K\"hero\"
C;X4;Y1;K\"race\"
C;X5;Y1;K\"levels\"
C;X1;Y2;K\"Aply\"
C;X2;Y2;K\"Peasant - Something\"
C;X3;Y2;K\"0\"
C;X4;Y2;K\"human\"
C;X5;Y2;K\"1\"
E
";
        let result = HEROES_EXTRACTION_RULE
            .process(HEROES_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Heroes(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }

    #[test]
    fn processor_skips_unsupported_races() {
        let slk = "ID;P
C;X1;Y1;K\"alias\"
C;X2;Y1;K\"comments\"
C;X3;Y1;K\"hero\"
C;X4;Y1;K\"race\"
C;X5;Y1;K\"levels\"
C;X1;Y2;K\"AZZZ\"
C;X2;Y2;K\"Nobody - Nothing\"
C;X3;Y2;K\"1\"
C;X4;Y2;K\"naga\"
C;X5;Y2;K\"3\"
E
";
        let result = HEROES_EXTRACTION_RULE
            .process(HEROES_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Heroes(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }

    #[test]
    fn processor_rejects_non_utf8_bytes() {
        let invalid_utf8: [u8; 4] = [0xff, 0xfe, 0x00, 0xff];
        let outcome = HEROES_EXTRACTION_RULE.process(HEROES_CASC_PATH, &invalid_utf8);
        assert!(outcome.is_err());
    }
}

mod units {
    use super::*;

    #[test]
    fn matcher_accepts_unitbalance_slk_path() {
        assert!(UNITS_EXTRACTION_RULE.matches(UNITS_CASC_PATH));
    }

    #[test]
    fn matcher_rejects_sibling_paths() {
        assert!(!UNITS_EXTRACTION_RULE.matches("war3.w3mod:units/abilitydata.slk"));
        assert!(!UNITS_EXTRACTION_RULE.matches("other.w3mod:units/unitbalance.slk"));
    }

    #[test]
    fn processor_routes_units_by_race_and_kind() {
        let slk = "ID;P
C;X1;Y1;K\"unitBalanceID\"
C;X2;Y1;K\"defType\"
C;X3;Y1;K\"isbldg\"
C;X4;Y1;K\"bldtm\"
C;X1;Y2;K\"hpea\"
C;X2;Y2;K\"worker\"
C;X3;Y2;K\"0\"
C;X4;Y2;K\"15\"
C;X1;Y3;K\"htow\"
C;X2;Y3;K\"building\"
C;X3;Y3;K\"1\"
C;X4;Y3;K\"60\"
C;X1;Y4;K\"Hamg\"
C;X2;Y4;K\"hero\"
C;X3;Y4;K\"0\"
C;X4;Y4;K\"55\"
E
";
        let result = UNITS_EXTRACTION_RULE
            .process(UNITS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Units(database) = result else {
            panic!("expected Units variant");
        };
        let human_units = database.get(&Race::Human).expect("human race missing");
        let worker_peasant = human_units
            .get(&UnitKind::Worker)
            .and_then(|workers| workers.get("hpea"))
            .expect("hpea worker missing");
        assert_eq!(worker_peasant.build_time(), 15);
        let building_town_hall = human_units
            .get(&UnitKind::Building)
            .and_then(|buildings| buildings.get("htow"))
            .expect("htow building missing");
        assert_eq!(building_town_hall.build_time(), 60);
        let hero_arthas = human_units
            .get(&UnitKind::Hero)
            .and_then(|heroes| heroes.get("Hamg"))
            .expect("Hamg hero missing");
        assert_eq!(hero_arthas.build_time(), 55);
    }

    #[test]
    fn processor_skips_unit_ids_with_unknown_race_prefix() {
        let slk = "ID;P
C;X1;Y1;K\"unitBalanceID\"
C;X2;Y1;K\"defType\"
C;X3;Y1;K\"isbldg\"
C;X4;Y1;K\"bldtm\"
C;X1;Y2;K\"zzzz\"
C;X2;Y2;K\"worker\"
C;X3;Y2;K\"0\"
C;X4;Y2;K\"10\"
E
";
        let result = UNITS_EXTRACTION_RULE
            .process(UNITS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Units(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }

    #[test]
    fn processor_skips_rows_with_empty_id() {
        let slk = "ID;P
C;X1;Y1;K\"unitBalanceID\"
C;X2;Y1;K\"defType\"
C;X3;Y1;K\"isbldg\"
C;X4;Y1;K\"bldtm\"
C;X1;Y2;K\"\"
C;X2;Y2;K\"worker\"
C;X3;Y2;K\"0\"
C;X4;Y2;K\"10\"
E
";
        let result = UNITS_EXTRACTION_RULE
            .process(UNITS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Units(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }
}

mod items {
    use super::*;

    #[test]
    fn matcher_accepts_itemdata_slk_path() {
        assert!(ITEMS_EXTRACTION_RULE.matches(ITEMS_CASC_PATH));
    }

    #[test]
    fn matcher_rejects_sibling_paths() {
        assert!(!ITEMS_EXTRACTION_RULE.matches("war3.w3mod:units/abilitydata.slk"));
        assert!(!ITEMS_EXTRACTION_RULE.matches("war3.w3mod:units/unitbalance.slk"));
    }

    #[test]
    fn processor_groups_items_by_class_and_parses_abilities() {
        let slk = "ID;P
C;X1;Y1;K\"itemID\"
C;X2;Y1;K\"class\"
C;X3;Y1;K\"Level\"
C;X4;Y1;K\"abilList\"
C;X5;Y1;K\"cooldownID\"
C;X1;Y2;K\"stel\"
C;X2;Y2;K\"Permanent\"
C;X3;Y2;K\"3\"
C;X4;Y2;K\"AIst,AIva\"
C;X5;Y2;K\"\"
E
";
        let result = ITEMS_EXTRACTION_RULE
            .process(ITEMS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Items(database) = result else {
            panic!("expected Items variant");
        };
        let permanents = database
            .get(&ItemClass::Permanent)
            .expect("Permanent class missing");
        let stel = permanents.get("stel").expect("stel item missing");
        assert_eq!(stel.level(), 3);
        assert_eq!(
            stel.ability_list(),
            &["AIst".to_string(), "AIva".to_string()]
        );
    }

    #[test]
    fn processor_strips_placeholder_ability_tokens() {
        let slk = "ID;P
C;X1;Y1;K\"itemID\"
C;X2;Y1;K\"class\"
C;X3;Y1;K\"Level\"
C;X4;Y1;K\"abilList\"
C;X1;Y2;K\"xxxx\"
C;X2;Y2;K\"Permanent\"
C;X3;Y2;K\"1\"
C;X4;Y2;K\"-,AIst,_, ,AIva\"
E
";
        let result = ITEMS_EXTRACTION_RULE
            .process(ITEMS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Items(database) = result else {
            unreachable!()
        };
        let item = database
            .get(&ItemClass::Permanent)
            .unwrap()
            .get("xxxx")
            .unwrap();
        assert_eq!(
            item.ability_list(),
            &["AIst".to_string(), "AIva".to_string()]
        );
    }

    #[test]
    fn processor_skips_rows_with_unknown_class() {
        let slk = "ID;P
C;X1;Y1;K\"itemID\"
C;X2;Y1;K\"class\"
C;X3;Y1;K\"Level\"
C;X4;Y1;K\"abilList\"
C;X1;Y2;K\"xxxx\"
C;X2;Y2;K\"NotARealClass\"
C;X3;Y2;K\"1\"
C;X4;Y2;K\"\"
E
";
        let result = ITEMS_EXTRACTION_RULE
            .process(ITEMS_CASC_PATH, slk.as_bytes())
            .unwrap();
        let ExtractResult::Items(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }
}

mod skins {
    use super::*;

    #[test]
    fn matcher_accepts_unit_skin_path() {
        assert!(UNIT_SKINS_EXTRACTION_RULE.matches(UNIT_SKINS_CASC_PATH));
        assert!(!UNIT_SKINS_EXTRACTION_RULE.matches(ITEM_SKINS_CASC_PATH));
    }

    #[test]
    fn matcher_accepts_item_skin_path() {
        assert!(ITEM_SKINS_EXTRACTION_RULE.matches(ITEM_SKINS_CASC_PATH));
        assert!(!ITEM_SKINS_EXTRACTION_RULE.matches(UNIT_SKINS_CASC_PATH));
    }

    #[test]
    fn matcher_accepts_ability_skin_path() {
        assert!(ABILITY_SKINS_EXTRACTION_RULE.matches(ABILITY_SKINS_CASC_PATH));
        assert!(!ABILITY_SKINS_EXTRACTION_RULE.matches(UNIT_SKINS_CASC_PATH));
    }

    #[test]
    fn unit_skin_processor_prefers_art_over_art_sd() {
        let text = "[hpea]\n\
                    Art=ReplaceableTextures\\CommandButtons\\BTNPeasant.blp\n\
                    Art:sd=ReplaceableTextures\\CommandButtons\\BTNPeasantSD.blp\n";
        let result = UNIT_SKINS_EXTRACTION_RULE
            .process(UNIT_SKINS_CASC_PATH, text.as_bytes())
            .unwrap();
        let ExtractResult::UnitSkin(database) = result else {
            panic!("expected UnitSkin variant");
        };
        let art_path = database.get("hpea").expect("hpea missing");
        assert_eq!(
            art_path,
            "ReplaceableTextures/CommandButtons/BTNPeasant.blp"
        );
    }

    #[test]
    fn unit_skin_processor_falls_back_to_art_sd_when_art_absent() {
        let text = "[hpea]\nArt:sd=ReplaceableTextures\\CommandButtons\\BTNPeasantSD.blp\n";
        let result = UNIT_SKINS_EXTRACTION_RULE
            .process(UNIT_SKINS_CASC_PATH, text.as_bytes())
            .unwrap();
        let ExtractResult::UnitSkin(database) = result else {
            unreachable!()
        };
        let art_path = database.get("hpea").expect("hpea missing");
        assert_eq!(
            art_path,
            "ReplaceableTextures/CommandButtons/BTNPeasantSD.blp"
        );
    }

    #[test]
    fn unit_skin_processor_skips_sections_without_any_art() {
        let text = "[hpea]\nOther=value\n";
        let result = UNIT_SKINS_EXTRACTION_RULE
            .process(UNIT_SKINS_CASC_PATH, text.as_bytes())
            .unwrap();
        let ExtractResult::UnitSkin(database) = result else {
            unreachable!()
        };
        assert!(database.is_empty());
    }

    #[test]
    fn unit_skin_processor_strips_utf8_bom() {
        let mut text_with_bom = Vec::new();
        text_with_bom.extend_from_slice("\u{feff}".as_bytes());
        text_with_bom.extend_from_slice("[hpea]\nArt=a\\b.blp\n".as_bytes());
        let result = UNIT_SKINS_EXTRACTION_RULE
            .process(UNIT_SKINS_CASC_PATH, &text_with_bom)
            .unwrap();
        let ExtractResult::UnitSkin(database) = result else {
            unreachable!()
        };
        assert_eq!(database.get("hpea"), Some(&"a/b.blp".to_string()));
    }
}

mod upgrades {
    use super::*;

    const HUMAN_ART_CASC_PATH: &str = "war3.w3mod:units/humanupgradefunc.txt";
    const HUMAN_NAME_CASC_PATH: &str = "somepath/enus.w3mod:units/humanupgradestrings.txt";

    #[test]
    fn art_rule_matcher_accepts_race_specific_func_path() {
        assert!(HUMAN_UPGRADES_ART_EXTRACTION_RULE.matches(HUMAN_ART_CASC_PATH));
        assert!(
            NIGHTELF_UPGRADES_ART_EXTRACTION_RULE
                .matches("war3.w3mod:units/nightelfupgradefunc.txt")
        );
        assert!(ORC_UPGRADES_ART_EXTRACTION_RULE.matches("war3.w3mod:units/orcupgradefunc.txt"));
        assert!(
            UNDEAD_UPGRADES_ART_EXTRACTION_RULE.matches("war3.w3mod:units/undeadupgradefunc.txt")
        );
    }

    #[test]
    fn art_rule_matcher_rejects_other_race_func_paths() {
        assert!(!HUMAN_UPGRADES_ART_EXTRACTION_RULE.matches("war3.w3mod:units/orcupgradefunc.txt"));
        assert!(
            !NIGHTELF_UPGRADES_ART_EXTRACTION_RULE.matches("war3.w3mod:units/humanupgradefunc.txt")
        );
    }

    #[test]
    fn name_rule_matcher_requires_enus_locale() {
        assert!(HUMAN_UPGRADES_NAME_EXTRACTION_RULE.matches(HUMAN_NAME_CASC_PATH));
        assert!(
            !HUMAN_UPGRADES_NAME_EXTRACTION_RULE
                .matches("war3.w3mod:units/humanupgradestrings.txt")
        );
    }

    #[test]
    fn art_processor_parses_icons_per_section() {
        let text = "[Rhme]\n\
                    Art=ReplaceableTextures\\CommandButtons\\BTNIronForgedSwords.blp,\
                    ReplaceableTextures\\CommandButtons\\BTNSteelForgedSwords.blp,\
                    ReplaceableTextures\\CommandButtons\\BTNMithrilForgedSwords.blp\n";
        let result = HUMAN_UPGRADES_ART_EXTRACTION_RULE
            .process(HUMAN_ART_CASC_PATH, text.as_bytes())
            .unwrap();
        let ExtractResult::HumanUpgradesArt(database) = result else {
            panic!("expected HumanUpgradesArt variant");
        };
        let entry = database.get("Rhme").expect("Rhme upgrade missing");
        let icons = entry.get_icons();
        assert_eq!(icons.len(), 3);
        assert!(icons[0].contains("IronForgedSwords"));
    }

    #[test]
    fn name_processor_parses_per_section() {
        let text = "[Rhme]\n\
                    Name=\"Iron Forged Swords\",\"Steel Forged Swords\",\"Mithril Forged Swords\"\n";
        let result = HUMAN_UPGRADES_NAME_EXTRACTION_RULE
            .process(HUMAN_NAME_CASC_PATH, text.as_bytes())
            .unwrap();
        let ExtractResult::HumanUpgradesName(database) = result else {
            panic!("expected HumanUpgradesName variant");
        };
        let names = database
            .get("Rhme")
            .expect("Rhme upgrade missing")
            .get_names();
        assert_eq!(
            names,
            vec![
                "Iron Forged Swords",
                "Steel Forged Swords",
                "Mithril Forged Swords"
            ]
        );
    }
}

mod strings {
    use super::*;

    #[test]
    fn human_ability_matcher_requires_enus_and_file() {
        assert!(
            HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
                .matches("x/enus.w3mod:units/humanabilitystrings.txt")
        );
        assert!(
            !HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
                .matches("x/zhcn.w3mod:units/humanabilitystrings.txt")
        );
        assert!(
            !HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
                .matches("x/enus.w3mod:units/orcabilitystrings.txt")
        );
    }

    #[test]
    fn human_unit_matcher_requires_enus_and_file() {
        assert!(
            HUMAN_UNIT_STRINGS_EXTRACTION_RULE.matches("x/enus.w3mod:units/humanunitstrings.txt")
        );
        assert!(
            !HUMAN_UNIT_STRINGS_EXTRACTION_RULE.matches("x/enus.w3mod:units/orcunitstrings.txt")
        );
    }

    #[test]
    fn per_race_ability_matchers_are_disjoint() {
        let ability_rules = [
            HUMAN_ABILITY_STRINGS_EXTRACTION_RULE,
            NIGHTELF_ABILITY_STRINGS_EXTRACTION_RULE,
            ORC_ABILITY_STRINGS_EXTRACTION_RULE,
            UNDEAD_ABILITY_STRINGS_EXTRACTION_RULE,
            NEUTRAL_ABILITY_STRINGS_EXTRACTION_RULE,
            ITEM_ABILITY_STRINGS_EXTRACTION_RULE,
            CAMPAIGN_ABILITY_STRINGS_EXTRACTION_RULE,
        ];
        let examples = [
            "x/enus.w3mod:units/humanabilitystrings.txt",
            "x/enus.w3mod:units/nightelfabilitystrings.txt",
            "x/enus.w3mod:units/orcabilitystrings.txt",
            "x/enus.w3mod:units/undeadabilitystrings.txt",
            "x/enus.w3mod:units/neutralabilitystrings.txt",
            "x/enus.w3mod:units/itemabilitystrings.txt",
            "x/enus.w3mod:units/campaignabilitystrings.txt",
        ];
        for (expected_index, path) in examples.iter().enumerate() {
            for (rule_index, rule) in ability_rules.iter().enumerate() {
                let should_match = rule_index == expected_index;
                assert_eq!(
                    rule.matches(path),
                    should_match,
                    "ability rule {rule_index} vs path '{path}' (expected match={should_match})"
                );
            }
        }
    }

    #[test]
    fn per_race_unit_matchers_are_disjoint() {
        let unit_rules = [
            HUMAN_UNIT_STRINGS_EXTRACTION_RULE,
            NIGHTELF_UNIT_STRINGS_EXTRACTION_RULE,
            ORC_UNIT_STRINGS_EXTRACTION_RULE,
            UNDEAD_UNIT_STRINGS_EXTRACTION_RULE,
            NEUTRAL_UNIT_STRINGS_EXTRACTION_RULE,
            ITEM_UNIT_STRINGS_EXTRACTION_RULE,
            CAMPAIGN_UNIT_STRINGS_EXTRACTION_RULE,
        ];
        let examples = [
            "x/enus.w3mod:units/humanunitstrings.txt",
            "x/enus.w3mod:units/nightelfunitstrings.txt",
            "x/enus.w3mod:units/orcunitstrings.txt",
            "x/enus.w3mod:units/undeadunitstrings.txt",
            "x/enus.w3mod:units/neutralunitstrings.txt",
            "x/enus.w3mod:units/itemstrings.txt",
            "x/enus.w3mod:units/campaignunitstrings.txt",
        ];
        for (expected_index, path) in examples.iter().enumerate() {
            for (rule_index, rule) in unit_rules.iter().enumerate() {
                let should_match = rule_index == expected_index;
                assert_eq!(
                    rule.matches(path),
                    should_match,
                    "unit rule {rule_index} vs path '{path}' (expected match={should_match})"
                );
            }
        }
    }

    #[test]
    fn human_ability_processor_emits_ability_strings_variant() {
        let text = "[AHhb]\nName=Holy Light\n[AHav]\nName=Avatar\n";
        let result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(
                "x/enus.w3mod:units/humanabilitystrings.txt",
                text.as_bytes(),
            )
            .unwrap();
        let ExtractResult::HumanAbilityStrings(database) = result else {
            panic!("expected HumanAbilityStrings variant");
        };
        assert_eq!(database.get("AHhb").unwrap().value(), "Holy Light");
        assert_eq!(database.get("AHav").unwrap().value(), "Avatar");
    }

    #[test]
    fn human_unit_processor_emits_unit_strings_variant() {
        let text = "[hpea]\nName=Peasant\n[htow]\nName=Town Hall\n";
        let result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process("x/enus.w3mod:units/humanunitstrings.txt", text.as_bytes())
            .unwrap();
        let ExtractResult::HumanUnitStrings(database) = result else {
            panic!("expected HumanUnitStrings variant");
        };
        assert_eq!(database.get("hpea").unwrap().value(), "Peasant");
        assert_eq!(database.get("htow").unwrap().value(), "Town Hall");
    }
}

/// Reforged ships a base `war3.w3mod:units/<file>` plus three balance
/// overlays under `war3.w3mod:_balance/<variant>.w3mod:units/<file>` for
/// melee_v0 and the two custom modes. The overlays add ability rows the
/// base file is missing (e.g. Shadow Strike on ndqp in custom_v1) — they
/// must be picked up by the same rules as the base file, and the merge
/// has to be additive so existing rows never get smaller.
mod balance_overlays {
    use super::*;

    const BASE_UNIT_ABILITIES_PATH: &str = "war3.w3mod:units/unitabilities.slk";
    const CUSTOM_V1_UNIT_ABILITIES_PATH: &str =
        "war3.w3mod:_balance/custom_v1.w3mod:units/unitabilities.slk";
    const MELEE_V0_UNIT_ABILITIES_PATH: &str =
        "war3.w3mod:_balance/melee_v0.w3mod:units/unitabilities.slk";
    const CUSTOM_V1_ABILITY_FUNC_PATH: &str =
        "war3.w3mod:_balance/custom_v1.w3mod:units/neutralabilityfunc.txt";
    const CUSTOM_V1_UNITBALANCE_PATH: &str =
        "war3.w3mod:_balance/custom_v1.w3mod:units/unitbalance.slk";

    #[test]
    fn unit_abilities_matcher_accepts_balance_overlay_paths() {
        assert!(UNIT_ABILITIES_EXTRACTION_RULE.matches(BASE_UNIT_ABILITIES_PATH));
        assert!(UNIT_ABILITIES_EXTRACTION_RULE.matches(CUSTOM_V1_UNIT_ABILITIES_PATH));
        assert!(UNIT_ABILITIES_EXTRACTION_RULE.matches(MELEE_V0_UNIT_ABILITIES_PATH));
    }

    #[test]
    fn unit_abilities_matcher_rejects_non_war3_namespaces() {
        let foreign_path = "other.w3mod:_balance/custom_v1.w3mod:units/unitabilities.slk";
        assert!(!UNIT_ABILITIES_EXTRACTION_RULE.matches(foreign_path));
    }

    /// `.txt`-based rules (`unitfunc.txt`, `abilityfunc.txt`) deliberately
    /// do *not* match the balance overlays. The overlays are alternative
    /// gameplay presets, not strict supersets — e.g. `_balance/melee_v0`'s
    /// `Goblin Merchant` lists `phea,pman,pinv,...` while the base lists
    /// `stwp,bspd,...,pinv`. Unioning those across presets pollutes the
    /// command card with items that don't belong on the base/Reforged
    /// preset, so for production-unit / training / research / sell-item
    /// data we read base only.
    #[test]
    fn ability_defaults_matcher_rejects_balance_overlay_paths() {
        assert!(!ABILITY_DEFAULTS_EXTRACTION_RULE.matches(CUSTOM_V1_ABILITY_FUNC_PATH));
    }

    #[test]
    fn units_matcher_rejects_balance_overlay_paths() {
        assert!(!UNITS_EXTRACTION_RULE.matches(CUSTOM_V1_UNITBALANCE_PATH));
    }

    fn unit_abilities_row(unit_id: &str, ability_list: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitAbilID\"\n\
             C;X2;Y1;K\"abilList\"\n\
             C;X3;Y1;K\"heroAbilList\"\n\
             C;X1;Y2;K\"{unit_id}\"\n\
             C;X2;Y2;K\"{ability_list}\"\n\
             E\n"
        )
    }

    /// Reproduces the maiden of pain (ndqp) regression: the base file lists
    /// `ACdr,ACss`, the custom_v1 overlay also lists both. After merging
    /// both extraction results we still expect ACss on the unit — i.e. the
    /// second overlay must not wipe out the first one's abilities.
    #[test]
    fn aggregator_unions_unit_abilities_across_overlays() {
        let base_slk = unit_abilities_row("ndqp", "ACdr,ACss");
        let overlay_slk = unit_abilities_row("ndqp", "ACdr,ACss");
        let base_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(BASE_UNIT_ABILITIES_PATH, base_slk.as_bytes())
            .unwrap();
        let overlay_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(CUSTOM_V1_UNIT_ABILITIES_PATH, overlay_slk.as_bytes())
            .unwrap();
        let aggregation = WarcraftDataAggregation::from(vec![base_result, overlay_result]);
        let ndqp_entry = aggregation
            .unit_abilities()
            .get("ndqp")
            .expect("ndqp entry missing");
        let abilities = ndqp_entry.abilities();
        assert_eq!(abilities, &[String::from("ACdr"), String::from("ACss")]);
    }

    /// When an overlay adds an ability the base file doesn't have (e.g.
    /// custom_v1's nane row that lists `ACvs,ACss` while the base lists
    /// only `Acvs`), the merged entry must carry both — that's the whole
    /// point of expanding the matcher to the balance dirs.
    #[test]
    fn aggregator_adds_overlay_only_abilities_to_existing_unit() {
        let base_slk = unit_abilities_row("nane", "Acvs");
        let overlay_slk = unit_abilities_row("nane", "ACvs,ACss");
        let base_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(BASE_UNIT_ABILITIES_PATH, base_slk.as_bytes())
            .unwrap();
        let overlay_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(CUSTOM_V1_UNIT_ABILITIES_PATH, overlay_slk.as_bytes())
            .unwrap();
        let aggregation = WarcraftDataAggregation::from(vec![base_result, overlay_result]);
        let nane_entry = aggregation
            .unit_abilities()
            .get("nane")
            .expect("nane entry missing");
        let abilities = nane_entry.abilities();
        let acss_present = abilities
            .iter()
            .any(|ability| ability.eq_ignore_ascii_case("ACss"));
        let acvs_present = abilities
            .iter()
            .any(|ability| ability.eq_ignore_ascii_case("Acvs"));
        assert!(acvs_present, "Acvs lost from nane after overlay merge");
        assert!(acss_present, "ACss missing from nane after overlay merge");
    }

    /// The hard rule: an overlay must never shrink a unit's ability list.
    /// Build the base with a superset (Ahar, Amil, Ahrp, Ahlh) and the
    /// overlay with a subset (Ahar, Amil, Ahrp) — the merged result must
    /// still contain Ahlh because the base had it.
    #[test]
    fn aggregator_never_drops_abilities_present_only_in_base() {
        let base_slk = unit_abilities_row("Hpal", "Ahar,Amil,Ahrp,Ahlh");
        let overlay_slk = unit_abilities_row("Hpal", "Ahar,Amil,Ahrp");
        let base_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(BASE_UNIT_ABILITIES_PATH, base_slk.as_bytes())
            .unwrap();
        let overlay_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(CUSTOM_V1_UNIT_ABILITIES_PATH, overlay_slk.as_bytes())
            .unwrap();
        let aggregation = WarcraftDataAggregation::from(vec![base_result, overlay_result]);
        let paladin_entry = aggregation
            .unit_abilities()
            .get("Hpal")
            .expect("Hpal entry missing");
        let abilities = paladin_entry.abilities();
        assert!(
            abilities.iter().any(|ability| ability == "Ahlh"),
            "base-only ability Ahlh was dropped by overlay merge",
        );
    }

    /// Same union behavior when the overlay is processed before the base
    /// (CASC iteration order isn't guaranteed). Earlier inserts must not
    /// disappear, later ones must still get added.
    #[test]
    fn aggregator_union_is_order_independent() {
        let base_slk = unit_abilities_row("ndqp", "ACdr,ACss");
        let overlay_slk = unit_abilities_row("ndqp", "ACdr,Anew");
        let base_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(BASE_UNIT_ABILITIES_PATH, base_slk.as_bytes())
            .unwrap();
        let overlay_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(CUSTOM_V1_UNIT_ABILITIES_PATH, overlay_slk.as_bytes())
            .unwrap();
        let overlay_first = WarcraftDataAggregation::from(vec![overlay_result, base_result]);
        let ndqp_entry = overlay_first
            .unit_abilities()
            .get("ndqp")
            .expect("ndqp entry missing");
        let abilities = ndqp_entry.abilities();
        let has_acss = abilities.iter().any(|ability| ability == "ACss");
        let has_anew = abilities.iter().any(|ability| ability == "Anew");
        assert!(has_acss);
        assert!(has_anew);
    }
}

/// Rule 5: when two self-referential abilities share the same default button
/// position and the same display name on a unit, one is a balance-patch
/// replacement of the other.  The superseded ability must be dropped when
/// the database is built from the aggregation.
///
/// Tests use a synthetic human unit (`htst`) with a minimal SLK/TXT set that
/// covers only the fields the build path reads.  The paths must match the
/// relevant extraction rule matchers.
mod rule_5_same_slot_dedup {
    use super::*;

    const UNIT_PATH: &str = "war3.w3mod:units/unitbalance.slk";
    const UNIT_ABILITIES_PATH: &str = "war3.w3mod:units/unitabilities.slk";
    const ABILITY_METADATA_PATH: &str = "war3.w3mod:units/abilitydata.slk";
    const ABILITY_DEFAULTS_PATH: &str = "war3.w3mod:units/humanabilityfunc.txt";
    const UNIT_STRINGS_PATH: &str = "x/enus.w3mod:units/humanunitstrings.txt";
    const ABILITY_STRINGS_PATH: &str = "x/enus.w3mod:units/humanabilitystrings.txt";

    fn unit_slk(unit_id: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitBalanceID\"\n\
             C;X2;Y1;K\"defType\"\n\
             C;X3;Y1;K\"isbldg\"\n\
             C;X4;Y1;K\"bldtm\"\n\
             C;X1;Y2;K\"{unit_id}\"\n\
             C;X2;Y2;K\"normal\"\n\
             C;X3;Y2;K\"0\"\n\
             C;X4;Y2;K\"30\"\n\
             E\n"
        )
    }

    fn unit_abilities_slk(unit_id: &str, ability_list: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitAbilID\"\n\
             C;X2;Y1;K\"abilList\"\n\
             C;X3;Y1;K\"heroAbilList\"\n\
             C;X1;Y2;K\"{unit_id}\"\n\
             C;X2;Y2;K\"{ability_list}\"\n\
             E\n"
        )
    }

    fn unit_abilities_slk_hero(unit_id: &str, hero_list: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitAbilID\"\n\
             C;X2;Y1;K\"abilList\"\n\
             C;X3;Y1;K\"heroAbilList\"\n\
             C;X1;Y2;K\"{unit_id}\"\n\
             C;X2;Y2;K\"\"\n\
             C;X3;Y2;K\"{hero_list}\"\n\
             E\n"
        )
    }

    fn ability_metadata_slk(id_a: &str, id_b: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X1;Y2;K\"{id_a}\"\n\
             C;X2;Y2;K\"{id_a}\"\n\
             C;X1;Y3;K\"{id_b}\"\n\
             C;X2;Y3;K\"{id_b}\"\n\
             E\n"
        )
    }

    /// Dodge chance is read from the real numeric data field, gated on the
    /// ability's base mechanic `code`: Evasion (`AEev`) keeps it in `DataA`,
    /// Drunken Brawler (`ANdb`) in `DataD` (its `DataA`/`DataB` are the
    /// critical-strike chance/multiplier). Only the real `levels` count is
    /// read, so the filler `DataA4`/`DataD4` past a 3-level ability stays 0.
    /// Non-evasion abilities get `[0.0; 4]`. Tooltip text is never consulted.
    #[test]
    fn evasion_chance_comes_from_real_data_field_per_base_code() {
        let meta_slk_text = "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X3;Y1;K\"levels\"\n\
             C;X4;Y1;K\"DataA1\"\n\
             C;X5;Y1;K\"DataA2\"\n\
             C;X6;Y1;K\"DataA3\"\n\
             C;X7;Y1;K\"DataD1\"\n\
             C;X8;Y1;K\"DataD2\"\n\
             C;X9;Y1;K\"DataD3\"\n\
             C;X1;Y2;K\"AEev\"\n\
             C;X2;Y2;K\"AEev\"\n\
             C;X3;Y2;K\"3\"\n\
             C;X4;Y2;K\"0.1\"\n\
             C;X5;Y2;K\"0.2\"\n\
             C;X6;Y2;K\"0.3\"\n\
             C;X1;Y3;K\"Acdb\"\n\
             C;X2;Y3;K\"ANdb\"\n\
             C;X3;Y3;K\"3\"\n\
             C;X4;Y3;K\"10\"\n\
             C;X5;Y3;K\"10\"\n\
             C;X6;Y3;K\"10\"\n\
             C;X7;Y3;K\"0.07\"\n\
             C;X8;Y3;K\"0.14\"\n\
             C;X9;Y3;K\"0.21\"\n\
             C;X1;Y4;K\"Shop\"\n\
             C;X2;Y4;K\"Apit\"\n\
             C;X3;Y4;K\"1\"\n\
             E\n"
        .to_string();

        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let ExtractResult::AbilityMetadata(database) = meta_result else {
            panic!("expected an AbilityMetadata extraction result");
        };

        let evasion = database
            .get("AEev")
            .expect("AEev present")
            .evasion_chance_per_level();
        assert_eq!(evasion, [0.1, 0.2, 0.3, 0.0]);

        let drunken_brawler = database
            .get("Acdb")
            .expect("Acdb present")
            .evasion_chance_per_level();
        assert_eq!(drunken_brawler, [0.07, 0.14, 0.21, 0.0]);

        let shop = database
            .get("Shop")
            .expect("Shop present")
            .evasion_chance_per_level();
        assert_eq!(shop, [0.0, 0.0, 0.0, 0.0]);
    }

    fn ability_defaults_txt(id_a: &str, id_b: &str, column: u8, row: u8) -> String {
        format!(
            "[{id_a}]\nButtonpos={column},{row}\n\
             [{id_b}]\nButtonpos={column},{row}\n"
        )
    }

    fn ability_defaults_with_off_state_txt(
        id_with_off: &str,
        id_without_off: &str,
        column: u8,
        row: u8,
    ) -> String {
        // id_with_off: autocast toggle — command card only after split
        // id_without_off: passive indicator — research panel only after split; give it a
        // Researchbuttonpos so it remains visible somewhere after its Buttonpos is cleared
        format!(
            "[{id_with_off}]\nButtonpos={column},{row}\nUnButtonpos={column},{row}\n\
             [{id_without_off}]\nButtonpos={column},{row}\nResearchbuttonpos={column},0\n"
        )
    }

    fn unit_strings_txt(unit_id: &str) -> String {
        format!("[{unit_id}]\nName=Test Unit\n")
    }

    fn ability_strings_txt(id_a: &str, id_b: &str, shared_name: &str) -> String {
        format!(
            "[{id_a}]\nName={shared_name}\n\
             [{id_b}]\nName={shared_name}\n"
        )
    }

    fn build_database(results: Vec<ExtractResult>) -> WarcraftDatabase {
        let aggregation = WarcraftDataAggregation::from(results);
        WarcraftDatabase::from(aggregation)
    }

    fn unit_ability_ids(database: &WarcraftDatabase, unit_id: &str) -> Vec<String> {
        let object = database
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

    fn unit_hero_ability_ids(database: &WarcraftDatabase, unit_id: &str) -> Vec<String> {
        let object = database
            .by_id(unit_id)
            .unwrap_or_else(|| panic!("unit {unit_id} missing from database"));
        let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
            panic!("{unit_id} is not a Unit");
        };
        unit_meta
            .hero_abilities()
            .iter()
            .map(|ability_id| ability_id.value().to_string())
            .collect()
    }

    fn ability_default_button_position(
        database: &WarcraftDatabase,
        ability_id: &str,
    ) -> Option<GridCoordinate> {
        let object = database.by_id(ability_id)?;
        let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
            return None;
        };
        ability_meta.default_button_position()
    }

    /// Two self-referential abilities at the same position with the same name:
    /// the earlier one is the base ability, the later one is the overlay
    /// replacement.  Rule 5 must keep only the last (OldAb is dropped).
    #[test]
    fn earlier_ability_dropped_when_same_slot_and_name() {
        let unit_slk_text = unit_slk("htst");
        let abilities_slk_text = unit_abilities_slk("htst", "OldAb,NewAb");
        let meta_slk_text = ability_metadata_slk("OldAb", "NewAb");
        let defaults_txt = ability_defaults_txt("OldAb", "NewAb", 2, 2);
        let unit_strings_text = unit_strings_txt("htst");
        let ability_strings_text = ability_strings_txt("OldAb", "NewAb", "Test Ability");

        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, unit_slk_text.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk_text.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_text.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_text.as_bytes())
            .unwrap();

        let database = build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ]);
        let abilities = unit_ability_ids(&database, "htst");
        let has_old = abilities.iter().any(|id| id.eq_ignore_ascii_case("OldAb"));
        let has_new = abilities.iter().any(|id| id.eq_ignore_ascii_case("NewAb"));
        assert!(
            !has_old,
            "OldAb must be suppressed when NewAb is at the same slot (abilities: {abilities:?})",
        );
        assert!(
            has_new,
            "NewAb must be retained as the replacement (abilities: {abilities:?})",
        );
    }

    /// When exactly one of two same-slot, same-name abilities has an off-state button
    /// (autocast toggle), both must be kept in the unit's ability list — they serve
    /// different UI sections.  The toggle appears on the command card; the passive
    /// indicator appears in the research panel.  `split_toggle_passive_positions`
    /// clears the passive's `button_position` (so it doesn't show on the command card)
    /// and the toggle's `research_button_position` (so it doesn't show in research).
    /// This covers Fire Lord (ANia toggle + ANic passive, both named "Incinerate").
    /// Order of abilities in the list does not affect the outcome.
    #[test]
    fn toggle_and_passive_both_kept_with_positions_split() {
        let unit_slk_text = unit_slk("htst");
        // TglAb (toggle) is LAST so "last wins" alone would keep it — but the
        // split must also keep AtoAb (passive) regardless of order.
        let abilities_slk_text = unit_abilities_slk("htst", "AtoAb,TglAb");
        let meta_slk_text = ability_metadata_slk("TglAb", "AtoAb");
        let defaults_txt = ability_defaults_with_off_state_txt("TglAb", "AtoAb", 2, 2);
        let unit_strings_text = unit_strings_txt("htst");
        let ability_strings_text = ability_strings_txt("AtoAb", "TglAb", "Test Passive");

        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, unit_slk_text.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk_text.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_text.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_text.as_bytes())
            .unwrap();

        let database = build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ]);
        let abilities = unit_ability_ids(&database, "htst");
        let has_toggle = abilities.iter().any(|id| id.eq_ignore_ascii_case("TglAb"));
        let has_passive = abilities.iter().any(|id| id.eq_ignore_ascii_case("AtoAb"));
        assert!(
            has_toggle,
            "TglAb (autocast toggle) must be in the ability list (abilities: {abilities:?})",
        );
        assert!(
            has_passive,
            "AtoAb (passive) must be in the ability list — it is not suppressed, \
             only its button_position is cleared (abilities: {abilities:?})",
        );
        // Verify the position split: toggle has a command-card button_position;
        // passive does not (its button_position was cleared by split_toggle_passive_positions).
        let toggle_button_pos = ability_default_button_position(&database, "TglAb");
        let passive_button_pos = ability_default_button_position(&database, "AtoAb");
        assert!(
            toggle_button_pos.is_some(),
            "TglAb must retain its button_position (command card) after the split",
        );
        assert!(
            passive_button_pos.is_none(),
            "AtoAb must have its button_position cleared (research-panel only) after the split",
        );
    }

    /// Abilities at the same position but with DIFFERENT names must NOT be
    /// deduplicated — they serve different purposes (like cascade conflicts
    /// where two abilities happen to share a default slot).
    #[test]
    fn different_names_at_same_position_are_not_deduplicated() {
        let unit_slk_text = unit_slk("htst");
        let abilities_slk_text = unit_abilities_slk("htst", "AbilA,AbilB");
        let meta_slk_text = ability_metadata_slk("AbilA", "AbilB");
        let defaults_txt = ability_defaults_txt("AbilA", "AbilB", 0, 2);
        let unit_strings_text = unit_strings_txt("htst");
        let ability_strings_text =
            "[AbilA]\nName=Devour Magic\n[AbilB]\nName=Absorb Mana\n".to_string();

        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, unit_slk_text.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk_text.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_text.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_text.as_bytes())
            .unwrap();

        let database = build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ]);
        let abilities = unit_ability_ids(&database, "htst");
        let has_a = abilities.iter().any(|id| id.eq_ignore_ascii_case("AbilA"));
        let has_b = abilities.iter().any(|id| id.eq_ignore_ascii_case("AbilB"));
        assert!(
            has_a && has_b,
            "both abilities must survive when they have different names (abilities: {abilities:?})",
        );
    }

    /// Abilities don't need to be self-referential (code == alias) for Rule 5
    /// to apply.  The Archer/Hippogryph case: `OldSelf` is self-referential
    /// (`code=OldSelf`) and `NewAlias` is an alias of a different base mechanic
    /// (`code=BaseMec`).  Both occupy the same default position with the same
    /// display name.  Rule 5 must suppress `OldSelf` because `NewAlias` is the
    /// last occurrence in the CASC-merged list (base before overlay).
    #[test]
    fn self_ref_dropped_in_favour_of_non_self_ref_alias_at_same_slot() {
        let unit_slk_text = unit_slk("htst");
        // OldSelf (base) listed before NewAlias (overlay) — CASC order.
        let abilities_slk_text = unit_abilities_slk("htst", "OldSelf,NewAlias");
        // OldSelf is self-referential; NewAlias has a different base mechanic code.
        let meta_slk_text = "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X1;Y2;K\"OldSelf\"\n\
             C;X2;Y2;K\"OldSelf\"\n\
             C;X1;Y3;K\"NewAlias\"\n\
             C;X2;Y3;K\"BaseMec\"\n\
             E\n"
        .to_string();
        let defaults_txt = ability_defaults_txt("OldSelf", "NewAlias", 0, 2);
        let unit_strings_text = unit_strings_txt("htst");
        let ability_strings_text = ability_strings_txt("OldSelf", "NewAlias", "Mount Test");

        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, unit_slk_text.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk_text.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_text.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_text.as_bytes())
            .unwrap();

        let database = build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ]);
        let abilities = unit_ability_ids(&database, "htst");
        let has_old = abilities
            .iter()
            .any(|id| id.eq_ignore_ascii_case("OldSelf"));
        let has_new = abilities
            .iter()
            .any(|id| id.eq_ignore_ascii_case("NewAlias"));
        assert!(
            !has_old,
            "OldSelf must be suppressed when NewAlias occupies the same slot (abilities: {abilities:?})",
        );
        assert!(
            has_new,
            "NewAlias must be retained as the balance-patch replacement (abilities: {abilities:?})",
        );
    }

    /// Rule 5 applies to hero abilities (heroAbilList) too.  Same-slot
    /// same-name pair in the hero list: the earlier one is dropped.
    #[test]
    fn hero_ability_earlier_dropped_when_same_slot_and_name() {
        // Use a hero unit so hero abilities are populated
        let hero_unit_slk = "ID;P\n\
             C;X1;Y1;K\"unitBalanceID\"\n\
             C;X2;Y1;K\"defType\"\n\
             C;X3;Y1;K\"isbldg\"\n\
             C;X4;Y1;K\"bldtm\"\n\
             C;X1;Y2;K\"Htst\"\n\
             C;X2;Y2;K\"hero\"\n\
             C;X3;Y2;K\"0\"\n\
             C;X4;Y2;K\"55\"\n\
             E\n"
        .to_string();
        let abilities_slk_text = unit_abilities_slk_hero("Htst", "OldHeroAb,NewHeroAb");
        let meta_slk_text = ability_metadata_slk("OldHeroAb", "NewHeroAb");
        let defaults_txt = ability_defaults_txt("OldHeroAb", "NewHeroAb", 3, 2);
        let unit_strings_text = unit_strings_txt("Htst");
        let ability_strings_text =
            ability_strings_txt("OldHeroAb", "NewHeroAb", "Hero Test Ability");

        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, hero_unit_slk.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk_text.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk_text.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_text.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_text.as_bytes())
            .unwrap();

        let database = build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ]);
        let hero_abilities = unit_hero_ability_ids(&database, "Htst");
        let has_old = hero_abilities
            .iter()
            .any(|id| id.eq_ignore_ascii_case("OldHeroAb"));
        let has_new = hero_abilities
            .iter()
            .any(|id| id.eq_ignore_ascii_case("NewHeroAb"));
        assert!(
            !has_old,
            "OldHeroAb must be suppressed in hero abilities (hero_abilities: {hero_abilities:?})",
        );
        assert!(
            has_new,
            "NewHeroAb must be retained in hero abilities (hero_abilities: {hero_abilities:?})",
        );
    }
}

/// Rule 2: a unit reached by transforming from another unit inherits the base
/// form's spells through its own SLK ability list, but those spells belong to
/// the base form and must not appear on the transformed unit.
///
/// Two SLK shapes encode the transform:
///   - "Call to Arms" style: `DataA1` = base unit, `DataB1` = transformed unit
///     (e.g. peasant `hpea` → militia `hmil`).
///   - toggle-morph style: `UnitID1` = morphed unit, no `DataB1` (e.g. Bear
///     Form, Raven Form). The morphed Druid's command card must not show the
///     caster form's Rejuvenation / Cyclone.
///
/// The toggle-morph signal is overloaded — summon abilities also name their
/// summoned unit in `UnitID1`. The distinguishing trait of a real toggle is
/// that the morphed unit carries the morph ability in its own list (to revert);
/// a summoned unit never lists the ability that summons it.
mod rule_2_transform_and_morph_suppression {
    use super::*;

    const UNIT_PATH: &str = "war3.w3mod:units/unitbalance.slk";
    const UNIT_ABILITIES_PATH: &str = "war3.w3mod:units/unitabilities.slk";
    const ABILITY_METADATA_PATH: &str = "war3.w3mod:units/abilitydata.slk";
    const ABILITY_DEFAULTS_PATH: &str = "war3.w3mod:units/humanabilityfunc.txt";
    const UNIT_STRINGS_PATH: &str = "x/enus.w3mod:units/humanunitstrings.txt";
    const ABILITY_STRINGS_PATH: &str = "x/enus.w3mod:units/humanabilitystrings.txt";

    fn build_database(results: Vec<ExtractResult>) -> WarcraftDatabase {
        let aggregation = WarcraftDataAggregation::from(results);
        WarcraftDatabase::from(aggregation)
    }

    fn unit_ability_ids(database: &WarcraftDatabase, unit_id: &str) -> Vec<String> {
        let object = database
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

    fn unit_balance_two(first_id: &str, second_id: &str) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitBalanceID\"\n\
             C;X2;Y1;K\"defType\"\n\
             C;X3;Y1;K\"isbldg\"\n\
             C;X4;Y1;K\"bldtm\"\n\
             C;X1;Y2;K\"{first_id}\"\n\
             C;X2;Y2;K\"normal\"\n\
             C;X3;Y2;K\"0\"\n\
             C;X4;Y2;K\"30\"\n\
             C;X1;Y3;K\"{second_id}\"\n\
             C;X2;Y3;K\"normal\"\n\
             C;X3;Y3;K\"0\"\n\
             C;X4;Y3;K\"30\"\n\
             E\n"
        )
    }

    fn unit_abilities_two(
        first_id: &str,
        first_list: &str,
        second_id: &str,
        second_list: &str,
    ) -> String {
        format!(
            "ID;P\n\
             C;X1;Y1;K\"unitAbilID\"\n\
             C;X2;Y1;K\"abilList\"\n\
             C;X3;Y1;K\"heroAbilList\"\n\
             C;X1;Y2;K\"{first_id}\"\n\
             C;X2;Y2;K\"{first_list}\"\n\
             C;X1;Y3;K\"{second_id}\"\n\
             C;X2;Y3;K\"{second_list}\"\n\
             E\n"
        )
    }

    fn process_all(
        unit_slk: &str,
        abilities_slk: &str,
        meta_slk: &str,
        defaults_txt: &str,
        unit_strings_txt: &str,
        ability_strings_txt: &str,
    ) -> WarcraftDatabase {
        let unit_result = UNITS_EXTRACTION_RULE
            .process(UNIT_PATH, unit_slk.as_bytes())
            .unwrap();
        let abilities_result = UNIT_ABILITIES_EXTRACTION_RULE
            .process(UNIT_ABILITIES_PATH, abilities_slk.as_bytes())
            .unwrap();
        let meta_result = ABILITY_METADATA_EXTRACTION_RULE
            .process(ABILITY_METADATA_PATH, meta_slk.as_bytes())
            .unwrap();
        let defaults_result = ABILITY_DEFAULTS_EXTRACTION_RULE
            .process(ABILITY_DEFAULTS_PATH, defaults_txt.as_bytes())
            .unwrap();
        let unit_strings_result = HUMAN_UNIT_STRINGS_EXTRACTION_RULE
            .process(UNIT_STRINGS_PATH, unit_strings_txt.as_bytes())
            .unwrap();
        let ability_strings_result = HUMAN_ABILITY_STRINGS_EXTRACTION_RULE
            .process(ABILITY_STRINGS_PATH, ability_strings_txt.as_bytes())
            .unwrap();
        build_database(vec![
            unit_result,
            abilities_result,
            meta_result,
            defaults_result,
            unit_strings_result,
            ability_strings_result,
        ])
    }

    /// Bear/Raven Form analogue: `hmrp` is the morphed form. Its SLK list
    /// inherits the base spell `Asrc` from the caster form `hbas`, but `Asrc`
    /// belongs to the base form. After suppression the morphed unit keeps the
    /// morph toggle (`Amrp`) and its own form-specific spell (`Adst`), and drops
    /// the inherited `Asrc`.
    #[test]
    fn morph_form_drops_inherited_base_ability() {
        let unit_slk = unit_balance_two("hbas", "hmrp");
        let abilities_slk = unit_abilities_two("hbas", "Amrp,Asrc", "hmrp", "Amrp,Asrc,Adst");
        let meta_slk = "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X3;Y1;K\"UnitID1\"\n\
             C;X1;Y2;K\"Amrp\"\n\
             C;X2;Y2;K\"Amrp\"\n\
             C;X3;Y2;K\"hmrp\"\n\
             C;X1;Y3;K\"Asrc\"\n\
             C;X2;Y3;K\"Asrc\"\n\
             C;X1;Y4;K\"Adst\"\n\
             C;X2;Y4;K\"Adst\"\n\
             E\n";
        let defaults_txt = "[Amrp]\nButtonpos=3,2\n[Asrc]\nButtonpos=0,0\n[Adst]\nButtonpos=1,0\n";
        let unit_strings_txt = "[hbas]\nName=Caster Form\n[hmrp]\nName=Morphed Form\n";
        let ability_strings_txt =
            "[Amrp]\nName=Morph Form\n[Asrc]\nName=Source Spell\n[Adst]\nName=Morph Spell\n";

        let database = process_all(
            &unit_slk,
            &abilities_slk,
            meta_slk,
            defaults_txt,
            unit_strings_txt,
            ability_strings_txt,
        );
        let abilities = unit_ability_ids(&database, "hmrp");
        let has_inherited = abilities.iter().any(|id| id.eq_ignore_ascii_case("Asrc"));
        let has_toggle = abilities.iter().any(|id| id.eq_ignore_ascii_case("Amrp"));
        let has_form_spell = abilities.iter().any(|id| id.eq_ignore_ascii_case("Adst"));
        assert!(
            !has_inherited,
            "morphed unit must drop the base form's spell Asrc (abilities: {abilities:?})",
        );
        assert!(
            has_toggle,
            "morphed unit must keep the morph toggle Amrp (abilities: {abilities:?})",
        );
        assert!(
            has_form_spell,
            "morphed unit must keep its own spell Adst (abilities: {abilities:?})",
        );
    }

    /// A summoned unit also names itself in the summon ability's `UnitID1`, but
    /// it does not carry that ability in its own list. Suppression must not fire,
    /// so a spell the summoned unit genuinely shares with the summoner survives.
    #[test]
    fn summoned_unit_keeps_shared_ability() {
        let unit_slk = unit_balance_two("hsmn", "hwat");
        let abilities_slk = unit_abilities_two("hsmn", "Asum,Asrc", "hwat", "Asrc");
        let meta_slk = "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X3;Y1;K\"UnitID1\"\n\
             C;X1;Y2;K\"Asum\"\n\
             C;X2;Y2;K\"Asum\"\n\
             C;X3;Y2;K\"hwat\"\n\
             C;X1;Y3;K\"Asrc\"\n\
             C;X2;Y3;K\"Asrc\"\n\
             E\n";
        let defaults_txt = "[Asum]\nButtonpos=3,2\n[Asrc]\nButtonpos=0,0\n";
        let unit_strings_txt = "[hsmn]\nName=Summoner\n[hwat]\nName=Summoned\n";
        let ability_strings_txt = "[Asum]\nName=Summon\n[Asrc]\nName=Shared Spell\n";

        let database = process_all(
            &unit_slk,
            &abilities_slk,
            meta_slk,
            defaults_txt,
            unit_strings_txt,
            ability_strings_txt,
        );
        let abilities = unit_ability_ids(&database, "hwat");
        let has_shared = abilities.iter().any(|id| id.eq_ignore_ascii_case("Asrc"));
        assert!(
            has_shared,
            "summoned unit must keep its shared spell Asrc — it is not a morph (abilities: {abilities:?})",
        );
    }

    /// Existing "Call to Arms" path (`DataA1` base, `DataB1` transformed): the
    /// transformed unit `htrn` inherits the base spell `Ahrv` from `hbse` and
    /// must drop it, keeping only its own spell `Aown`.
    #[test]
    fn call_to_arms_target_drops_inherited_base_ability() {
        let unit_slk = unit_balance_two("hbse", "htrn");
        let abilities_slk = unit_abilities_two("hbse", "Acta,Ahrv", "htrn", "Ahrv,Aown");
        let meta_slk = "ID;P\n\
             C;X1;Y1;K\"alias\"\n\
             C;X2;Y1;K\"code\"\n\
             C;X3;Y1;K\"DataA1\"\n\
             C;X4;Y1;K\"DataB1\"\n\
             C;X1;Y2;K\"Acta\"\n\
             C;X2;Y2;K\"Acta\"\n\
             C;X3;Y2;K\"hbse\"\n\
             C;X4;Y2;K\"htrn\"\n\
             C;X1;Y3;K\"Ahrv\"\n\
             C;X2;Y3;K\"Ahrv\"\n\
             C;X1;Y4;K\"Aown\"\n\
             C;X2;Y4;K\"Aown\"\n\
             E\n";
        let defaults_txt = "[Acta]\nButtonpos=3,2\n[Ahrv]\nButtonpos=0,0\n[Aown]\nButtonpos=1,0\n";
        let unit_strings_txt = "[hbse]\nName=Base Form\n[htrn]\nName=Transformed Form\n";
        let ability_strings_txt =
            "[Acta]\nName=Call To Arms\n[Ahrv]\nName=Harvest\n[Aown]\nName=Own Spell\n";

        let database = process_all(
            &unit_slk,
            &abilities_slk,
            meta_slk,
            defaults_txt,
            unit_strings_txt,
            ability_strings_txt,
        );
        let abilities = unit_ability_ids(&database, "htrn");
        let has_inherited = abilities.iter().any(|id| id.eq_ignore_ascii_case("Ahrv"));
        let has_own = abilities.iter().any(|id| id.eq_ignore_ascii_case("Aown"));
        assert!(
            !has_inherited,
            "transformed unit must drop the base form's spell Ahrv (abilities: {abilities:?})",
        );
        assert!(
            has_own,
            "transformed unit must keep its own spell Aown (abilities: {abilities:?})",
        );
    }
}
