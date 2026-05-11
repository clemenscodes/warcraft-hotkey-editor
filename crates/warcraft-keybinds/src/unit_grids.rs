use std::collections::HashMap;

use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

use crate::custom_keys::CustomKeys;
use crate::grid_layout::GridLayout;
use crate::hotkey_token::HotkeyToken;
use crate::model::GridCoordinate;
use crate::slot::{CommandCard, GridSlotId};
use crate::unit_slots::UnitCommandSlots;

const GRID_SLOT_COUNT: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GridRole {
    MainCommand,
    HeroSkillTree,
    BuildMenu,
    UprootedForm,
}

impl GridRole {
    pub fn is_research_context(self) -> bool {
        matches!(self, GridRole::HeroSkillTree)
    }
}

pub struct NamedCommandGrid {
    role: GridRole,
    card: CommandCard,
}

impl NamedCommandGrid {
    fn new(role: GridRole, card: CommandCard) -> Self {
        Self { role, card }
    }

    pub fn role(&self) -> GridRole {
        self.role
    }

    pub fn card(&self) -> &CommandCard {
        &self.card
    }
}

pub struct UnitGrids {
    unit_id: WarcraftObjectId,
    grids: Vec<NamedCommandGrid>,
}

impl UnitGrids {
    pub fn for_unit(unit_id: WarcraftObjectId) -> Self {
        let main_card = WARCRAFT_DATABASE.command_card(unit_id);
        let main_grid = NamedCommandGrid::new(GridRole::MainCommand, main_card);
        let mut grids = vec![main_grid];
        if let Some(hero_card) = WARCRAFT_DATABASE.research_menu(unit_id) {
            let hero_grid = NamedCommandGrid::new(GridRole::HeroSkillTree, hero_card);
            grids.push(hero_grid);
        } else if let Some(build_card) = WARCRAFT_DATABASE.build_menu(unit_id) {
            let build_grid = NamedCommandGrid::new(GridRole::BuildMenu, build_card);
            grids.push(build_grid);
        } else if let Some(uprooted_card) = WARCRAFT_DATABASE.uprooted_menu(unit_id) {
            let uprooted_grid = NamedCommandGrid::new(GridRole::UprootedForm, uprooted_card);
            grids.push(uprooted_grid);
        }
        Self { unit_id, grids }
    }

    pub fn unit_id(&self) -> WarcraftObjectId {
        self.unit_id
    }

    pub fn grids(&self) -> &[NamedCommandGrid] {
        &self.grids
    }

    pub fn grid_count(&self) -> usize {
        self.grids.len()
    }

    pub fn position_collisions(&self, custom_keys: &CustomKeys) -> Box<[PositionCollisionCard]> {
        self.grids
            .iter()
            .filter_map(|named_grid| {
                let role = named_grid.role;
                let is_research = role.is_research_context();
                let mut slots_by_position: HashMap<GridCoordinate, Vec<GridSlotId>> =
                    HashMap::new();
                for slot in named_grid.card.filled_slots() {
                    if let Some(position) = custom_keys.position_for_slot(&slot, is_research) {
                        slots_by_position.entry(position).or_default().push(slot);
                    }
                }
                let mut cells: [[Option<CollisionSlots>; 4]; 3] =
                    std::array::from_fn(|_| std::array::from_fn(|_| None));
                let mut has_collision = false;
                for (position, colliding_slots) in slots_by_position {
                    if colliding_slots.len() < 2 {
                        continue;
                    }
                    let row = usize::from(position.row());
                    let column = usize::from(position.column());
                    cells[row][column] = Some(CollisionSlots::new(&colliding_slots));
                    has_collision = true;
                }
                if !has_collision {
                    return None;
                }
                Some(PositionCollisionCard { role, cells })
            })
            .collect()
    }

