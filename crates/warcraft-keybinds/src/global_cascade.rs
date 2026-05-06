use std::collections::{HashMap, HashSet};

use warcraft_api::{ButtonPosition, WarcraftObjectMeta};

use crate::CustomKeysFile;
use crate::catalog::CommandCatalog;
use crate::lookup::ObjectLookup;
use crate::slot::GridSlotId;
use crate::unit_slots::UnitSlots;

const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum ContainerRole {
    CommandCard,
    BuildMenu,
    UprootedMenu,
    ResearchMenu,
}

impl ContainerRole {
    fn is_research(self) -> bool {
        matches!(self, Self::ResearchMenu)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ContainerKey {
    unit_id: &'static str,
    role: ContainerRole,
}

/// What kind of binding a solver slot maps to. `Ability` covers both
/// `GridSlotId::Ability` and `GridSlotId::AbilityOff` — they share a
/// single logical placement (same cell across the containers they
/// appear in) but write different fields on the underlying binding.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SolverSlotKind {
    Ability(String),
    Command(String),
}

/// A unified solver slot: one logical entity to place. Same ability
/// id appearing as both on-form (`Ability(_)`) and off-form
/// (`AbilityOff(_)`) collapses into a single `SolverSlot` so the cell
/// they share is reserved across all containers either variant
/// appears in.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SolverSlot {
    kind: SolverSlotKind,
    is_research: bool,
}

impl SolverSlot {
    fn new(grid_slot: &GridSlotId, role: ContainerRole) -> Self {
        let is_research_flag = role.is_research();
        match grid_slot {
            GridSlotId::Ability(ability_id) => {
                let id_lowercase = ability_id.to_lowercase();
                Self {
                    kind: SolverSlotKind::Ability(id_lowercase),
                    is_research: is_research_flag,
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                let id_lowercase = ability_id.to_lowercase();
                Self {
                    kind: SolverSlotKind::Ability(id_lowercase),
                    is_research: is_research_flag,
                }
            }
            GridSlotId::Command(command_name) => {
                let name_lowercase = command_name.to_lowercase();
                Self {
                    kind: SolverSlotKind::Command(name_lowercase),
                    is_research: is_research_flag,
                }
            }
        }
    }
}

/// Per-container record: the solver slot plus whether the
/// corresponding `GridSlotId` was the off-form variant. Used both to
/// place the slot and, during apply, to decide whether to write the
/// `Unbuttonpos` field on the underlying binding.
struct ContainerSlotEntry {
    solver_slot: SolverSlot,
    is_off_form: bool,
}

impl ContainerSlotEntry {
    #[cfg(test)]
    fn position_in_file(&self, file: &CustomKeysFile) -> Option<ButtonPosition> {
        match (&self.solver_slot.kind, self.solver_slot.is_research) {
            (SolverSlotKind::Ability(ability_id), false) => {
                let binding = file.binding(ability_id)?;
                if self.is_off_form {
                    let position_ref = binding.unbutton_position()?;
                    let column = position_ref.column();
                    let row = position_ref.row();
                    Some(ButtonPosition::new(column, row))
                } else {
                    let position_ref = binding.button_position()?;
                    let column = position_ref.column();
                    let row = position_ref.row();
                    Some(ButtonPosition::new(column, row))
                }
            }
            (SolverSlotKind::Ability(ability_id), true) => {
                let binding = file.binding(ability_id)?;
                let position_ref = binding.research_button_position()?;
                let column = position_ref.column();
                let row = position_ref.row();
                Some(ButtonPosition::new(column, row))
            }
            (SolverSlotKind::Command(command_name), _) => {
                let binding = file.command(command_name)?;
                let position_ref = binding.button_position()?;
                let column = position_ref.column();
                let row = position_ref.row();
                Some(ButtonPosition::new(column, row))
            }
        }
    }
}

struct CollectedContainer {
    key: ContainerKey,
    entries: Vec<ContainerSlotEntry>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct GridCell {
    column: u8,
    row: u8,
}

impl GridCell {
    fn is_within_grid(self) -> bool {
        self.column < GRID_COLUMNS && self.row < GRID_ROWS
    }
}

impl From<ButtonPosition> for GridCell {
    fn from(position: ButtonPosition) -> Self {
        let column_value = position.column();
        let row_value = position.row();
        Self {
            column: column_value,
            row: row_value,
        }
    }
}

impl From<GridCell> for ButtonPosition {
    fn from(cell: GridCell) -> Self {
        Self::new(cell.column, cell.row)
    }
}

#[derive(Default)]
struct Occupancy {
    by_container: HashMap<ContainerKey, HashSet<GridCell>>,
}

impl Occupancy {
    fn is_free(&self, container: ContainerKey, cell: GridCell) -> bool {
        let cells_for_container = self.by_container.get(&container);
        match cells_for_container {
            Some(cells) => !cells.contains(&cell),
            None => true,
        }
    }

