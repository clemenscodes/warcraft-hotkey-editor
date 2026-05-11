use std::fmt;

use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

use crate::custom_keys::CustomKeys;
use crate::grid::layout::GridLayout;
use crate::unit::grids::{HotkeyCollisionCard, PositionCollisionCard, UnitGrids};
use crate::unit::slots::UnitCommandSlots;

#[derive(Debug)]
pub struct UnitCollisionReport {
    entries: Vec<UnitCollisionEntry>,
}

impl PartialEq for UnitCollisionReport {
    fn eq(&self, other: &Self) -> bool {
        self.entries == other.entries
    }
}

#[derive(Clone, Copy, Debug)]
pub struct UnitCollisionEntry {
    unit_id: WarcraftObjectId,
    unit_name: &'static str,
    position_cards: [PositionCollisionCard; 2],
    hotkey_cards: [HotkeyCollisionCard; 2],
}

impl UnitCollisionEntry {
    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn unit_name(&self) -> &'static str {
        self.unit_name
    }

    pub fn position_cards(&self) -> [PositionCollisionCard; 2] {
        self.position_cards
    }

    pub fn hotkey_cards(&self) -> [HotkeyCollisionCard; 2] {
        self.hotkey_cards
    }
}

impl PartialEq for UnitCollisionEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.unit_id != other.unit_id || self.unit_name != other.unit_name {
            return false;
        }
        let self_position_cards: Vec<PositionCollisionCard> = self
            .position_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        let other_position_cards: Vec<PositionCollisionCard> = other
            .position_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        if self_position_cards != other_position_cards {
            return false;
        }
        let self_hotkey_cards: Vec<HotkeyCollisionCard> = self
            .hotkey_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        let other_hotkey_cards: Vec<HotkeyCollisionCard> = other
            .hotkey_cards
            .iter()
            .filter(|card| !card.is_empty())
            .copied()
            .collect();
        self_hotkey_cards == other_hotkey_cards
    }
}

impl UnitCollisionReport {
    pub fn compute(custom_keys: &CustomKeys, layout: GridLayout) -> Self {
        let mut entries: Vec<UnitCollisionEntry> = WARCRAFT_DATABASE
            .all_unit_ids()
            .filter_map(|unit_id| {
                let unit_name = WARCRAFT_DATABASE
                    .by_id(unit_id.value())
                    .and_then(|object| object.names().first().copied())
                    .filter(|name| !name.is_empty())?;
                let unit_grids = UnitGrids::for_unit(unit_id);
                let position_cards = unit_grids.position_collisions(custom_keys);
                let hotkey_cards = unit_grids.hotkey_collisions(custom_keys, layout);
                let no_position_collisions = position_cards.iter().all(|card| card.is_empty());
                let no_hotkey_collisions = hotkey_cards.iter().all(|card| card.is_empty());
                if no_position_collisions && no_hotkey_collisions {
                    return None;
                }
                Some(UnitCollisionEntry {
                    unit_id,
                    unit_name,
                    position_cards,
                    hotkey_cards,
                })
            })
            .collect();

        entries.sort_by(|left, right| {
            left.unit_name
                .cmp(right.unit_name)
                .then_with(|| left.unit_id.value().cmp(right.unit_id.value()))
        });

        Self { entries }
    }