    pub fn hotkey_collisions(
        &self,
        custom_keys: &CustomKeys,
        layout: GridLayout,
    ) -> Box<[HotkeyCollisionCard]> {
        self.grids
            .iter()
            .filter_map(|named_grid| {
                let role = named_grid.role;
                let is_research = role.is_research_context();
                let mut slots_by_token: HashMap<HotkeyToken, Vec<GridSlotId>> = HashMap::new();
                for slot in named_grid.card.filled_slots() {
                    if let Some(token) =
                        custom_keys.effective_hotkey_token(&slot, layout, is_research)
                    {
                        slots_by_token.entry(token).or_default().push(slot);
                    }
                }
                let mut cells: [[Option<HotkeyCollisionAtCell>; 4]; 3] =
                    std::array::from_fn(|_| std::array::from_fn(|_| None));
                let mut has_collision = false;
                for (token, colliding_slots) in slots_by_token {
                    if colliding_slots.len() < 2 {
                        continue;
                    }
                    let HotkeyToken::Letter { character } = token else {
                        continue;
                    };
                    let Some(position) = layout.position_for_letter(character) else {
                        continue;
                    };
                    let row = usize::from(position.row());
                    let column = usize::from(position.column());
                    let collision_slots = CollisionSlots::new(&colliding_slots);
                    cells[row][column] = Some(HotkeyCollisionAtCell {
                        token,
                        collision_slots,
                    });
                    has_collision = true;
                }
                if !has_collision {
                    return None;
                }
                Some(HotkeyCollisionCard { role, cells })
            })
            .collect()
    }
}

#[derive(Clone, Copy)]
pub struct CollisionSlots {
    slots: [Option<GridSlotId>; GRID_SLOT_COUNT],
    count: u8,
}

impl CollisionSlots {
    fn new(items: &[GridSlotId]) -> Self {
        let mut slots = [None; GRID_SLOT_COUNT];
        for (index, &slot) in items.iter().enumerate().take(GRID_SLOT_COUNT) {
            slots[index] = Some(slot);
        }
        let raw_count = items.len().min(GRID_SLOT_COUNT);
        let count = u8::try_from(raw_count).expect("slot count bounded by 12");
        Self { slots, count }
    }

    pub fn len(&self) -> usize {
        usize::from(self.count)
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = GridSlotId> + '_ {
        let count = usize::from(self.count);
        self.slots[..count].iter().copied().flatten()
    }
}

#[derive(Clone)]
pub struct PositionCollisionCard {
    role: GridRole,
    cells: [[Option<CollisionSlots>; 4]; 3],
}

impl PositionCollisionCard {
    pub fn role(&self) -> GridRole {
        self.role
    }

    pub fn collision_at(&self, position: GridCoordinate) -> Option<CollisionSlots> {
        let row = usize::from(position.row());
        let column = usize::from(position.column());
        self.cells[row][column]
    }

    pub fn is_empty(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_none()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (GridCoordinate, CollisionSlots)> + '_ {
        self.cells.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(column_index, cell)| {
                    let collision_slots = (*cell)?;
                    let row_u8 = u8::try_from(row_index).ok()?;
                    let column_u8 = u8::try_from(column_index).ok()?;
                    let row = crate::model::RowIndex::try_from(row_u8).ok()?;
                    let column = crate::model::ColumnIndex::try_from(column_u8).ok()?;
                    let position = GridCoordinate::new(column, row);
                    Some((position, collision_slots))
                })
        })
    }
}

#[derive(Clone, Copy)]
pub struct HotkeyCollisionAtCell {
    token: HotkeyToken,
    collision_slots: CollisionSlots,
}

impl HotkeyCollisionAtCell {
    pub fn token(&self) -> HotkeyToken {
        self.token
    }

    pub fn slots(&self) -> CollisionSlots {
        self.collision_slots
    }
}

#[derive(Clone)]
pub struct HotkeyCollisionCard {
    role: GridRole,
    cells: [[Option<HotkeyCollisionAtCell>; 4]; 3],
}

impl HotkeyCollisionCard {
    pub fn role(&self) -> GridRole {
        self.role
    }