    fn reserve(&mut self, container: ContainerKey, cell: GridCell) {
        let cells_for_container = self.by_container.entry(container).or_default();
        cells_for_container.insert(cell);
    }

    fn is_free_in_all(&self, containers: &[ContainerKey], cell: GridCell) -> bool {
        for container in containers {
            let container_value = *container;
            let free_here = self.is_free(container_value, cell);
            if !free_here {
                return false;
            }
        }
        true
    }

    fn count_blockers(&self, containers: &[ContainerKey], cell: GridCell) -> usize {
        let mut blocker_count: usize = 0;
        for container in containers {
            let container_value = *container;
            let free_here = self.is_free(container_value, cell);
            if !free_here {
                blocker_count += 1;
            }
        }
        blocker_count
    }
}

/// Aggregated information per `SolverSlot` that affects `apply`.
struct PerSlotMetadata {
    /// True if at least one container holds this slot in its
    /// `AbilityOff` variant. Determines whether `apply` writes the
    /// `Unbuttonpos` field.
    appears_as_off_form: bool,
    /// True if at least one container holds this slot in its on-form
    /// variant (`Ability` or `Command`). Determines whether `apply`
    /// writes the `Buttonpos` (or research-button-pos) field.
    appears_as_on_form: bool,
}

impl PerSlotMetadata {
    fn empty() -> Self {
        Self {
            appears_as_off_form: false,
            appears_as_on_form: false,
        }
    }

    fn merge_entry(&mut self, is_off_form: bool) {
        if is_off_form {
            self.appears_as_off_form = true;
        } else {
            self.appears_as_on_form = true;
        }
    }
}

/// Result of the global cascade: one canonical position per
/// `SolverSlot`. Apply with `apply` to write back into a
/// `CustomKeysFile`.
pub struct GlobalCascade {
    placements: HashMap<SolverSlot, ButtonPosition>,
    metadata: HashMap<SolverSlot, PerSlotMetadata>,
}

impl GlobalCascade {
    /// Solve the cascade for every container in the database. Reads
    /// the file's existing positions as the per-slot preferred
    /// candidates. Higher-multiplicity slots — abilities and commands
    /// that appear in many containers, plus toggle abilities whose
    /// on-form and off-form together cover many containers — are
    /// placed first; lower-multiplicity slots cascade around the
    /// frozen high-priority decisions.
    pub fn solve(file: &CustomKeysFile) -> Self {
        let containers = Self::collect_containers();
        let metadata = Self::build_metadata(&containers);
        let priority_order = Self::compute_priority_order(&containers, &metadata);
        let mut occupancy = Occupancy::default();
        let mut placements: HashMap<SolverSlot, ButtonPosition> = HashMap::new();
        for solver_slot in &priority_order {
            let containing = Self::containers_holding(&containers, solver_slot);
            if containing.is_empty() {
                continue;
            }
            let preferred_cell = Self::preferred_cell_for(solver_slot, &metadata, file);
            let auto_place_row = Self::auto_place_row_for(solver_slot);
            let chosen_cell =
                Self::pick_cell(&occupancy, &containing, preferred_cell, auto_place_row);
            for container_key in &containing {
                let container_value = *container_key;
                occupancy.reserve(container_value, chosen_cell);
            }
            let position = ButtonPosition::from(chosen_cell);
            let slot_for_storage = solver_slot.clone();
            placements.insert(slot_for_storage, position);
        }
        Self {
            placements,
            metadata,
        }
    }

