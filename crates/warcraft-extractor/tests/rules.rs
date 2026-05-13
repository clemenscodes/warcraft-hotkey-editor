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

use warcraft_api::{ItemClass, Race, UnitKind};
use warcraft_extractor::{
    ABILITY_DEFAULTS_EXTRACTION_RULE, ABILITY_SKINS_EXTRACTION_RULE,
    CAMPAIGN_ABILITY_STRINGS_EXTRACTION_RULE, CAMPAIGN_UNIT_STRINGS_EXTRACTION_RULE, ExtractResult,
    HEROES_EXTRACTION_RULE, HUMAN_ABILITY_STRINGS_EXTRACTION_RULE,
    HUMAN_UNIT_STRINGS_EXTRACTION_RULE, HUMAN_UPGRADES_ART_EXTRACTION_RULE,
    HUMAN_UPGRADES_NAME_EXTRACTION_RULE, ITEM_ABILITY_STRINGS_EXTRACTION_RULE,
    ITEM_SKINS_EXTRACTION_RULE, ITEM_UNIT_STRINGS_EXTRACTION_RULE, ITEMS_EXTRACTION_RULE,
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