    pub fn collision_at(&self, position: GridCoordinate) -> Option<HotkeyCollisionAtCell> {
        let row = usize::from(position.row());
        let column = usize::from(position.column());
        self.cells[row][column]
    }

    pub fn is_empty(&self) -> bool {
        self.cells
            .iter()
            .all(|row| row.iter().all(|cell| cell.is_none()))
    }

    pub fn iter(&self) -> impl Iterator<Item = (GridCoordinate, HotkeyCollisionAtCell)> + '_ {
        self.cells.iter().enumerate().flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(column_index, cell)| {
                    let entry = (*cell)?;
                    let row_u8 = u8::try_from(row_index).ok()?;
                    let column_u8 = u8::try_from(column_index).ok()?;
                    let row = crate::model::RowIndex::try_from(row_u8).ok()?;
                    let column = crate::model::ColumnIndex::try_from(column_u8).ok()?;
                    let position = GridCoordinate::new(column, row);
                    Some((position, entry))
                })
        })
    }
}

#[cfg(test)]
mod unit_grids_tests {
    use super::*;
    use crate::custom_keys::CustomKeys;
    use crate::grid_layout::GridLayout;
    use crate::model::{AbilityBinding, ColumnIndex, GridCoordinate, Hotkey, RowIndex};

    fn paladin_id() -> WarcraftObjectId {
        WarcraftObjectId::new("Hpal")
    }

    fn peasant_id() -> WarcraftObjectId {
        WarcraftObjectId::new("hpea")
    }

    fn footman_id() -> WarcraftObjectId {
        WarcraftObjectId::new("hfoo")
    }

    fn tree_of_life_id() -> WarcraftObjectId {
        WarcraftObjectId::new("etol")
    }

    #[test]
    fn regular_unit_has_one_grid() {
        let unit_grids = UnitGrids::for_unit(footman_id());
        assert_eq!(unit_grids.grid_count(), 1);
    }

    #[test]
    fn regular_unit_grid_role_is_main_command() {
        let unit_grids = UnitGrids::for_unit(footman_id());
        let first_grid = &unit_grids.grids()[0];
        assert_eq!(first_grid.role(), GridRole::MainCommand);
    }