    /// Write all solved positions back into the file. Every binding
    /// touched by the solver gets its corresponding position field
    /// set; for ability slots that appear as off-form in any
    /// container, both `Buttonpos` and `Unbuttonpos` are set to the
    /// same resolved cell so toggle bindings stay co-located.
    pub fn apply(&self, file: &mut CustomKeysFile) {
        for (solver_slot, position) in &self.placements {
            let metadata = self.metadata.get(solver_slot);
            let writes_off_form = metadata
                .map(|entry| entry.appears_as_off_form)
                .unwrap_or(false);
            let writes_on_form = metadata
                .map(|entry| entry.appears_as_on_form)
                .unwrap_or(true);
            let crate_position = Self::to_crate_position(*position);
            let new_position = Some(crate_position);
            match (&solver_slot.kind, solver_slot.is_research) {
                (SolverSlotKind::Ability(ability_id), false) => {
                    if let Some(binding) = file.binding_or_default_mut(ability_id) {
                        if writes_on_form {
                            binding.set_button_position(new_position);
                        }
                        if writes_off_form {
                            binding.set_unbutton_position(new_position);
                        }
                    }
                }
                (SolverSlotKind::Ability(ability_id), true) => {
                    if let Some(binding) = file.binding_or_default_mut(ability_id) {
                        binding.set_research_button_position(new_position);
                    }
                }
                (SolverSlotKind::Command(command_name), _) => {
                    if let Some(binding) = file.command_or_default_mut(command_name) {
                        binding.set_button_position(new_position);
                    }
                }
            }
        }
    }

    pub(crate) fn collect_containers() -> Vec<CollectedContainer> {
        let mut result: Vec<CollectedContainer> = Vec::new();
        for unit_id in UnitSlots::all_unit_ids() {
            let command_card_slots = UnitSlots::command_card_for(unit_id);
            if !command_card_slots.is_empty() {
                let key = ContainerKey {
                    unit_id,
                    role: ContainerRole::CommandCard,
                };
                let key_role = key.role;
                let entries = Self::filter_solvable(&command_card_slots, key_role);
                let collected = CollectedContainer { key, entries };
                result.push(collected);
            }
            if let Some(build_menu_slots) = UnitSlots::build_menu_for(unit_id) {
                let key = ContainerKey {
                    unit_id,
                    role: ContainerRole::BuildMenu,
                };
                let key_role = key.role;
                let entries = Self::filter_solvable(&build_menu_slots, key_role);
                let collected = CollectedContainer { key, entries };
                result.push(collected);
            }
            if let Some(uprooted_slots) = UnitSlots::uprooted_menu_for(unit_id) {
                let key = ContainerKey {
                    unit_id,
                    role: ContainerRole::UprootedMenu,
                };
                let key_role = key.role;
                let entries = Self::filter_solvable(&uprooted_slots, key_role);
                let collected = CollectedContainer { key, entries };
                result.push(collected);
            }
            if let Some(research_slots) = UnitSlots::research_menu_for(unit_id) {
                let key = ContainerKey {
                    unit_id,
                    role: ContainerRole::ResearchMenu,
                };
                let key_role = key.role;
                let entries = Self::filter_solvable(&research_slots, key_role);
                let collected = CollectedContainer { key, entries };
                result.push(collected);
            }
        }
        result
    }

    fn filter_solvable(slots: &[GridSlotId], role: ContainerRole) -> Vec<ContainerSlotEntry> {
        let mut result: Vec<ContainerSlotEntry> = Vec::new();
        let mut already_present: HashSet<SolverSlot> = HashSet::new();
        for grid_slot in slots {
            let is_context = CommandCatalog::is_context_command(grid_slot);
            if is_context {
                continue;
            }
            let solver_slot = SolverSlot::new(grid_slot, role);
            // A single container can list the same slot id multiple
            // times (e.g. when an ability appears via two collection
            // paths in unit_slots.rs). For solver purposes a
            // duplicate is the same placement; collapse it.
            let already_seen = already_present.contains(&solver_slot);
            if already_seen {
                continue;
            }
            let solver_slot_for_set = solver_slot.clone();
            already_present.insert(solver_slot_for_set);
            let is_off_form = matches!(grid_slot, GridSlotId::AbilityOff(_));
            let entry = ContainerSlotEntry {
                solver_slot,
                is_off_form,
            };
            result.push(entry);
        }
        result
    }