    pub fn entries(&self) -> &[UnitCollisionEntry] {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn for_unit(&self, unit_id: &str) -> Self {
        let entries = self
            .entries
            .iter()
            .filter(|entry| entry.unit_id().value().eq_ignore_ascii_case(unit_id))
            .copied()
            .collect();
        Self { entries }
    }
}

impl fmt::Display for UnitCollisionReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.entries.is_empty() {
            return writeln!(formatter, "No collisions.");
        }
        for entry in &self.entries {
            writeln!(
                formatter,
                "{} ({}):",
                entry.unit_name(),
                entry.unit_id().value()
            )?;
            for card in entry.position_cards() {
                for (position, slots) in card {
                    let slot_list: Vec<&str> = slots.iter().map(|slot| slot.as_str()).collect();
                    let column = u8::from(position.column());
                    let row = u8::from(position.row());
                    writeln!(
                        formatter,
                        "  position ({column},{row}) {:?}  {}",
                        card.role(),
                        slot_list.join(", ")
                    )?;
                }
            }
            for card in entry.hotkey_cards() {
                for (_, entry) in card {
                    let slot_list: Vec<&str> =
                        entry.slots().iter().map(|slot| slot.as_str()).collect();
                    writeln!(
                        formatter,
                        "  hotkey {} {:?}  {}",
                        entry.token(),
                        card.role(),
                        slot_list.join(", ")
                    )?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod unit_collision_report_tests {
    use super::*;
    use crate::identity::slot::GridSlotId;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};
    use crate::unit::grids::{GridRole, HotkeyCollisionCardBuilder, PositionCollisionCardBuilder};
    use warcraft_api::WarcraftObjectId;

    struct UnitCollisionReportBuilder {
        entries: Vec<UnitCollisionEntry>,
    }

    impl UnitCollisionReportBuilder {
        fn new() -> Self {
            Self {
                entries: Vec::new(),
            }
        }

        fn entry(mut self, entry: UnitCollisionEntry) -> Self {
            self.entries.push(entry);
            self
        }

        fn build(self) -> UnitCollisionReport {
            UnitCollisionReport {
                entries: self.entries,
            }
        }
    }

    struct UnitCollisionEntryBuilder {
        unit_id: WarcraftObjectId,
        unit_name: &'static str,
        position_cards: [PositionCollisionCard; 2],
        hotkey_cards: [HotkeyCollisionCard; 2],
    }

    impl UnitCollisionEntryBuilder {
        fn new(
            unit_id: &'static str,
            unit_name: &'static str,
            empty_pos: PositionCollisionCard,
            empty_hot: HotkeyCollisionCard,
        ) -> Self {
            Self {
                unit_id: WarcraftObjectId::new(unit_id),
                unit_name,
                position_cards: [empty_pos, empty_pos],
                hotkey_cards: [empty_hot, empty_hot],
            }
        }

        fn main_position_card(mut self, card: PositionCollisionCard) -> Self {
            self.position_cards[0] = card;
            self
        }

        fn main_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[0] = card;
            self
        }

        fn secondary_hotkey_card(mut self, card: HotkeyCollisionCard) -> Self {
            self.hotkey_cards[1] = card;
            self
        }

        fn build(self) -> UnitCollisionEntry {
            UnitCollisionEntry {
                unit_id: self.unit_id,
                unit_name: self.unit_name,
                position_cards: self.position_cards,
                hotkey_cards: self.hotkey_cards,
            }
        }
    }

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    #[test]
    fn paladin_has_no_collisions_in_normalized_default() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_filtered = report.for_unit("Hpal");
        assert!(
            paladin_filtered.is_empty(),
            "Paladin must have no collisions in the normalized default on QWERTY"
        );
    }

    #[test]
    fn detects_position_collision_across_all_units() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a position"
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.position_cards().iter().any(|card| !card.is_empty()),
            "position collision card must be present for Paladin"
        );
        let has_collision_at_position = entry
            .position_cards()
            .iter()
            .any(|card| card.collision_at(shared_position).is_some());
        assert!(
            has_collision_at_position,
            "collision must be reported at the shared position"
        );
    }

    #[test]
    fn detects_hotkey_collision_across_all_units() {
        let hotkey_q = Hotkey::Letter('Q');
        let holy_light_binding = AbilityBinding::builder().hotkey(hotkey_q).build();
        let divine_shield_binding = AbilityBinding::builder().hotkey(hotkey_q).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let paladin_entry = report
            .entries()
            .iter()
            .find(|entry| entry.unit_id() == paladin_id());
        assert!(
            paladin_entry.is_some(),
            "Paladin must appear in collision report when two abilities share a hotkey"
        );
        let entry = paladin_entry.unwrap();
        assert!(
            entry.hotkey_cards().iter().any(|card| !card.is_empty()),
            "hotkey collision card must be present for Paladin"
        );
    }

    #[test]
    fn for_unit_filters_to_matching_unit() {
        let shared_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Two);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("Hpal");
        assert!(
            filtered
                .entries()
                .iter()
                .all(|entry| entry.unit_id() == paladin_id()),
            "for_unit must return only entries for the requested unit"
        );
    }

    #[test]
    fn for_unit_returns_empty_for_unknown_unit() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let filtered = report.for_unit("ZZZZ");
        assert!(
            filtered.is_empty(),
            "unknown unit id must yield empty report"
        );
    }

    #[test]
    fn entries_are_sorted_by_unit_name() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);
        let names: Vec<&str> = report.entries().iter().map(|e| e.unit_name()).collect();
        let mut sorted_names = names.clone();
        sorted_names.sort();
        assert_eq!(names, sorted_names, "entries must be sorted by unit name");
    }

    #[test]
    fn default_customkeys_collision_matches_expected() {
        let join_handle = std::thread::Builder::new()
            .stack_size(32 * 1024 * 1024)
            .spawn(run_default_collision_check)
            .unwrap();
        join_handle.join().unwrap();
    }

    fn run_default_collision_check() {
        let template_text = include_str!("../../../hotkey-editor/templates/CustomKeys.txt");
        let custom_keys = CustomKeys::from(template_text).normalize();
        let layout = GridLayout::qwerty_grid();
        let report = UnitCollisionReport::compute(&custom_keys, layout);

        let empty_pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
        let empty_pos = empty_pos_builder.build();
        let empty_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
        let empty_hot = empty_hot_builder.build();

        let mut builder = UnitCollisionReportBuilder::new();

        // Ancient Hydra (nahy): position (1,2) MainCommand  Awrh, Aspo
        //                        hotkey X MainCommand  Awrh, Aspo
        let entry = {
            let slot1 = GridSlotId::ability("Awrh");
            let slot2 = GridSlotId::ability("Aspo");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('X', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nahy", "Ancient Hydra", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ancient Sasquatch (nsqa): position (1,2) MainCommand  ACfr, ACtc
        //                            hotkey X MainCommand  ACfr, ACtc
        let entry = {
            let slot1 = GridSlotId::ability("ACfr");
            let slot2 = GridSlotId::ability("ACtc");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('X', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nsqa", "Ancient Sasquatch", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ancient of Wonders (eden): position (3,2) MainCommand  Anei, Aro1
        let entry = {
            let slot1 = GridSlotId::ability("Anei");
            let slot2 = GridSlotId::ability("Aro1");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(3, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("eden", "Ancient of Wonders", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Avatar of Vengeance (espv): hotkey V MainCommand  Avng, ACrk
        let entry = {
            let slot1 = GridSlotId::ability("Avng");
            let slot2 = GridSlotId::ability("ACrk");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('V', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("espv", "Avatar of Vengeance", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Beastmaster (Orex): hotkey S MainCommand  CmdStop, Aamk
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("Aamk");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Orex", "Beastmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Blue Dragon (nadr): position (0,2) MainCommand  Afrc, ACdv
        let entry = {
            let slot1 = GridSlotId::ability("Afrc");
            let slot2 = GridSlotId::ability("ACdv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nadr", "Blue Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Blue Dragonspawn Overseer (nbdo): position (2,2) MainCommand  ACav, ACev
        //                                    hotkey C MainCommand  ACav, ACev
        let entry = {
            let slot1 = GridSlotId::ability("ACav");
            let slot2 = GridSlotId::ability("ACev");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nbdo",
                "Blue Dragonspawn Overseer",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Brewmaster (Nsjs): hotkey S MainCommand  CmdStop, Aamk
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("Aamk");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Nsjs", "Brewmaster", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Brood Mother (nsbm): position (0,2) MainCommand  ACvs, ACen
        //                       hotkey Z MainCommand  ACvs, ACen
        let entry = {
            let slot1 = GridSlotId::ability("ACvs");
            let slot2 = GridSlotId::ability("ACen");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nsbm", "Brood Mother", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Centaur Sorcerer (ncks): position (0,2) MainCommand  ACdm, ACbl
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("ACbl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("ncks", "Centaur Sorcerer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Chaplain (nchp): position (0,2) MainCommand  Adsm, Anh2
        let entry = {
            let slot1 = GridSlotId::ability("Adsm");
            let slot2 = GridSlotId::ability("Anh2");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nchp", "Chaplain", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Corrupted Tree of Ages (ncta): position (0,2) MainCommand  Aeat, ncte
        let entry = {
            let slot1 = GridSlotId::ability("Aeat");
            let slot2 = GridSlotId::ability("ncte");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "ncta",
                "Corrupted Tree of Ages",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Corrupted Tree of Life (nctl): position (0,2) MainCommand  Aeat, ncta
        let entry = {
            let slot1 = GridSlotId::ability("Aeat");
            let slot2 = GridSlotId::ability("ncta");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nctl",
                "Corrupted Tree of Life",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dark Troll High Priest (ndth): position (0,2) MainCommand  Anh2, ACdm
        let entry = {
            let slot1 = GridSlotId::ability("Anh2");
            let slot2 = GridSlotId::ability("ACdm");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "ndth",
                "Dark Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dark Wizard (nwzd): position (2,2) MainCommand  ACpy, ACba
        //                      hotkey C MainCommand  ACpy, ACba
        let entry = {
            let slot1 = GridSlotId::ability("ACpy");
            let slot2 = GridSlotId::ability("ACba");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nwzd", "Dark Wizard", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Death Knight (Udea): hotkey C MainCommand  AUdc, AUau
        let entry = {
            let slot1 = GridSlotId::ability("AUdc");
            let slot2 = GridSlotId::ability("AUau");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Udea", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Death Knight (Uear): hotkey C MainCommand  AUdc, AUau
        let entry = {
            let slot1 = GridSlotId::ability("AUdc");
            let slot2 = GridSlotId::ability("AUau");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Uear", "Death Knight", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Death Revenant (nrvd): position (0,2) MainCommand  ACdc, ACrd
        let entry = {
            let slot1 = GridSlotId::ability("ACdc");
            let slot2 = GridSlotId::ability("ACrd");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nrvd", "Death Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Demigod (Ecen): hotkey S MainCommand  CmdStop, SCc1
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("SCc1");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Ecen", "Demigod", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Demon Hunter (Eevi): position (2,0) MainCommand  CmdHoldPos, ANcl
        let entry = {
            let slot1 = GridSlotId::ability("CmdHoldPos");
            let slot2 = GridSlotId::ability("ANcl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 0, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Eevi", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Demon Hunter (Eevm): position (2,0) MainCommand  CmdHoldPos, ANcl
        let entry = {
            let slot1 = GridSlotId::ability("CmdHoldPos");
            let slot2 = GridSlotId::ability("ANcl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 0, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Eevm", "Demon Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Destroyer (ubsp): position (0,2) MainCommand  Aabs, Advm
        let entry = {
            let slot1 = GridSlotId::ability("Aabs");
            let slot2 = GridSlotId::ability("Advm");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ubsp", "Destroyer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Doom Guard (nba2): hotkey F MainCommand  ACsk, ACrf
        let entry = {
            let slot1 = GridSlotId::ability("ACsk");
            let slot2 = GridSlotId::ability("ACrf");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('F', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nba2", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Doom Guard (nbal): hotkey F MainCommand  ACsk, ACrf
        let entry = {
            let slot1 = GridSlotId::ability("ACsk");
            let slot2 = GridSlotId::ability("ACrf");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('F', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nbal", "Doom Guard", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Draenei Seer (ndrs): position (0,2) MainCommand  AChv, ACsw
        let entry = {
            let slot1 = GridSlotId::ability("AChv");
            let slot2 = GridSlotId::ability("ACsw");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ndrs", "Draenei Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Ubal): hotkey E MainCommand  AUsl, AOeq
        //                    hotkey E HeroSkillTree  AUsl, AOeq
        let entry = {
            let slot1 = GridSlotId::ability("AUsl");
            let slot2 = GridSlotId::ability("AOeq");
            let slots = [slot1, slot2];
            let main_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let main_hot_builder = main_hot_builder.collision('E', &slots);
            let main_hot = main_hot_builder.build();
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('E', &slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Ubal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Udre): hotkey C MainCommand  AUcs, AUav
        let entry = {
            let slot1 = GridSlotId::ability("AUcs");
            let slot2 = GridSlotId::ability("AUav");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Udre", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Udth): hotkey D MainCommand  AEsh, AUdd
        //                    hotkey D HeroSkillTree  AEsh, AUdd
        let entry = {
            let slot1 = GridSlotId::ability("AEsh");
            let slot2 = GridSlotId::ability("AUdd");
            let slots = [slot1, slot2];
            let main_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let main_hot_builder = main_hot_builder.collision('D', &slots);
            let main_hot = main_hot_builder.build();
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('D', &slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Udth", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Umal): hotkey C MainCommand  AUcs, ANdc
        //                    hotkey E HeroSkillTree  AUsl, ANdc
        let entry = {
            let slot_aucs = GridSlotId::ability("AUcs");
            let slot_andc = GridSlotId::ability("ANdc");
            let main_slots = [slot_aucs, slot_andc];
            let main_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let main_hot_builder = main_hot_builder.collision('C', &main_slots);
            let main_hot = main_hot_builder.build();
            let slot_ausl = GridSlotId::ability("AUsl");
            let hero_slots = [slot_ausl, slot_andc];
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('E', &hero_slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Umal", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Utic): position (1,2) MainCommand  ANrc, AUsl
        let entry = {
            let slot1 = GridSlotId::ability("ANrc");
            let slot2 = GridSlotId::ability("AUsl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Utic", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Dreadlord (Uvng): hotkey C MainCommand  AUcs, AUav
        let entry = {
            let slot1 = GridSlotId::ability("AUcs");
            let slot2 = GridSlotId::ability("AUav");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Uvng", "Dreadlord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Earth (npn3): position (0,2) MainCommand  ANta, ACpv
        let entry = {
            let slot1 = GridSlotId::ability("ANta");
            let slot2 = GridSlotId::ability("ACpv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("npn3", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Earth (npn6): position (0,2) MainCommand  ANta, ACpv
        let entry = {
            let slot1 = GridSlotId::ability("ANta");
            let slot2 = GridSlotId::ability("ACpv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("npn6", "Earth", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Elder Voidwalker (nvde): position (0,2) MainCommand  ACde, ACfl
        //                           hotkey Z MainCommand  ACde, ACfl
        let entry = {
            let slot1 = GridSlotId::ability("ACde");
            let slot2 = GridSlotId::ability("ACfl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nvde", "Elder Voidwalker", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Entangled Gold Mine (egol): position (1,2) MainCommand  Adri, Aenc
        let entry = {
            let slot1 = GridSlotId::ability("Adri");
            let slot2 = GridSlotId::ability("Aenc");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("egol", "Entangled Gold Mine", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Eredar Diabolist (nerd): position (0,2) MainCommand  ANfb, ACpa
        //                           hotkey Z MainCommand  ANfb, ACpa
        let entry = {
            let slot1 = GridSlotId::ability("ANfb");
            let slot2 = GridSlotId::ability("ACpa");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nerd", "Eredar Diabolist", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Faceless One Terror (nfot): position (1,2) MainCommand  ACmf, ACsl
        //                              hotkey X MainCommand  ACmf, ACsl
        let entry = {
            let slot1 = GridSlotId::ability("ACmf");
            let slot2 = GridSlotId::ability("ACsl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('X', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nfot", "Faceless One Terror", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Faceless One Trickster (nfor): position (0,2) MainCommand  ACpu, ACcs
        let entry = {
            let slot1 = GridSlotId::ability("ACpu");
            let slot2 = GridSlotId::ability("ACcs");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfor",
                "Faceless One Trickster",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Fel Ravager (npfm): position (0,2) MainCommand  ACde, ACbk
        //                      hotkey Z MainCommand  ACde, ACbk
        let entry = {
            let slot1 = GridSlotId::ability("ACde");
            let slot2 = GridSlotId::ability("ACbk");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("npfm", "Fel Ravager", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Forest Troll Shadow Priest (nfsp): position (0,2) MainCommand  ACdm, Anh1
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("Anh1");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfsp",
                "Forest Troll Shadow Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Forgotten One (nfgo): position (0,2) MainCommand  ACtn, ACfb
        //                        hotkey Z MainCommand  ACtn, ACfb
        let entry = {
            let slot1 = GridSlotId::ability("ACtn");
            let slot2 = GridSlotId::ability("ACfb");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nfgo", "Forgotten One", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Giant Spider (nsgt): position (0,2) MainCommand  ACvs, ACen
        //                       hotkey Z MainCommand  ACvs, ACen
        let entry = {
            let slot1 = GridSlotId::ability("ACvs");
            let slot2 = GridSlotId::ability("ACen");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nsgt", "Giant Spider", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Gnoll Warlord (ngow): position (0,2) MainCommand  ACro, ACbl
        //                        hotkey Z MainCommand  ACro, ACbl
        let entry = {
            let slot1 = GridSlotId::ability("ACro");
            let slot2 = GridSlotId::ability("ACbl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ngow", "Gnoll Warlord", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Goblin Laboratory (ngad): position (0,0) MainCommand  ngsp, nzep, Andt
        let entry = {
            let slot1 = GridSlotId::ability("ngsp");
            let slot2 = GridSlotId::ability("nzep");
            let slot3 = GridSlotId::ability("Andt");
            let slots = [slot1, slot2, slot3];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 0, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("ngad", "Goblin Laboratory", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Goblin Merchant (ngme): hotkey E MainCommand  bspd, stel
        let entry = {
            let slot1 = GridSlotId::ability("bspd");
            let slot2 = GridSlotId::ability("stel");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('E', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("ngme", "Goblin Merchant", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Heretic (nhhr): position (0,2) MainCommand  ACca, ACrd
        let entry = {
            let slot1 = GridSlotId::ability("ACca");
            let slot2 = GridSlotId::ability("ACrd");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nhhr", "Heretic", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // High Elven Barracks (nheb): hotkey D MainCommand  nws1, Rhde
        let entry = {
            let slot1 = GridSlotId::ability("nws1");
            let slot2 = GridSlotId::ability("Rhde");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('D', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nheb", "High Elven Barracks", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ice Troll High Priest (nith): position (1,2) MainCommand  ACd2, ACf2
        let entry = {
            let slot1 = GridSlotId::ability("ACd2");
            let slot2 = GridSlotId::ability("ACf2");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(1, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nith",
                "Ice Troll High Priest",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Kobold Geomancer (nkog): position (0,2) MainCommand  ACdm, ACsw
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("ACsw");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nkog", "Kobold Geomancer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Lightning Revenant (nrvl): position (0,2) MainCommand  ACcl, ACpu
        let entry = {
            let slot1 = GridSlotId::ability("ACcl");
            let slot2 = GridSlotId::ability("ACpu");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nrvl", "Lightning Revenant", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Meat Wagon (umtw): hotkey C MainCommand  Amel, Apts
        let entry = {
            let slot1 = GridSlotId::ability("Amel");
            let slot2 = GridSlotId::ability("Apts");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("umtw", "Meat Wagon", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Mercenary Camp (nmr4): position (0,0) MainCommand  ncea, ncen
        //                         hotkey W MainCommand  nhrw, nqbh
        let entry = {
            let pos_slot1 = GridSlotId::ability("ncea");
            let pos_slot2 = GridSlotId::ability("ncen");
            let pos_slots = [pos_slot1, pos_slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 0, &pos_slots);
            let pos_card = pos_builder.build();
            let hot_slot1 = GridSlotId::ability("nhrw");
            let hot_slot2 = GridSlotId::ability("nqbh");
            let hot_slots = [hot_slot1, hot_slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('W', &hot_slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nmr4", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Mercenary Camp (nmrd): position (0,0) MainCommand  ntkh, nbdw, nubw
        let entry = {
            let slot1 = GridSlotId::ability("ntkh");
            let slot2 = GridSlotId::ability("nbdw");
            let slot3 = GridSlotId::ability("nubw");
            let slots = [slot1, slot2, slot3];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 0, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nmrd", "Mercenary Camp", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Mountain King (Hmbr): hotkey C MainCommand  AHtc, AHbh
        let entry = {
            let slot1 = GridSlotId::ability("AHtc");
            let slot2 = GridSlotId::ability("AHbh");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Hmbr", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Mountain King (Hmkg): hotkey C MainCommand  AHtc, AHbh
        let entry = {
            let slot1 = GridSlotId::ability("AHtc");
            let slot2 = GridSlotId::ability("AHbh");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Hmkg", "Mountain King", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Mur'gul Snarecaster (nmsn): position (0,2) MainCommand  ACdm, ACsw
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("ACsw");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nmsn", "Mur'gul Snarecaster", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Naga Royal Guard (nnrg): position (0,2) MainCommand  ACcb, ACcv
        //                           hotkey Z MainCommand  ACcb, ACcv
        let entry = {
            let slot1 = GridSlotId::ability("ACcb");
            let slot2 = GridSlotId::ability("ACcv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nnrg", "Naga Royal Guard", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nerubian Queen (nnwq): position (0,2) MainCommand  ACrd, ACca
        let entry = {
            let slot1 = GridSlotId::ability("ACrd");
            let slot2 = GridSlotId::ability("ACca");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nnwq", "Nerubian Queen", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nerubian Seer (nnwr): position (0,2) MainCommand  ACdm, ACrd
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("ACrd");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nnwr", "Nerubian Seer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nerubian Webspinner (nnwl): position (0,2) MainCommand  ACrd, ACwb
        let entry = {
            let slot1 = GridSlotId::ability("ACrd");
            let slot2 = GridSlotId::ability("ACwb");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nnwl", "Nerubian Webspinner", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Nether Dragon (nndr): position (2,2) MainCommand  ACcr, ACmi
        //                        hotkey C MainCommand  ACcr, ACmi
        let entry = {
            let slot1 = GridSlotId::ability("ACcr");
            let slot2 = GridSlotId::ability("ACmi");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nndr", "Nether Dragon", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Phoenix (hphx): position (0,2) MainCommand  Ahpe, Apxf
        //                  hotkey Z MainCommand  Ahpe, Apxf
        let entry = {
            let slot1 = GridSlotId::ability("Ahpe");
            let slot2 = GridSlotId::ability("Apxf");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("hphx", "Phoenix", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Pit Lord (Nman): hotkey C MainCommand  AHtc, ANrn
        //                   hotkey E HeroSkillTree  ANrn, AOeq
        let entry = {
            let main_slot1 = GridSlotId::ability("AHtc");
            let main_slot2 = GridSlotId::ability("ANrn");
            let main_slots = [main_slot1, main_slot2];
            let main_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let main_hot_builder = main_hot_builder.collision('C', &main_slots);
            let main_hot = main_hot_builder.build();
            let hero_slot1 = GridSlotId::ability("ANrn");
            let hero_slot2 = GridSlotId::ability("AOeq");
            let hero_slots = [hero_slot1, hero_slot2];
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('E', &hero_slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Nman", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Pit Lord (Npld): hotkey C MainCommand  AHtc, ANrn
        //                   hotkey E HeroSkillTree  ANrn, AOeq
        let entry = {
            let main_slot1 = GridSlotId::ability("AHtc");
            let main_slot2 = GridSlotId::ability("ANrn");
            let main_slots = [main_slot1, main_slot2];
            let main_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let main_hot_builder = main_hot_builder.collision('C', &main_slots);
            let main_hot = main_hot_builder.build();
            let hero_slot1 = GridSlotId::ability("ANrn");
            let hero_slot2 = GridSlotId::ability("AOeq");
            let hero_slots = [hero_slot1, hero_slot2];
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('E', &hero_slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Npld", "Pit Lord", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(main_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Polar Furbolg Elder Shaman (nfpe): position (0,2) MainCommand  ACfn, AChv
        let entry = {
            let slot1 = GridSlotId::ability("ACfn");
            let slot2 = GridSlotId::ability("AChv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nfpe",
                "Polar Furbolg Elder Shaman",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Priestess of the Moon (Emoo): hotkey C MainCommand  AEst, AEar
        let entry = {
            let slot1 = GridSlotId::ability("AEst");
            let slot2 = GridSlotId::ability("AEar");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "Emoo",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Priestess of the Moon (Etyr): hotkey C MainCommand  AEst, AEar
        let entry = {
            let slot1 = GridSlotId::ability("AEst");
            let slot2 = GridSlotId::ability("AEar");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "Etyr",
                "Priestess of the Moon",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Ranger (Hvwd): hotkey C MainCommand  AEst, AEar
        let entry = {
            let slot1 = GridSlotId::ability("AEst");
            let slot2 = GridSlotId::ability("AEar");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Hvwd", "Ranger", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Salamander Vizier (nslv): position (0,2) MainCommand  Ambd, ACdm, ACbl
        //                            hotkey Z MainCommand  Ambd, ACbl
        let entry = {
            let slot_ambd = GridSlotId::ability("Ambd");
            let slot_acdm = GridSlotId::ability("ACdm");
            let slot_acbl = GridSlotId::ability("ACbl");
            let pos_slots = [slot_ambd, slot_acdm, slot_acbl];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &pos_slots);
            let pos_card = pos_builder.build();
            let hot_slots = [slot_ambd, slot_acbl];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &hot_slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nslv", "Salamander Vizier", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Satyr Soulstealer (nstl): position (0,2) MainCommand  ACrd, Ambd
        let entry = {
            let slot1 = GridSlotId::ability("ACrd");
            let slot2 = GridSlotId::ability("Ambd");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nstl", "Satyr Soulstealer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Sea Giant Behemoth (nsgb): position (0,2) MainCommand  ACtb, ACpv
        let entry = {
            let slot1 = GridSlotId::ability("ACtb");
            let slot2 = GridSlotId::ability("ACpv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgb", "Sea Giant Behemoth", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Sea Giant Hunter (nsgh): position (0,2) MainCommand  ACen, ACpv
        //                           hotkey Z MainCommand  ACen, ACpv
        let entry = {
            let slot1 = GridSlotId::ability("ACen");
            let slot2 = GridSlotId::ability("ACpv");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("nsgh", "Sea Giant Hunter", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Shadow Hunter (Orkn): hotkey S MainCommand  CmdStop, Aamk
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("Aamk");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Orkn", "Shadow Hunter", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Skeletal Orc Champion (nsoc): position (2,2) MainCommand  ACvp, ACcr
        //                                hotkey C MainCommand  ACvp, ACcr
        let entry = {
            let slot1 = GridSlotId::ability("ACvp");
            let slot2 = GridSlotId::ability("ACcr");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsoc",
                "Skeletal Orc Champion",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Snarlmane the Bloodgorger (ngos): position (0,2) MainCommand  ACac, ACbl
        //                                    hotkey Z MainCommand  ACac, ACbl
        let entry = {
            let slot1 = GridSlotId::ability("ACac");
            let slot2 = GridSlotId::ability("ACbl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "ngos",
                "Snarlmane the Bloodgorger",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Spellbreaker (hspt): hotkey C MainCommand  Acmg, Amim
        let entry = {
            let slot1 = GridSlotId::ability("Acmg");
            let slot2 = GridSlotId::ability("Amim");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("hspt", "Spellbreaker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Spirit Walker (ospm): hotkey F MainCommand  ACsk, Acpf
        let entry = {
            let slot1 = GridSlotId::ability("ACsk");
            let slot2 = GridSlotId::ability("Acpf");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('F', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ospm", "Spirit Walker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Spirit Walker (ospw): hotkey F MainCommand  ACsk, Acpf
        let entry = {
            let slot1 = GridSlotId::ability("ACsk");
            let slot2 = GridSlotId::ability("Acpf");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('F', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ospw", "Spirit Walker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Stonemaul Warchief (noga): position (2,2) MainCommand  ACbh, SCae
        //                             hotkey C MainCommand  ACbh, SCae
        let entry = {
            let slot1 = GridSlotId::ability("ACbh");
            let slot2 = GridSlotId::ability("SCae");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(2, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("noga", "Stonemaul Warchief", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Storm (npn2): position (0,2) MainCommand  ANwk, Adsm
        let entry = {
            let slot1 = GridSlotId::ability("ANwk");
            let slot2 = GridSlotId::ability("Adsm");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("npn2", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Storm (npn5): position (0,2) MainCommand  ANwk, Adsm
        let entry = {
            let slot1 = GridSlotId::ability("ANwk");
            let slot2 = GridSlotId::ability("Adsm");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("npn5", "Storm", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Storm Wyrm (nstw): position (0,2) MainCommand  ACdv, ACcl
        let entry = {
            let slot1 = GridSlotId::ability("ACdv");
            let slot2 = GridSlotId::ability("ACcl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nstw", "Storm Wyrm", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Stormreaver Necrolyte (nsrn): position (0,2) MainCommand  ACcl, ACbl
        //                                hotkey Z MainCommand  ACcl, ACbl
        let entry = {
            let slot1 = GridSlotId::ability("ACcl");
            let slot2 = GridSlotId::ability("ACbl");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('Z', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new(
                "nsrn",
                "Stormreaver Necrolyte",
                empty_pos,
                empty_hot,
            );
            let eb = eb.main_position_card(pos_card);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Tauren Chieftain (Ocb2): hotkey S MainCommand  CmdStop, Aamk
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("Aamk");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb =
                UnitCollisionEntryBuilder::new("Ocb2", "Tauren Chieftain", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Tinker (Nrob): hotkey S MainCommand  CmdStop, ANde
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("ANde");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Nrob", "Tinker", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Tuskarr Healer (ntkh): position (0,2) MainCommand  Anh1, ACdm
        let entry = {
            let slot1 = GridSlotId::ability("Anh1");
            let slot2 = GridSlotId::ability("ACdm");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ntkh", "Tuskarr Healer", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Warden (Ewar): hotkey D HeroSkillTree  AIhm, AEsh
        let entry = {
            let slot1 = GridSlotId::ability("AIhm");
            let slot2 = GridSlotId::ability("AEsh");
            let slots = [slot1, slot2];
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('D', &slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Ewar", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Warden (Ewrd): hotkey D HeroSkillTree  AIhm, AEsh
        let entry = {
            let slot1 = GridSlotId::ability("AIhm");
            let slot2 = GridSlotId::ability("AEsh");
            let slots = [slot1, slot2];
            let hero_hot_builder = HotkeyCollisionCardBuilder::new(GridRole::HeroSkillTree, layout);
            let hero_hot_builder = hero_hot_builder.collision('D', &slots);
            let hero_hot = hero_hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Ewrd", "Warden", empty_pos, empty_hot);
            let eb = eb.secondary_hotkey_card(hero_hot);
            eb.build()
        };
        builder = builder.entry(entry);

        // Warlock (Uwar): hotkey S MainCommand  CmdStop, ACm2
        let entry = {
            let slot1 = GridSlotId::ability("CmdStop");
            let slot2 = GridSlotId::ability("ACm2");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('S', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("Uwar", "Warlock", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Watery Minion (nsns): position (0,2) MainCommand  ACdm, ACsw
        let entry = {
            let slot1 = GridSlotId::ability("ACdm");
            let slot2 = GridSlotId::ability("ACsw");
            let slots = [slot1, slot2];
            let pos_builder = PositionCollisionCardBuilder::new(GridRole::MainCommand);
            let pos_builder = pos_builder.collision_at(0, 2, &slots);
            let pos_card = pos_builder.build();
            let eb = UnitCollisionEntryBuilder::new("nsns", "Watery Minion", empty_pos, empty_hot);
            let eb = eb.main_position_card(pos_card);
            eb.build()
        };
        builder = builder.entry(entry);

        // Wraith (ngh2): hotkey C MainCommand  ACcs, ACps
        let entry = {
            let slot1 = GridSlotId::ability("ACcs");
            let slot2 = GridSlotId::ability("ACps");
            let slots = [slot1, slot2];
            let hot_builder = HotkeyCollisionCardBuilder::new(GridRole::MainCommand, layout);
            let hot_builder = hot_builder.collision('C', &slots);
            let hot_card = hot_builder.build();
            let eb = UnitCollisionEntryBuilder::new("ngh2", "Wraith", empty_pos, empty_hot);
            let eb = eb.main_hotkey_card(hot_card);
            eb.build()
        };
        builder = builder.entry(entry);

        let expected = builder.build();
        assert_eq!(
            report, expected,
            "default CustomKeys.txt collision report changed — update the expected entries if intentional"
        );
    }
}
