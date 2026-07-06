//! The paint-ready command grid. The UI calls [`CustomKeys::rendered_command_grid`]
//! and copies each [`RenderedTile`] straight onto its tile widget; every decision
//! (occupant, hotkey, conflict, passive, command, selected, draggable, tier
//! preview) is made here, in the domain, never in the UI.

use crate::custom_keys::CustomKeys;
use crate::display::ability_cell::{AbilityCell, AbilityIconPath};
use crate::display::grid_behavior::GridBehavior;
use crate::grid::layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::identity::hotkey_token::HotkeyToken;
use crate::identity::slot::GridSlotId;
use crate::model::{ColumnIndex, GridCoordinate, RowIndex};
use std::collections::{HashMap, HashSet};
use warcraft_database::ObjectLookup;

/// One fully resolved tile, ready to paint. Its address is a [`GridCoordinate`],
/// never a loose integer.
#[derive(Clone, PartialEq, Debug)]
pub struct RenderedTile {
    coordinate: GridCoordinate,
    occupant: Option<GridSlotId>,
    display_name: String,
    icon: Option<AbilityIconPath>,
    hotkey: HotkeyToken,
    is_selected: bool,
    is_conflict: bool,
    is_passive: bool,
    is_command: bool,
    draggable: bool,
}

impl RenderedTile {
    pub fn coordinate(&self) -> GridCoordinate {
        self.coordinate
    }

    pub fn occupant(&self) -> Option<GridSlotId> {
        self.occupant
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn icon(&self) -> Option<&AbilityIconPath> {
        self.icon.as_ref()
    }

    pub fn hotkey(&self) -> HotkeyToken {
        self.hotkey
    }

    pub fn is_selected(&self) -> bool {
        self.is_selected
    }

    pub fn is_conflict(&self) -> bool {
        self.is_conflict
    }

    pub fn is_passive(&self) -> bool {
        self.is_passive
    }

    pub fn is_command(&self) -> bool {
        self.is_command
    }

    pub fn draggable(&self) -> bool {
        self.draggable
    }
}

/// Inputs the UI sends for one render pass, beyond the grid's own slots and layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandGridRenderInput<'a> {
    slots: &'a [GridSlotId],
    layout: GridLayout,
    selected: Option<GridSlotId>,
    selected_is_research: bool,
    tier_overrides: &'a HashMap<String, usize>,
    restrict_draggable_to: &'a [GridSlotId],
}

impl<'a> CommandGridRenderInput<'a> {
    pub fn new(
        slots: &'a [GridSlotId],
        layout: GridLayout,
        selected: Option<GridSlotId>,
        selected_is_research: bool,
        tier_overrides: &'a HashMap<String, usize>,
        restrict_draggable_to: &'a [GridSlotId],
    ) -> Self {
        Self {
            slots,
            layout,
            selected,
            selected_is_research,
            tier_overrides,
            restrict_draggable_to,
        }
    }

    pub fn slots(&self) -> &'a [GridSlotId] {
        self.slots
    }

    pub fn layout(&self) -> GridLayout {
        self.layout
    }

    pub fn selected(&self) -> Option<GridSlotId> {
        self.selected
    }

    pub fn selected_is_research(&self) -> bool {
        self.selected_is_research
    }

    pub fn tier_overrides(&self) -> &'a HashMap<String, usize> {
        self.tier_overrides
    }

    pub fn restrict_draggable_to(&self) -> &'a [GridSlotId] {
        self.restrict_draggable_to
    }
}

impl CustomKeys {
    /// Resolves every tile of a command grid into paint-ready form. The UI copies
    /// the result onto its widgets and makes no decisions of its own.
    pub fn rendered_command_grid<B: GridBehavior>(
        &self,
        behavior: &B,
        input: &CommandGridRenderInput,
    ) -> Vec<RenderedTile> {
        let is_research_context = behavior.research_positions();
        let conflicting_tokens =
            self.conflicting_tokens(input.slots(), input.layout(), is_research_context);
        let mut tiles: Vec<RenderedTile> = Vec::new();
        for row in 0..COMMAND_GRID_ROWS {
            for column in 0..COMMAND_GRID_COLUMNS {
                let Ok(column_index) = ColumnIndex::try_from(column) else {
                    continue;
                };
                let Ok(row_index) = RowIndex::try_from(row) else {
                    continue;
                };
                let coordinate = GridCoordinate::new(column_index, row_index);
                let tile = self.resolved_tile(behavior, input, &conflicting_tokens, coordinate);
                tiles.push(tile);
            }
        }
        tiles
    }