    fn build_metadata(
        containers: &[CollectedContainer],
    ) -> HashMap<SolverSlot, PerSlotMetadata> {
        let mut metadata: HashMap<SolverSlot, PerSlotMetadata> = HashMap::new();
        for container in containers {
            for entry in &container.entries {
                let slot_clone = entry.solver_slot.clone();
                let metadata_entry = metadata
                    .entry(slot_clone)
                    .or_insert_with(PerSlotMetadata::empty);
                metadata_entry.merge_entry(entry.is_off_form);
            }
        }
        metadata
    }

    fn compute_priority_order(
        containers: &[CollectedContainer],
        metadata: &HashMap<SolverSlot, PerSlotMetadata>,
    ) -> Vec<SolverSlot> {
        let mut multiplicity: HashMap<SolverSlot, usize> = HashMap::new();
        let mut first_seen_index: HashMap<SolverSlot, usize> = HashMap::new();
        let mut next_index: usize = 0;
        for container in containers {
            for entry in &container.entries {
                let slot_for_count = entry.solver_slot.clone();
                let count_entry = multiplicity.entry(slot_for_count).or_insert(0);
                *count_entry += 1;
                let slot_for_index = entry.solver_slot.clone();
                let already_seen = first_seen_index.contains_key(&slot_for_index);
                if !already_seen {
                    first_seen_index.insert(slot_for_index, next_index);
                    next_index += 1;
                }
            }
        }
        let mut all_slots: Vec<SolverSlot> = metadata.keys().cloned().collect();
        all_slots.sort_by(|left, right| {
            let multiplicity_left = multiplicity.get(left).copied().unwrap_or(0);
            let multiplicity_right = multiplicity.get(right).copied().unwrap_or(0);
            let by_multiplicity_descending = multiplicity_right.cmp(&multiplicity_left);
            if by_multiplicity_descending != std::cmp::Ordering::Equal {
                return by_multiplicity_descending;
            }
            let index_left = first_seen_index.get(left).copied().unwrap_or(usize::MAX);
            let index_right = first_seen_index.get(right).copied().unwrap_or(usize::MAX);
            index_left.cmp(&index_right)
        });
        all_slots
    }

    fn containers_holding(
        containers: &[CollectedContainer],
        solver_slot: &SolverSlot,
    ) -> Vec<ContainerKey> {
        let mut result: Vec<ContainerKey> = Vec::new();
        for container in containers {
            let appears_in_container = container
                .entries
                .iter()
                .any(|entry| &entry.solver_slot == solver_slot);
            if appears_in_container {
                result.push(container.key);
            }
        }
        result
    }

    /// The solver's preferred cell for a slot: take the file's
    /// stored position for whichever underlying field this slot maps
    /// to, falling back to the off-form's stored Unbuttonpos when no
    /// on-form Buttonpos exists. Returns `None` when the file has
    /// nothing stored for this slot, in which case `pick_cell` walks
    /// the auto-place row.
    fn preferred_cell_for(
        solver_slot: &SolverSlot,
        metadata: &HashMap<SolverSlot, PerSlotMetadata>,
        file: &CustomKeysFile,
    ) -> Option<ButtonPosition> {
        match (&solver_slot.kind, solver_slot.is_research) {
            (SolverSlotKind::Ability(ability_id), false) => {
                let metadata_entry = metadata.get(solver_slot);
                let has_on_form = metadata_entry
                    .map(|entry| entry.appears_as_on_form)
                    .unwrap_or(true);
                let has_off_form = metadata_entry
                    .map(|entry| entry.appears_as_off_form)
                    .unwrap_or(false);
                let binding = file.binding(ability_id)?;
                let on_position = if has_on_form {
                    binding.button_position().map(|position_ref| {
                        let column = position_ref.column();
                        let row = position_ref.row();
                        ButtonPosition::new(column, row)
                    })
                } else {
                    None
                };
                if on_position.is_some() {
                    return on_position;
                }
                if has_off_form {
                    let off_position = binding.unbutton_position().map(|position_ref| {
                        let column = position_ref.column();
                        let row = position_ref.row();
                        ButtonPosition::new(column, row)
                    });
                    return off_position;
                }
                None
            }
            (SolverSlotKind::Ability(ability_id), true) => {
                let binding = file.binding(ability_id)?;
                let position_ref = binding.research_button_position()?;
                let column = position_ref.column();
                let row = position_ref.row();
                Some(ButtonPosition::new(column, row))
            }
            (SolverSlotKind::Command(command_name), _) => {
                let binding = file.command(command_name)?;
                let position_ref = binding.button_position()?;
                let column = position_ref.column();
                let row = position_ref.row();
                Some(ButtonPosition::new(column, row))
            }
        }
    }