    #[test]
    fn hero_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn hero_second_grid_role_is_hero_skill_tree() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::HeroSkillTree);
    }

    #[test]
    fn hero_skill_tree_is_research_context() {
        assert!(GridRole::HeroSkillTree.is_research_context());
    }

    #[test]
    fn main_command_is_not_research_context() {
        assert!(!GridRole::MainCommand.is_research_context());
    }

    #[test]
    fn build_menu_is_not_research_context() {
        assert!(!GridRole::BuildMenu.is_research_context());
    }

    #[test]
    fn uprooted_form_is_not_research_context() {
        assert!(!GridRole::UprootedForm.is_research_context());
    }

    #[test]
    fn worker_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn worker_second_grid_role_is_build_menu() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::BuildMenu);
    }

    #[test]
    fn uprootable_building_has_two_grids() {
        let unit_grids = UnitGrids::for_unit(tree_of_life_id());
        assert_eq!(unit_grids.grid_count(), 2);
    }

    #[test]
    fn uprootable_building_second_grid_role_is_uprooted_form() {
        let unit_grids = UnitGrids::for_unit(tree_of_life_id());
        let second_grid = &unit_grids.grids()[1];
        assert_eq!(second_grid.role(), GridRole::UprootedForm);
    }

    #[test]
    fn hero_skill_tree_is_non_empty() {
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let skill_tree = &unit_grids.grids()[1];
        assert!(!skill_tree.card().is_empty());
    }

    #[test]
    fn worker_build_menu_is_non_empty() {
        let unit_grids = UnitGrids::for_unit(peasant_id());
        let build_menu = &unit_grids.grids()[1];
        assert!(!build_menu.card().is_empty());
    }

    #[test]
    fn unit_grids_exposes_correct_unit_id() {
        let unit_id = paladin_id();
        let unit_grids = UnitGrids::for_unit(unit_id);
        assert_eq!(unit_grids.unit_id(), unit_id);
    }

    #[test]
    fn position_collisions_empty_for_normalized_default() {
        let custom_keys = CustomKeys::from("").normalize();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        assert!(
            cards.is_empty(),
            "normalized default state must have no position collisions for Paladin"
        );
    }

    #[test]
    fn position_collisions_detects_two_abilities_at_same_slot() {
        let collision_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(collision_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        let has_collision = cards
            .iter()
            .any(|card| card.collision_at(collision_position).is_some());
        assert!(
            has_collision,
            "placing two Paladin abilities at (0,0) must produce a position collision"
        );
    }

    #[test]
    fn position_collision_reports_both_slots() {
        let shared_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let holy_light_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let divine_shield_binding = AbilityBinding::builder()
            .button_position(shared_position)
            .build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.position_collisions(&custom_keys);
        let collision = cards
            .iter()
            .find_map(|card| card.collision_at(shared_position))
            .expect("collision at (1,0) must be found");
        let slot_ids: Vec<&str> = collision.iter().map(|slot| slot.as_str()).collect();
        assert!(slot_ids.contains(&"AHhb"), "collision must include AHhb");
        assert!(slot_ids.contains(&"AHds"), "collision must include AHds");
    }

    #[test]
    fn hotkey_collisions_empty_for_normalized_default() {
        let custom_keys = CustomKeys::from("").normalize();
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        assert!(
            cards.is_empty(),
            "normalized default state must have no hotkey collisions for Paladin"
        );
    }

    #[test]
    fn hotkey_collisions_detects_two_abilities_with_same_hotkey() {
        let hotkey_q = Hotkey::from('Q');
        let holy_light_binding = AbilityBinding::builder().hotkey(hotkey_q.clone()).build();
        let divine_shield_binding = AbilityBinding::builder().hotkey(hotkey_q).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        assert!(
            !cards.is_empty(),
            "two Paladin abilities with hotkey Q must produce a hotkey collision"
        );
    }

    #[test]
    fn hotkey_collision_reports_colliding_token() {
        let hotkey_w = Hotkey::from('W');
        let holy_light_binding = AbilityBinding::builder().hotkey(hotkey_w.clone()).build();
        let divine_shield_binding = AbilityBinding::builder().hotkey(hotkey_w).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_binding);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        let w_position = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let w_entry = cards
            .iter()
            .find_map(|card| card.collision_at(w_position))
            .expect("collision at W position must be found");
        assert_eq!(w_entry.slots().len(), 2);
        assert!(
            matches!(w_entry.token(), HotkeyToken::Letter { character } if character == 'W'),
            "collision token must be W"
        );
    }

    #[test]
    fn hotkey_collisions_are_per_grid_not_cross_grid() {
        let hotkey_q = Hotkey::from('Q');
        let holy_light_binding = AbilityBinding::builder().hotkey(hotkey_q.clone()).build();
        let divine_shield_research = AbilityBinding::builder().research_hotkey(hotkey_q).build();
        let mut custom_keys = CustomKeys::from("").normalize();
        custom_keys.put_ability("AHhb", holy_light_binding);
        custom_keys.put_ability("AHds", divine_shield_research);
        let layout = GridLayout::qwerty_grid();
        let unit_grids = UnitGrids::for_unit(paladin_id());
        let cards = unit_grids.hotkey_collisions(&custom_keys, layout);
        let q_position = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let cross_grid_collision = cards.iter().any(|card| {
            card.collision_at(q_position).map_or(false, |entry| {
                let slot_ids: Vec<&str> = entry.slots().iter().map(|slot| slot.as_str()).collect();
                slot_ids.contains(&"AHhb") && slot_ids.contains(&"AHds")
            })
        });
        assert!(
            !cross_grid_collision,
            "same hotkey in main grid and skill tree must not be reported as a collision"
        );
    }
}