    /// The set of hotkey tokens that collide within this grid, computed once per
    /// render. A token collides when two or more distinct objects occupying a
    /// cell in this context resolve to it. This mirrors the per-tile
    /// [`Self::find_hotkey_conflict`] check but evaluates each slot's token a
    /// single time, so the whole grid's conflict marking is linear in the slot
    /// count rather than quadratic.
    fn conflicting_tokens(
        &self,
        slots: &[GridSlotId],
        layout: GridLayout,
        is_research_context: bool,
    ) -> HashSet<HotkeyToken> {
        let mut ids_by_token: HashMap<HotkeyToken, HashSet<String>> = HashMap::new();
        for slot in slots {
            let Some(token) = self.effective_hotkey_token(slot, layout, is_research_context) else {
                continue;
            };
            let slot_id = slot.as_str().to_ascii_lowercase();
            let ids = ids_by_token.entry(token).or_default();
            ids.insert(slot_id);
        }
        let mut conflicting: HashSet<HotkeyToken> = HashSet::new();
        for (token, ids) in ids_by_token {
            if ids.len() >= 2 {
                conflicting.insert(token);
            }
        }
        conflicting
    }

    /// The slot occupying a grid coordinate, resolved in the behavior's position
    /// namespace. The UI passes a [`GridCoordinate`] and the behavior; every
    /// integer conversion and namespace choice happens here, never in the renderer.
    pub fn slot_at<B: GridBehavior>(
        &self,
        behavior: &B,
        slots: &[GridSlotId],
        coordinate: GridCoordinate,
    ) -> Option<GridSlotId> {
        let is_research_context = behavior.research_positions();
        let column = u8::from(coordinate.column());
        let row = u8::from(coordinate.row());
        self.slot_at_position(slots, is_research_context, column, row)
    }

    /// The display name of the ability whose off-state reserves the empty `to`
    /// tile, so a drop there must be refused. `None` when the move is allowed. The
    /// UI uses `Some(_)` both to paint the blocked state and to show the toast.
    pub fn command_grid_move_blocker<B: GridBehavior>(
        &self,
        behavior: &B,
        slots: &[GridSlotId],
        from: GridCoordinate,
        to: GridCoordinate,
    ) -> Option<String> {
        if !behavior.flag_offstate_collisions() {
            return None;
        }
        let is_research_context = behavior.research_positions();
        let target_occupant = self.slot_at_position(
            slots,
            is_research_context,
            u8::from(to.column()),
            u8::from(to.row()),
        );
        if target_occupant.is_some() {
            return None;
        }
        let moving_slot = self.slot_at_position(
            slots,
            is_research_context,
            u8::from(from.column()),
            u8::from(from.row()),
        )?;
        let moving_id = moving_slot.as_str();
        slots.iter().find_map(|slot| {
            let GridSlotId::Ability(ability_id) = slot else {
                return None;
            };
            let ability_code = ability_id.value();
            if ability_code.eq_ignore_ascii_case(moving_id) {
                return None;
            }
            let off_slot = GridSlotId::AbilityOff(*ability_id);
            let off_position = self.position_for_slot(&off_slot, false)?;
            if off_position != to {
                return None;
            }
            let database_object = ObjectLookup::by_id(ability_code);
            let primary_name = database_object.and_then(|object| object.names().first().copied());
            let blocker_name = primary_name.unwrap_or(ability_code).to_owned();
            Some(blocker_name)
        })
    }