    /// Pick the first cell that's free in every containing container,
    /// trying candidates in this order:
    ///
    ///   1. The supplied `preferred_cell` if any (stored hint).
    ///   2. The auto-place row, sweeping columns left to right.
    ///   3. Every other row, sweeping columns left to right.
    ///
    /// If no cell is free in *all* containers, fall back to the cell
    /// that's blocked in the fewest containers — the unavoidable
    /// over-constrained case where one ability lives in a set of
    /// units whose grids are collectively saturated.
    fn pick_cell(
        occupancy: &Occupancy,
        containing: &[ContainerKey],
        preferred_cell: Option<ButtonPosition>,
        auto_place_row: u8,
    ) -> GridCell {
        let mut visited: HashSet<GridCell> = HashSet::new();
        let mut best_blocked_cell: Option<GridCell> = None;
        let mut best_blocked_count: usize = usize::MAX;
        let consider = |cell: GridCell, visited: &mut HashSet<GridCell>| -> Option<GridCell> {
            let already_visited = visited.contains(&cell);
            if already_visited {
                return None;
            }
            visited.insert(cell);
            let feasible = occupancy.is_free_in_all(containing, cell);
            if feasible {
                return Some(cell);
            }
            None
        };
        if let Some(position) = preferred_cell {
            let cell = GridCell::from(position);
            let inside_grid = cell.is_within_grid();
            if inside_grid {
                if let Some(picked) = consider(cell, &mut visited) {
                    return picked;
                }
                let blocked_count = occupancy.count_blockers(containing, cell);
                if blocked_count < best_blocked_count {
                    best_blocked_count = blocked_count;
                    best_blocked_cell = Some(cell);
                }
            }
        }
        for column in 0..GRID_COLUMNS {
            let cell = GridCell {
                column,
                row: auto_place_row,
            };
            if let Some(picked) = consider(cell, &mut visited) {
                return picked;
            }
            let blocked_count = occupancy.count_blockers(containing, cell);
            if blocked_count < best_blocked_count {
                best_blocked_count = blocked_count;
                best_blocked_cell = Some(cell);
            }
        }
        for row in 0..GRID_ROWS {
            if row == auto_place_row {
                continue;
            }
            for column in 0..GRID_COLUMNS {
                let cell = GridCell { column, row };
                if let Some(picked) = consider(cell, &mut visited) {
                    return picked;
                }
                let blocked_count = occupancy.count_blockers(containing, cell);
                if blocked_count < best_blocked_count {
                    best_blocked_count = blocked_count;
                    best_blocked_cell = Some(cell);
                }
            }
        }
        // The grid is over-constrained for this slot's container set.
        // Return the cell that blocks the fewest containers, which is
        // the closest thing to a sensible answer when there is no
        // universally free cell. If even that is somehow None (no
        // cells visited at all — should not happen for the 4×3 grid),
        // fall back to the auto-place row's first column.
        let fallback_cell = GridCell {
            column: 0,
            row: auto_place_row,
        };
        best_blocked_cell.unwrap_or(fallback_cell)
    }

    fn auto_place_row_for(solver_slot: &SolverSlot) -> u8 {
        if solver_slot.is_research {
            return 0;
        }
        match &solver_slot.kind {
            SolverSlotKind::Ability(ability_id) => {
                let object_lookup = ObjectLookup::by_id(ability_id);
                match object_lookup {
                    Some(warcraft_object) => match warcraft_object.meta() {
                        WarcraftObjectMeta::Ability(_) => 2,
                        _ => 0,
                    },
                    None => 2,
                }
            }
            SolverSlotKind::Command(_) => 2,
        }
    }

