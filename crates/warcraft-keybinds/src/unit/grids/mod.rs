use crate::custom_keys::CustomKeys;
use crate::grid::layout::GridLayout;
use crate::identity::hotkey_token::HotkeyToken;
use crate::identity::slot::{CommandCard, GridSlotId};
use crate::model::GridCoordinate;
use crate::unit::slots::UnitCommandSlots;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

const GRID_SLOT_COUNT: usize = 12;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum GridRole {
    MainCommand,
    HeroSkillTree,
    BuildMenu,
    UprootedForm,
}

impl GridRole {
    pub fn is_research_context(self) -> bool {
        matches!(self, Self::HeroSkillTree)
    }

    /// Stable sort index for laying out groups when multiple grid roles share
    /// the same `(row, column)` cell.  Lower comes first.
    pub fn sort_index(self) -> u8 {
        match self {
            Self::MainCommand => 0,
            Self::BuildMenu => 1,
            Self::UprootedForm => 2,
            Self::HeroSkillTree => 3,
        }
    }

    /// Short human-readable label used in CLI output and Display formatting.
    pub fn label(self) -> &'static str {
        match self {
            Self::MainCommand => "main command",
            Self::BuildMenu => "build menu",
            Self::UprootedForm => "uprooted",
            Self::HeroSkillTree => "research",
        }
    }
}

impl std::fmt::Display for GridRole {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnitGrids {
    unit_id: WarcraftObjectId,
    grids: Vec<NamedCommandGrid>,
}

static UNIT_GRIDS_CACHE: LazyLock<HashMap<WarcraftObjectId, UnitGrids>> = LazyLock::new(|| {
    let mut cache = HashMap::new();
    for unit_id in WARCRAFT_DATABASE.all_unit_ids() {
        let grids = UnitGrids::build_for_unit(unit_id);
        cache.insert(unit_id, grids);
    }
    cache
});

impl UnitGrids {
    pub fn for_unit(unit_id: WarcraftObjectId) -> Self {
        if let Some(cached) = UNIT_GRIDS_CACHE.get(&unit_id) {
            return cached.clone();
        }
        Self::build_for_unit(unit_id)
    }

    fn build_for_unit(unit_id: WarcraftObjectId) -> Self {
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

    pub fn position_collisions(&self, custom_keys: &CustomKeys) -> [PositionCollisionCard; 2] {
        let empty = PositionCollisionCard {
            role: GridRole::MainCommand,
            cells: [[None; 4]; 3],
        };
        let mut result = [empty, empty];
        for (grid_index, named_grid) in self.grids.iter().enumerate().take(2) {
            let role = named_grid.role;
            let is_research = role.is_research_context();
            let mut slots_by_position: HashMap<GridCoordinate, Vec<GridSlotId>> = HashMap::new();
            for slot in named_grid.card.filled_slots() {
                if let Some(position) = custom_keys.position_for_slot(&slot, is_research) {
                    slots_by_position.entry(position).or_default().push(slot);
                }
            }
            let mut cells: [[Option<CollisionSlots>; 4]; 3] =
                std::array::from_fn(|_| std::array::from_fn(|_| None));
            for (position, colliding_slots) in slots_by_position {
                if colliding_slots.len() < 2 {
                    continue;
                }
                let row = usize::from(position.row());
                let column = usize::from(position.column());
                let slots_slice = colliding_slots.as_slice();
                cells[row][column] = Some(CollisionSlots::new(slots_slice));
            }
            result[grid_index] = PositionCollisionCard { role, cells };
        }
        result
    }

    pub fn hotkey_collisions(
        &self,
        custom_keys: &CustomKeys,
        layout: GridLayout,
    ) -> [HotkeyCollisionCard; 2] {
        let empty = HotkeyCollisionCard {
            role: GridRole::MainCommand,
            cells: [[None; 4]; 3],
        };
        let mut result = [empty, empty];
        for (grid_index, named_grid) in self.grids.iter().enumerate().take(2) {
            let role = named_grid.role;
            let is_research = role.is_research_context();
            let mut slots_by_token: HashMap<HotkeyToken, Vec<GridSlotId>> = HashMap::new();
            for slot in named_grid.card.filled_slots() {
                if let Some(token) = custom_keys.effective_hotkey_token(&slot, layout, is_research)
                {
                    slots_by_token.entry(token).or_default().push(slot);
                }
            }
            let mut cells: [[Option<HotkeyCollisionAtCell>; 4]; 3] =
                std::array::from_fn(|_| std::array::from_fn(|_| None));
            for (token, mut colliding_slots) in slots_by_token {
                let mut seen: HashSet<&str> = HashSet::new();
                colliding_slots.retain(|slot| seen.insert(slot.as_str()));
                if colliding_slots.len() < 2 {
                    continue;
                }
                let HotkeyToken::Letter(letter) = token else {
                    continue;
                };
                let Some(position) = layout.position_for_letter(letter.character()) else {
                    continue;
                };
                let row = usize::from(position.row());
                let column = usize::from(position.column());
                let slots_slice = colliding_slots.as_slice();
                let collision_slots = CollisionSlots::new(slots_slice);
                cells[row][column] = Some(HotkeyCollisionAtCell {
                    token,
                    collision_slots,
                });
            }
            result[grid_index] = HotkeyCollisionCard { role, cells };
        }
        result
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionCollisionCardIterator {
    card: PositionCollisionCard,
    index: u8,
}

impl Iterator for PositionCollisionCardIterator {
    type Item = (GridCoordinate, CollisionSlots);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            if index >= 12 {
                return None;
            }
            self.index += 1;
            let row_u8 = index / 4;
            let column_u8 = index % 4;
            let row_usize = usize::from(row_u8);
            let column_usize = usize::from(column_u8);
            let cell = self.card.cells[row_usize][column_usize];
            let Some(collision_slots) = cell else {
                continue;
            };
            let row = crate::model::RowIndex::try_from(row_u8).ok()?;
            let column = crate::model::ColumnIndex::try_from(column_u8).ok()?;
            let position = GridCoordinate::new(column, row);
            return Some((position, collision_slots));
        }
    }
}

impl IntoIterator for PositionCollisionCard {
    type Item = (GridCoordinate, CollisionSlots);
    type IntoIter = PositionCollisionCardIterator;