    fn resolved_tile<B: GridBehavior>(
        &self,
        behavior: &B,
        input: &CommandGridRenderInput,
        conflicting_tokens: &HashSet<HotkeyToken>,
        coordinate: GridCoordinate,
    ) -> RenderedTile {
        let is_research_context = behavior.research_positions();
        let column = u8::from(coordinate.column());
        let row = u8::from(coordinate.row());
        let occupant_slot = self.slot_at_position(input.slots(), is_research_context, column, row);
        let occupant_cell = occupant_slot.map(|slot| self.cell_for_slot(slot));
        let is_off_state = matches!(occupant_slot, Some(GridSlotId::AbilityOff(_)));
        let is_command = matches!(occupant_slot, Some(GridSlotId::Command(_)));
        let has_occupant = occupant_cell.is_some();
        let is_selected = occupant_slot.is_some_and(|occupant| {
            input.selected().is_some_and(|active| {
                occupant == active && input.selected_is_research() == is_research_context
            })
        });
        let object_id_option = occupant_cell.as_ref().map(|cell| cell.object_id());
        let tier_index = object_id_option
            .and_then(|id| input.tier_overrides().get(id.value()).copied())
            .unwrap_or(0);
        let database_object = object_id_option.and_then(|id| ObjectLookup::by_id(id.value()));
        let tier_name = database_object
            .and_then(|object| object.names().get(tier_index).copied())
            .map(String::from);
        let tier_icon = database_object
            .and_then(|object| object.icons().get(tier_index).copied())
            .map(|raw_icon| AbilityIconPath::Database(raw_icon.trim()));
        let display_name = if is_off_state {
            occupant_cell
                .as_ref()
                .map(|cell| cell.display_name().to_string())
                .unwrap_or_default()
        } else {
            tier_name
                .or_else(|| {
                    occupant_cell
                        .as_ref()
                        .map(|cell| cell.display_name().to_string())
                })
                .unwrap_or_default()
        };
        let cell_icon = occupant_cell
            .as_ref()
            .and_then(|cell| cell.icon_path().cloned());
        let icon = if tier_index > 0 {
            tier_icon.or(cell_icon)
        } else {
            cell_icon.or(tier_icon)
        };
        let effective_token = occupant_slot.and_then(|slot| {
            self.effective_hotkey_token(&slot, input.layout(), is_research_context)
        });
        let layout_character = input
            .layout()
            .letter_at(coordinate.column(), coordinate.row())
            .expect("every grid cell has a layout letter");
        let layout_token =
            HotkeyToken::try_from(layout_character).expect("layout letters are always A to Z");
        let hotkey = effective_token.unwrap_or(layout_token);
        let is_passive = behavior.show_passive_badge()
            && object_id_option
                .map(|id| ObjectLookup::is_passive_ability(id.value()))
                .unwrap_or(false);
        let is_conflict = effective_token
            .map(|token| conflicting_tokens.contains(&token))
            .unwrap_or(false);
        let draggable = has_occupant
            && (input.restrict_draggable_to().is_empty()
                || occupant_slot.is_some_and(|slot| input.restrict_draggable_to().contains(&slot)));
        RenderedTile {
            coordinate,
            occupant: occupant_slot,
            display_name,
            icon,
            hotkey,
            is_selected,
            is_conflict,
            is_passive,
            is_command,
            draggable,
        }
    }

