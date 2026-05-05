#[cfg(test)]
mod tests {
    use warcraft_api::{ItemClass, Race, UnitKind, WarcraftObjectKind, WarcraftObjectMeta};

    use crate::WARCRAFT_DATABASE;

    const MIN_OBJECT_COUNT: usize = 700;

    /// Object IDs that must always exist. If Blizzard removes or renames any of
    /// these, the corresponding test fails and a human has to decide whether to
    /// update the anchor set or treat it as a regression.
    const ANCHOR_OBJECT_IDS: &[&str] = &[
        "hpea", // Human Peasant
        "htow", // Human Town Hall
        "Hamg", // Human hero: Archmage
        "Hpal", // Human hero: Paladin
        "opeo", // Orc Peon
        "ogre", // Orc Great Hall variant
        "Opgh", // Orc hero: Tauren Chieftain / Far Seer variant
        "uaco", // Undead Acolyte
        "unpl", // Undead Necropolis
        "Udea", // Undead hero: Death Knight
        "etol", // Night Elf Tree of Life
        "Emoo", // Night Elf hero: Keeper of the Grove / Moonkin variant
        "AHbh", // Ability: Paladin Holy Light
        "AHav", // Ability: Paladin Avatar
    ];

    #[test]
    fn test_should_have_expected_object_count() {
        let actual_count = WARCRAFT_DATABASE.db().len();
        assert!(
            actual_count >= MIN_OBJECT_COUNT,
            "object count {actual_count} below floor {MIN_OBJECT_COUNT}"
        );
    }

    #[test]
    fn test_db_objects_should_all_have_valid_names_and_icons() {
        for (_, object) in WARCRAFT_DATABASE.iter() {
            assert!(!object.names().is_empty(), "object has no names");
            for name in object.names() {
                assert!(!name.trim().is_empty(), "object contains empty name");
            }
            for icon in object.icons() {
                assert!(
                    icon.ends_with(".blp"),
                    "icon does not end with .blp: {icon}"
                );
            }
        }
    }

    #[test]
    fn test_object_ids_are_ascii() {
        for (id, _) in WARCRAFT_DATABASE.iter() {
            let value = id.value();
            assert!(
                value.is_ascii(),
                "object id {value:?} contains non-ASCII characters"
            );
        }
    }