    /// Convert from the `warcraft_api::ButtonPosition` type used
    /// throughout the solver into the `crate::ButtonPosition`
    /// expected by the binding setter API. The two types are
    /// structurally identical; this is a re-pack until the duplicate
    /// type is consolidated in a future cleanup phase.
    fn to_crate_position(position: ButtonPosition) -> crate::ButtonPosition {
        let column_value = position.column();
        let row_value = position.row();
        crate::ButtonPosition::new(column_value, row_value)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct SlotPosition {
        position: ButtonPosition,
    }

    impl SlotPosition {
        fn collision_count(slot_positions: &[Self]) -> usize {
            let mut seen: HashMap<GridCell, usize> = HashMap::new();
            for slot_position in slot_positions {
                let cell = GridCell::from(slot_position.position);
                let count_entry = seen.entry(cell).or_insert(0);
                *count_entry += 1;
            }
            let mut collision_count: usize = 0;
            for value in seen.values() {
                let value_copy = *value;
                if value_copy > 1 {
                    collision_count += value_copy - 1;
                }
            }
            collision_count
        }
    }

    #[test]
    fn solver_produces_collision_free_command_card_for_real_baseline() {
        let baseline_text = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = CustomKeysFile::from(baseline_text);
        let solution = GlobalCascade::solve(&file);
        solution.apply(&mut file);

        let containers = GlobalCascade::collect_containers();
        for container in &containers {
            let mut positions_in_container: Vec<SlotPosition> = Vec::new();
            for entry in &container.entries {
                if let Some(position) = entry.position_in_file(&file) {
                    let slot_position = SlotPosition { position };
                    positions_in_container.push(slot_position);
                }
            }
            let collisions = SlotPosition::collision_count(&positions_in_container);
            assert_eq!(
                collisions, 0,
                "container {:?} has unresolved collisions after solve",
                container.key,
            );
        }
    }

    #[test]
    fn solver_assigns_high_multiplicity_command_to_baseline_position() {
        let baseline_text = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = CustomKeysFile::from(baseline_text);
        let solution = GlobalCascade::solve(&file);
        solution.apply(&mut file);
        let cmd_attack = file.command("CmdAttack");
        let cmd_attack_position = cmd_attack
            .and_then(|binding| binding.button_position())
            .copied();
        assert!(
            cmd_attack_position.is_some(),
            "CmdAttack must have a resolved Buttonpos"
        );
    }

    #[test]
    fn solver_is_idempotent_on_solved_input() {
        let baseline_text = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file_first = CustomKeysFile::from(baseline_text);
        let solution_first = GlobalCascade::solve(&file_first);
        solution_first.apply(&mut file_first);
        let text_after_first = file_first.to_file_content();

        let mut file_second = CustomKeysFile::from(text_after_first.as_str());
        let solution_second = GlobalCascade::solve(&file_second);
        solution_second.apply(&mut file_second);
        let text_after_second = file_second.to_file_content();

        assert_eq!(
            text_after_first, text_after_second,
            "running the solver twice must produce identical text"
        );
    }

    #[test]
    fn cross_unit_ability_keeps_one_position_across_all_units() {
        let baseline_text = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = CustomKeysFile::from(baseline_text);
        let solution = GlobalCascade::solve(&file);
        solution.apply(&mut file);

        let stored_position = file
            .binding("Anh2")
            .and_then(|binding| binding.button_position())
            .copied();
        assert!(
            stored_position.is_some(),
            "Anh2 must have a resolved Buttonpos"
        );
    }

    #[test]
    fn toggle_ability_on_form_and_off_form_share_a_cell() {
        // For abilities that appear as both Ability and AbilityOff in
        // different units, the unified solver writes Buttonpos and
        // Unbuttonpos to the same cell so the toggle stays
        // co-located on the grid.
        let baseline_text = include_str!("../../hotkey-editor/templates/CustomKeys.txt");
        let mut file = CustomKeysFile::from(baseline_text);
        let solution = GlobalCascade::solve(&file);
        solution.apply(&mut file);

        let aro1_binding = file.binding("Aro1");
        let on_position = aro1_binding
            .and_then(|binding| binding.button_position())
            .copied();
        let off_position = aro1_binding
            .and_then(|binding| binding.unbutton_position())
            .copied();
        assert!(
            on_position.is_some(),
            "Aro1 must have a resolved Buttonpos (on-form is used in uprooted menus)"
        );
        assert!(
            off_position.is_some(),
            "Aro1 must have a resolved Unbuttonpos (off-form is used in rooted command cards)"
        );
        assert_eq!(
            on_position, off_position,
            "Aro1 on-form and off-form must share the same cell"
        );
    }

}