    fn cell_for_slot(&self, slot: GridSlotId) -> AbilityCell {
        match slot {
            GridSlotId::Ability(ability_id) => {
                let binding = self.binding(ability_id);
                AbilityCell::for_ability(ability_id, binding)
            }
            GridSlotId::AbilityOff(ability_id) => {
                let binding = self.binding(ability_id);
                AbilityCell::for_ability_off(ability_id, binding)
            }
            GridSlotId::Command(command_name) => {
                let command_code = command_name.value();
                let binding = self.command(command_code);
                AbilityCell::for_command(command_name, binding)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::display::grid_behavior::CommandBehavior;

    fn render(
        keys: &CustomKeys,
        slots: &[GridSlotId],
        restrict: &[GridSlotId],
    ) -> Vec<RenderedTile> {
        let tier_overrides = HashMap::new();
        let input = CommandGridRenderInput {
            slots,
            layout: GridLayout::qwerty_grid(),
            selected: None,
            selected_is_research: false,
            tier_overrides: &tier_overrides,
            restrict_draggable_to: restrict,
        };
        let behavior = CommandBehavior;
        keys.rendered_command_grid(&behavior, &input)
    }

    fn tile_at(tiles: &[RenderedTile], column: u8, row: u8) -> &RenderedTile {
        tiles
            .iter()
            .find(|tile| {
                u8::from(tile.coordinate().column()) == column
                    && u8::from(tile.coordinate().row()) == row
            })
            .expect("tile exists")
    }

    #[test]
    fn renders_occupant_hotkey_and_marks_it_draggable() {
        let keys = CustomKeys::parse_raw("[ACad]\nHotkey=P\nButtonpos=0,0\n");
        let slots = [GridSlotId::ability("ACad")];
        let tiles = render(&keys, &slots, &[]);
        let occupied = tile_at(&tiles, 0, 0);
        assert_eq!(occupied.occupant(), Some(GridSlotId::ability("ACad")));
        let expected_hotkey = HotkeyToken::try_from('P').expect("letter");
        assert_eq!(occupied.hotkey(), expected_hotkey);
        assert!(occupied.draggable());
        assert!(!occupied.is_command());
    }

    #[test]
    fn empty_tile_shows_its_layout_letter_and_is_not_draggable() {
        let keys = CustomKeys::parse_raw("[ACad]\nButtonpos=0,0\n");
        let slots = [GridSlotId::ability("ACad")];
        let tiles = render(&keys, &slots, &[]);
        let empty = tile_at(&tiles, 1, 0);
        assert_eq!(empty.occupant(), None);
        let expected_hotkey = HotkeyToken::try_from('W').expect("letter");
        assert_eq!(empty.hotkey(), expected_hotkey);
        assert!(!empty.draggable());
    }

    #[test]
    fn cascade_pinned_slot_is_still_draggable_in_the_editor() {
        let keys = CustomKeys::parse_raw("[Aro1]\nButtonpos=0,0\n");
        let slots = [GridSlotId::ability("Aro1")];
        let tiles = render(&keys, &slots, &[]);
        let occupied = tile_at(&tiles, 0, 0);
        assert!(GridSlotId::ability("Aro1").is_pinned());
        assert!(occupied.draggable());
    }

    #[test]
    fn restrict_list_narrows_the_draggable_set() {
        let keys = CustomKeys::parse_raw("[ACad]\nButtonpos=0,0\n[AHbz]\nButtonpos=1,0\n");
        let slots = [GridSlotId::ability("ACad"), GridSlotId::ability("AHbz")];
        let restrict = [GridSlotId::ability("ACad")];
        let tiles = render(&keys, &slots, &restrict);
        assert!(tile_at(&tiles, 0, 0).draggable());
        assert!(!tile_at(&tiles, 1, 0).draggable());
    }

    #[test]
    fn duplicate_hotkey_is_flagged_as_conflict() {
        let keys = CustomKeys::parse_raw(
            "[ACad]\nHotkey=F\nButtonpos=0,0\n[AHbz]\nHotkey=F\nButtonpos=1,0\n",
        );
        let slots = [GridSlotId::ability("ACad"), GridSlotId::ability("AHbz")];
        let tiles = render(&keys, &slots, &[]);
        assert!(tile_at(&tiles, 0, 0).is_conflict());
        assert!(tile_at(&tiles, 1, 0).is_conflict());
    }

    #[test]
    fn slot_at_resolves_the_occupant_in_the_behavior_namespace() {
        let keys = CustomKeys::parse_raw("[ACad]\nButtonpos=1,0\n");
        let slots = [GridSlotId::ability("ACad")];
        let behavior = CommandBehavior;
        let occupied = GridCoordinate::new(ColumnIndex::One, RowIndex::Zero);
        let found = keys.slot_at(&behavior, &slots, occupied);
        assert_eq!(found, Some(GridSlotId::ability("ACad")));
        let empty = GridCoordinate::new(ColumnIndex::Two, RowIndex::Zero);
        assert_eq!(keys.slot_at(&behavior, &slots, empty), None);
    }

    #[test]
    fn move_blocker_refuses_a_tile_reserved_by_an_off_state() {
        let keys = CustomKeys::parse_raw(
            "[ACad]\nButtonpos=0,0\n[AHbz]\nButtonpos=0,1\nUnbuttonpos=1,1\n",
        );
        let slots = [GridSlotId::ability("ACad"), GridSlotId::ability("AHbz")];
        let behavior = CommandBehavior;
        let from = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
        let to = GridCoordinate::new(ColumnIndex::One, RowIndex::One);
        let blocker = keys.command_grid_move_blocker(&behavior, &slots, from, to);
        assert!(blocker.is_some());
    }
}