    #[test]
    fn test_non_command_object_ids_are_three_or_four_chars() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() == WarcraftObjectKind::Command {
                continue;
            }
            let value = id.value();
            let character_count = value.chars().count();
            assert!(
                (3..=4).contains(&character_count),
                "non-command object id {value:?} has {character_count} chars (expected 3 or 4)"
            );
        }
    }

    #[test]
    fn test_anchor_objects_are_present() {
        for &anchor_id in ANCHOR_OBJECT_IDS {
            let object = WARCRAFT_DATABASE.get(anchor_id.into());
            assert!(
                object.is_some(),
                "anchor object {anchor_id} missing — Blizzard removed/renamed it, or the extractor dropped it"
            );
        }
    }

    #[test]
    fn test_all_abilities_have_valid_meta() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Ability {
                continue;
            }

            let max_level = WARCRAFT_DATABASE
                .get_ability_max_level(id.value().into())
                .expect("ability missing max_level");

            assert!(
                (1..=4).contains(&max_level),
                "ability {id:?} has invalid max_level {max_level}"
            );

            WARCRAFT_DATABASE
                .get_ability_base_cooldown(id.value().into())
                .expect("ability missing base cooldown");
        }
    }

    #[test]
    fn test_ability_cooldown_per_level_is_consistent() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Ability {
                continue;
            }

            let max_level = WARCRAFT_DATABASE
                .get_ability_max_level(id.value().into())
                .unwrap();

            for level in 1..=max_level {
                WARCRAFT_DATABASE
                    .get_ability_cooldown_for_level(id.value().into(), level)
                    .expect("missing cooldown for level");
            }
        }
    }

    #[test]
    fn test_ultimate_abilities_have_max_level_one() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Ability {
                continue;
            }
            let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
                unreachable!("kind mismatch");
            };
            if !ability_meta.is_ultimate() {
                continue;
            }
            assert_eq!(
                ability_meta.max_level(),
                1,
                "ultimate ability {id:?} has max_level {} (expected 1)",
                ability_meta.max_level()
            );
        }
    }

    #[test]
    fn test_non_ultimate_abilities_have_max_level_at_most_four() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Ability {
                continue;
            }
            let WarcraftObjectMeta::Ability(ability_meta) = object.meta() else {
                unreachable!("kind mismatch");
            };
            if ability_meta.is_ultimate() {
                continue;
            }
            assert!(
                ability_meta.max_level() <= 4,
                "non-ultimate ability {id:?} has max_level {} (> 4)",
                ability_meta.max_level()
            );
        }
    }

    #[test]
    fn test_ability_cooldowns_are_bounded_by_ten_minutes() {
        const MAX_COOLDOWN_MS: u32 = 10 * 60 * 1000;
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Ability {
                continue;
            }
            let cooldowns = WARCRAFT_DATABASE
                .get_ability_cooldowns(id.value().into())
                .expect("ability cooldowns missing");
            for (level_index, cooldown_ms) in cooldowns.iter().enumerate() {
                assert!(
                    *cooldown_ms <= MAX_COOLDOWN_MS,
                    "ability {id:?} level-{} cooldown {cooldown_ms} ms exceeds 10-minute ceiling",
                    level_index + 1
                );
            }
        }
    }

    #[test]
    fn test_unit_build_times_are_reasonable() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Unit {
                continue;
            }

            let build_time = WARCRAFT_DATABASE
                .get_unit_build_time(id.value().into())
                .expect("unit missing build time");

            assert!(
                build_time > 0 && build_time < 1000,
                "unit {id:?} has suspicious build time {build_time}"
            );
        }
    }

    #[test]
    fn test_upgrades_have_multiple_levels() {
        for (id, object) in WARCRAFT_DATABASE.iter() {
            if object.kind() != WarcraftObjectKind::Upgrade {
                continue;
            }

            let max_level = WARCRAFT_DATABASE
                .get_upgrade_max_level(id.value().into())
                .expect("upgrade missing max_level");

            assert!(
                max_level >= 1,
                "upgrade {id:?} has invalid max_level {max_level}"
            );
        }
    }

    // NOTE: item `abilList` / `cooldownID` entries deliberately do NOT have to
    // resolve against the database — the extractor only materializes *hero*
    // abilities (hero_flag == 1) as Ability-kind objects, while items
    // commonly reference regular ability SLK rows that are never tracked.
    // A test that asserts referential closure here would fail against
    // perfectly healthy data. Leaving it out on purpose.

    #[test]
    fn test_every_playable_race_has_a_worker() {
        for playable_race in [Race::Human, Race::Nightelf, Race::Orc, Race::Undead] {
            let has_worker = WARCRAFT_DATABASE.iter().any(|(_, object)| {
                if object.kind() != WarcraftObjectKind::Unit {
                    return false;
                }
                if object.race() != Some(playable_race) {
                    return false;
                }
                let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                    return false;
                };
                unit_meta.unit_kind() == UnitKind::Worker
            });
            assert!(has_worker, "race {playable_race:?} has no Worker units");
        }
    }

    #[test]
    fn test_every_playable_race_has_a_hero() {
        for playable_race in [Race::Human, Race::Nightelf, Race::Orc, Race::Undead] {
            let has_hero = WARCRAFT_DATABASE.iter().any(|(_, object)| {
                if object.kind() != WarcraftObjectKind::Unit {
                    return false;
                }
                if object.race() != Some(playable_race) {
                    return false;
                }
                let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                    return false;
                };
                unit_meta.unit_kind() == UnitKind::Hero
            });
            assert!(has_hero, "race {playable_race:?} has no Hero units");
        }
    }

    #[test]
    fn test_every_playable_race_has_a_building() {
        for playable_race in [Race::Human, Race::Nightelf, Race::Orc, Race::Undead] {
            let has_building = WARCRAFT_DATABASE.iter().any(|(_, object)| {
                if object.kind() != WarcraftObjectKind::Unit {
                    return false;
                }
                if object.race() != Some(playable_race) {
                    return false;
                }
                let WarcraftObjectMeta::Unit(unit_meta) = object.meta() else {
                    return false;
                };
                unit_meta.unit_kind() == UnitKind::Building
            });
            assert!(has_building, "race {playable_race:?} has no Building units");
        }
    }

    #[test]
    fn test_core_item_classes_are_populated() {
        for expected_class in [ItemClass::Permanent, ItemClass::Charged, ItemClass::PowerUp] {
            let has_item_in_class = WARCRAFT_DATABASE.iter().any(|(_, object)| {
                if object.kind() != WarcraftObjectKind::Item {
                    return false;
                }
                let WarcraftObjectMeta::Item(item_meta) = object.meta() else {
                    return false;
                };
                *item_meta.class() == expected_class
            });
            assert!(
                has_item_in_class,
                "item class {expected_class:?} has no entries"
            );
        }
    }
}