    fn into_iter(self) -> Self::IntoIter {
        PositionCollisionCardIterator {
            card: self,
            index: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HotkeyCollisionCardIterator {
    card: HotkeyCollisionCard,
    index: u8,
}

impl Iterator for HotkeyCollisionCardIterator {
    type Item = (GridCoordinate, HotkeyCollisionAtCell);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            if index >= 12 {
                return None;
            }
            self.index += 1;
            let row_u8 = index / 4;
            let column_u8 = index % 4;
            let row_usize = usize::from(row_u8);
            let column_usize = usize::from(column_u8);
            let cell = self.card.cells[row_usize][column_usize];
            let Some(entry) = cell else {
                continue;
            };
            let row = crate::model::RowIndex::try_from(row_u8).ok()?;
            let column = crate::model::ColumnIndex::try_from(column_u8).ok()?;
            let position = GridCoordinate::new(column, row);
            return Some((position, entry));
        }
    }
}

impl IntoIterator for HotkeyCollisionCard {
    type Item = (GridCoordinate, HotkeyCollisionAtCell);
    type IntoIter = HotkeyCollisionCardIterator;

    fn into_iter(self) -> Self::IntoIter {
        HotkeyCollisionCardIterator {
            card: self,
            index: 0,
        }
    }
}

impl PartialEq for CollisionSlots {
    fn eq(&self, other: &Self) -> bool {
        if self.count != other.count {
            return false;
        }
        let count = usize::from(self.count);
        (0..count).all(|index| {
            let left = self.slots[index].map(|slot| slot.as_str());
            let right = other.slots[index].map(|slot| slot.as_str());
            left == right
        })
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PositionCollisionCardBuilder {
    role: GridRole,
    cells: [[Option<CollisionSlots>; 4]; 3],
}

#[cfg(test)]
impl PositionCollisionCardBuilder {
    pub(crate) fn new(role: GridRole) -> Self {
        Self {
            role,
            cells: [[None; 4]; 3],
        }
    }

    pub(crate) fn collision_at(mut self, column: u8, row: u8, slots: &[GridSlotId]) -> Self {
        let column_index = usize::from(column);
        let row_index = usize::from(row);
        self.cells[row_index][column_index] = Some(CollisionSlots::new(slots));
        self
    }

    pub(crate) fn build(self) -> PositionCollisionCard {
        PositionCollisionCard {
            role: self.role,
            cells: self.cells,
        }
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct HotkeyCollisionCardBuilder {
    role: GridRole,
    cells: [[Option<HotkeyCollisionAtCell>; 4]; 3],
    layout: GridLayout,
}

#[cfg(test)]
impl HotkeyCollisionCardBuilder {
    pub(crate) fn new(role: GridRole, layout: GridLayout) -> Self {
        Self {
            role,
            cells: [[None; 4]; 3],
            layout,
        }
    }

    pub(crate) fn collision(mut self, letter: char, slots: &[GridSlotId]) -> Self {
        let upper = letter.to_ascii_uppercase();
        let Ok(token) = HotkeyToken::try_from(upper) else {
            return self;
        };
        let Some(position) = self.layout.position_for_letter(upper) else {
            return self;
        };
        let row = usize::from(position.row());
        let column = usize::from(position.column());
        let collision_slots = CollisionSlots::new(slots);
        self.cells[row][column] = Some(HotkeyCollisionAtCell {
            token,
            collision_slots,
        });
        self
    }

    pub(crate) fn build(self) -> HotkeyCollisionCard {
        HotkeyCollisionCard {
            role: self.role,
            cells: self.cells,
        }
    }
}

impl ddd::ReadModel for UnitGrids {}

#[cfg(test)]
mod tests;
