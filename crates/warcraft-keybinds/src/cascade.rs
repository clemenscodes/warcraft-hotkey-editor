use std::collections::HashMap;

use warcraft_api::{ButtonPosition, WarcraftObjectMeta};

use crate::CustomKeysFile;
use crate::catalog::CommandCatalog;
use crate::lookup::ObjectLookup;
use crate::slot::GridSlotId;
use crate::unit_slots::UnitSlots;

const GRID_COLUMNS: u8 = 4;
const GRID_ROWS: u8 = 3;

#[derive(Clone)]
struct ResolvedSlot {
    slot_id: GridSlotId,
    position: Option<ButtonPosition>,
}

pub fn current_for(
    slot: &GridSlotId,
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> Option<ButtonPosition> {
    match slot {
        GridSlotId::Ability(ability_id) => {
            current_for_ability(ability_id, custom_keys, is_research_context)
        }
        GridSlotId::AbilityOff(ability_id) => current_for_ability_off(ability_id, custom_keys),
        GridSlotId::Command(command_name) => current_for_command(command_name, custom_keys),
    }
}

pub fn current_for_ability_off(
    ability_id: &str,
    custom_keys: Option<&CustomKeysFile>,
) -> Option<ButtonPosition> {
    let custom_unbutton = custom_keys
        .and_then(|file| file.binding(ability_id))
        .and_then(|binding| binding.unbutton_position())
        .map(|position| ButtonPosition::new(position.column(), position.row()));
    if custom_unbutton.is_some() {
        return custom_unbutton;
    }
    let warcraft_object = ObjectLookup::by_id(ability_id)?;
    match warcraft_object.meta() {
        WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_button_position(),
        _ => None,
    }
}

pub fn current_for_ability(
    ability_id: &str,
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> Option<ButtonPosition> {
    let custom_button = custom_keys
        .and_then(|file| file.binding(ability_id))
        .and_then(|binding| binding.button_position())
        .map(|position| ButtonPosition::new(position.column(), position.row()));
    let custom_research = custom_keys
        .and_then(|file| file.binding(ability_id))
        .and_then(|binding| binding.research_button_position())
        .map(|position| ButtonPosition::new(position.column(), position.row()));

    if is_research_context {
        if custom_research.is_some() {
            return custom_research;
        }
        return ObjectLookup::by_id(ability_id)
            .and_then(|warcraft_object| warcraft_object.default_research_button_position());
    }

    if custom_button.is_some() {
        return custom_button;
    }
    ObjectLookup::by_id(ability_id)
        .and_then(|warcraft_object| warcraft_object.default_button_position())
}

pub fn current_for_command(
    command_name: &str,
    custom_keys: Option<&CustomKeysFile>,
) -> Option<ButtonPosition> {
    let custom_position = custom_keys
        .and_then(|file| file.command(command_name))
        .and_then(|binding| binding.button_position())
        .map(|position| ButtonPosition::new(position.column(), position.row()));
    if custom_position.is_some() {
        return custom_position;
    }
    default_command_position(command_name)
}

pub fn default_command_position(command_name: &str) -> Option<ButtonPosition> {
    use warcraft_api::WarcraftObjectMeta;
    let warcraft_object = ObjectLookup::by_id(command_name)?;
    let WarcraftObjectMeta::Command(command_meta) = warcraft_object.meta() else {
        return None;
    };
    command_meta.default_button_position()
}

pub fn should_auto_position(slot: &GridSlotId) -> bool {
    let GridSlotId::Ability(ability_id) = slot else {
        return false;
    };
    let Some(warcraft_object) = ObjectLookup::by_id(ability_id) else {
        return false;
    };
    !matches!(warcraft_object.meta(), WarcraftObjectMeta::Ability(_))
}

pub fn resolved_for(
    slot: &GridSlotId,
    candidate_slots: &[GridSlotId],
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> Option<ButtonPosition> {
    let resolved_entries =
        resolve_container(candidate_slots, custom_keys, is_research_context);
    let matching_entry = resolved_entries
        .iter()
        .find(|(slot_id, _)| slots_match(slot_id, slot))?;
    matching_entry.1
}

pub fn resolve_container(
    candidate_slots: &[GridSlotId],
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> Vec<(GridSlotId, Option<ButtonPosition>)> {
    resolve_container_impl(candidate_slots, custom_keys, is_research_context, &HashMap::new())
}

fn resolve_container_impl(
    candidate_slots: &[GridSlotId],
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
    globally_blocked: &HashMap<String, Vec<ButtonPosition>>,
) -> Vec<(GridSlotId, Option<ButtonPosition>)> {
    let mut entries: Vec<ResolvedSlot> = candidate_slots
        .iter()
        .map(|slot| ResolvedSlot {
            slot_id: slot.clone(),
            position: None,
        })
        .collect();
    let mut occupied_positions: Vec<ButtonPosition> = Vec::new();

    for entry in entries.iter_mut() {
        if !matches!(entry.slot_id, GridSlotId::Command(_)) {
            continue;
        }
        if CommandCatalog::is_context_command(&entry.slot_id) {
            continue;
        }
        let assigned_position = resolve_with_cascade(
            &entry.slot_id,
            &occupied_positions,
            custom_keys,
            is_research_context,
        );
        if let Some(position_value) = assigned_position {
            occupied_positions.push(position_value);
        }
        entry.position = assigned_position;
    }

    // Pre-compute the positions "reserved" by not-yet-processed abilities
    // with explicit positions. When cascading a collided ability, we avoid
    // landing on another ability's reserved slot — otherwise a cascade chain
    // (e.g. A and B both at 0,2 → B cascades to 1,2, stealing C's slot and
    // pushing C to 2,2) writes a globally wrong position for C.
    let mut reserved_positions: Vec<ButtonPosition> = entries
        .iter()
        .filter(|e| {
            matches!(e.slot_id, GridSlotId::Ability(_) | GridSlotId::AbilityOff(_))
        })
        .filter(|e| has_custom_position(&e.slot_id, custom_keys, is_research_context))
        .filter_map(|e| current_for(&e.slot_id, custom_keys, is_research_context))
        .collect();

    for entry in entries.iter_mut() {
        if !matches!(
            entry.slot_id,
            GridSlotId::Ability(_) | GridSlotId::AbilityOff(_)
        ) {
            continue;
        }
        if !has_custom_position(&entry.slot_id, custom_keys, is_research_context) {
            continue;
        }
        let explicit_position = current_for(&entry.slot_id, custom_keys, is_research_context);
        // Release this ability's reservation before computing the cascade so
        // the ability itself is not blocked from its own preferred slot.
        if let Some(pos) = explicit_position {
            if let Some(idx) = reserved_positions
                .iter()
                .position(|p| p.column() == pos.column() && p.row() == pos.row())
            {
                reserved_positions.remove(idx);
            }
        }
        let assigned = match explicit_position {
            Some(pos) if position_occupied(&occupied_positions, pos) => {
                // Extend the avoid list with positions globally claimed by
                // co-unit abilities across all other units that share this ability.
                // This prevents a cascade in one unit from stealing the natural
                // home of an ability in another unit.
                let global: &[ButtonPosition] = match &entry.slot_id {
                    GridSlotId::Ability(id) => globally_blocked
                        .get(&id.to_ascii_lowercase())
                        .map_or(&[], |v| v.as_slice()),
                    _ => &[],
                };
                let mut combined = reserved_positions.clone();
                combined.extend_from_slice(global);
                next_free_cell_not_reserved(pos.row(), &occupied_positions, &combined)
            }
            other => other,
        };
        if let Some(position_value) = assigned {
            occupied_positions.push(position_value);
        }
        entry.position = assigned;
    }

    for entry in entries.iter_mut() {
        if !matches!(
            entry.slot_id,
            GridSlotId::Ability(_) | GridSlotId::AbilityOff(_)
        ) {
            continue;
        }
        if entry.position.is_some() {
            continue;
        }
        let assigned_position = resolve_with_cascade(
            &entry.slot_id,
            &occupied_positions,
            custom_keys,
            is_research_context,
        );
        if let Some(position_value) = assigned_position {
            occupied_positions.push(position_value);
        }
        entry.position = assigned_position;
    }

    for entry in entries.iter_mut() {
        if !CommandCatalog::is_context_command(&entry.slot_id) {
            continue;
        }
        let explicit_position = current_for(&entry.slot_id, custom_keys, is_research_context);
        let Some(position_value) = explicit_position else {
            continue;
        };
        if position_occupied(&occupied_positions, position_value) {
            continue;
        }
        entry.position = Some(position_value);
        occupied_positions.push(position_value);
    }

    entries
        .into_iter()
        .map(|entry| (entry.slot_id, entry.position))
        .collect()
}

pub fn has_custom_position(
    slot: &GridSlotId,
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> bool {
    match slot {
        GridSlotId::Ability(ability_id) => {
            if is_research_context {
                custom_keys
                    .and_then(|f| f.binding(ability_id))
                    .and_then(|b| b.research_button_position())
                    .is_some()
            } else {
                custom_keys
                    .and_then(|f| f.binding(ability_id))
                    .and_then(|b| b.button_position())
                    .is_some()
            }
        }
        GridSlotId::AbilityOff(ability_id) => custom_keys
            .and_then(|f| f.binding(ability_id))
            .and_then(|b| b.unbutton_position())
            .is_some(),
        GridSlotId::Command(command_name) => custom_keys
            .and_then(|f| f.command(command_name))
            .and_then(|b| b.button_position())
            .is_some(),
    }
}

pub fn resolve_with_cascade(
    slot: &GridSlotId,
    occupied_positions: &[ButtonPosition],
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
) -> Option<ButtonPosition> {
    let explicit_position = current_for(slot, custom_keys, is_research_context);
    match explicit_position {
        Some(position_value) => {
            if position_occupied(occupied_positions, position_value) {
                next_free_cell(position_value.row(), occupied_positions)
            } else {
                Some(position_value)
            }
        }
        None => {
            if should_auto_position(slot) || is_research_context {
                next_free_cell(0, occupied_positions)
            } else if matches!(slot, GridSlotId::Ability(_)) {
                // Some abilities have no Buttonpos= in any abilityfunc.txt. The WC3
                // engine auto-places these on the bottom row (row 2) of the command
                // card, cascading right if occupied (e.g. Aatp/Prioritize on Gargoyle).
                next_free_cell(2, occupied_positions)
            } else {
                None
            }
        }
    }
}

pub fn position_occupied(occupied_positions: &[ButtonPosition], candidate: ButtonPosition) -> bool {
    occupied_positions.iter().any(|existing| {
        existing.column() == candidate.column() && existing.row() == candidate.row()
    })
}

pub fn next_free_cell(
    preferred_row: u8,
    occupied_positions: &[ButtonPosition],
) -> Option<ButtonPosition> {
    for column in 0..GRID_COLUMNS {
        let candidate = ButtonPosition::new(column, preferred_row);
        if !position_occupied(occupied_positions, candidate) {
            return Some(candidate);
        }
    }
    for row in 0..GRID_ROWS {
        if row == preferred_row {
            continue;
        }
        for column in 0..GRID_COLUMNS {
            let candidate = ButtonPosition::new(column, row);
            if !position_occupied(occupied_positions, candidate) {
                return Some(candidate);
            }
        }
    }
    None
}

/// Like `next_free_cell` but also avoids positions reserved by not-yet-placed
/// abilities. Falls back to ignoring reservations when no unreserved cell is
/// available, to guarantee a result whenever any cell is free.
pub fn next_free_cell_not_reserved(
    preferred_row: u8,
    occupied_positions: &[ButtonPosition],
    reserved_positions: &[ButtonPosition],
) -> Option<ButtonPosition> {
    let is_blocked =
        |c: ButtonPosition| position_occupied(occupied_positions, c) || position_occupied(reserved_positions, c);
    for column in 0..GRID_COLUMNS {
        let candidate = ButtonPosition::new(column, preferred_row);
        if !is_blocked(candidate) {
            return Some(candidate);
        }
    }
    for row in 0..GRID_ROWS {
        if row == preferred_row {
            continue;
        }
        for column in 0..GRID_COLUMNS {
            let candidate = ButtonPosition::new(column, row);
            if !is_blocked(candidate) {
                return Some(candidate);
            }
        }
    }
    // All non-reserved cells are occupied — fall back to ignoring reservations.
    next_free_cell(preferred_row, occupied_positions)
}

/// Resolve and write back cascade positions for every ability slot in
/// `slot_ids`. After this call the positions stored in `file` are
/// self-consistent — no further fixup at render time.
pub fn fully_normalize(file: &mut CustomKeysFile) {
    // Snapshot initial positions of every ability across every unit BEFORE any
    // write-back.  When cascading an ability that collides in one unit, we use
    // this map to avoid landing on a position that another ability claims as its
    // natural home in a different unit.  Without this, a cascade in unit A can
    // corrupt a perfectly valid layout in unit B that shares the same ability.
    let globally_blocked = precompute_globally_blocked(file);

    for unit_id in UnitSlots::all_unit_ids() {
        let cmd_card = UnitSlots::command_card_for(unit_id);
        if !cmd_card.is_empty() {
            write_container_resolved_global(file, &cmd_card, false, &globally_blocked);
        }
        if let Some(build_menu) = UnitSlots::build_menu_for(unit_id) {
            write_container_resolved_global(file, &build_menu, false, &globally_blocked);
        }
        if let Some(uprooted_menu) = UnitSlots::uprooted_menu_for(unit_id) {
            write_container_resolved_global(file, &uprooted_menu, false, &globally_blocked);
        }
        if let Some(research_menu) = UnitSlots::research_menu_for(unit_id) {
            write_container_resolved_global(file, &research_menu, true, &globally_blocked);
        }
    }
}

/// For each ability, records the initial positions of every other ability that
/// shares a command-card container with it, across all units.  These positions
/// are "globally blocked" — a cascade must not land on them lest it corrupt
/// another unit's layout.
fn precompute_globally_blocked(file: &CustomKeysFile) -> HashMap<String, Vec<ButtonPosition>> {
    let mut result: HashMap<String, Vec<ButtonPosition>> = HashMap::new();

    for unit_id in UnitSlots::all_unit_ids() {
        let cmd_card = UnitSlots::command_card_for(unit_id);
        if !cmd_card.is_empty() {
            add_co_unit_positions(&cmd_card, file, false, &mut result);
        }
        if let Some(build_menu) = UnitSlots::build_menu_for(unit_id) {
            add_co_unit_positions(&build_menu, file, false, &mut result);
        }
        if let Some(uprooted_menu) = UnitSlots::uprooted_menu_for(unit_id) {
            add_co_unit_positions(&uprooted_menu, file, false, &mut result);
        }
        if let Some(research_menu) = UnitSlots::research_menu_for(unit_id) {
            add_co_unit_positions(&research_menu, file, true, &mut result);
        }
    }

    result
}

fn add_co_unit_positions(
    slots: &[GridSlotId],
    file: &CustomKeysFile,
    is_research: bool,
    result: &mut HashMap<String, Vec<ButtonPosition>>,
) {
    let pairs: Vec<(String, ButtonPosition)> = slots
        .iter()
        .filter_map(|s| {
            let GridSlotId::Ability(id) = s else {
                return None;
            };
            let pos = current_for(s, Some(file), is_research)?;
            Some((id.to_ascii_lowercase(), pos))
        })
        .collect();

    for (id_a, _pos_a) in &pairs {
        let blocked = result.entry(id_a.clone()).or_default();
        for (id_b, pos_b) in &pairs {
            if id_a == id_b {
                continue;
            }
            if !blocked
                .iter()
                .any(|p: &ButtonPosition| p.column() == pos_b.column() && p.row() == pos_b.row())
            {
                blocked.push(*pos_b);
            }
        }
    }
}

pub fn write_container_resolved(
    file: &mut CustomKeysFile,
    slot_ids: &[GridSlotId],
    is_research: bool,
) {
    write_container_resolved_global(file, slot_ids, is_research, &HashMap::new());
}

fn write_container_resolved_global(
    file: &mut CustomKeysFile,
    slot_ids: &[GridSlotId],
    is_research: bool,
    globally_blocked: &HashMap<String, Vec<ButtonPosition>>,
) {
    let resolved = resolve_container_impl(slot_ids, Some(file), is_research, globally_blocked);
    for (slot_id, vis_pos_opt) in &resolved {
        let Some(vis_pos) = vis_pos_opt else {
            continue;
        };
        let new_pos = crate::ButtonPosition::new(vis_pos.column(), vis_pos.row());
        match slot_id {
            GridSlotId::Ability(id) => {
                if is_research {
                    let stored = file
                        .binding(id)
                        .and_then(|b| b.research_button_position())
                        .map(|p| ButtonPosition::new(p.column(), p.row()));
                    if stored != Some(*vis_pos)
                        && let Some(binding) = file.binding_or_default_mut(id)
                    {
                        binding.set_research_button_position(Some(new_pos));
                    }
                } else {
                    let stored = file
                        .binding(id)
                        .and_then(|b| b.button_position())
                        .map(|p| ButtonPosition::new(p.column(), p.row()));
                    if stored != Some(*vis_pos)
                        && let Some(binding) = file.binding_or_default_mut(id)
                    {
                        binding.set_button_position(Some(new_pos));
                    }
                }
            }
            GridSlotId::AbilityOff(id) => {
                let stored = file
                    .binding(id)
                    .and_then(|b| b.unbutton_position())
                    .map(|p| ButtonPosition::new(p.column(), p.row()));
                if stored != Some(*vis_pos)
                    && let Some(binding) = file.binding_or_default_mut(id)
                {
                    binding.set_unbutton_position(Some(new_pos));
                }
            }
            GridSlotId::Command(name) => {
                let stored = file
                    .command(name)
                    .and_then(|b| b.button_position())
                    .map(|p| ButtonPosition::new(p.column(), p.row()));
                if stored != Some(*vis_pos)
                    && let Some(binding) = file.command_or_default_mut(name)
                {
                    binding.set_button_position(Some(new_pos));
                }
            }
        }
    }
    if !is_research {
        normalize_unbutton_positions(file, slot_ids);
    }
}

/// For each on-state ability in this container whose `UnButtonpos` collides
/// with another ability's `Buttonpos`, move the `UnButtonpos` to the self-cell
/// (the ability's own `Buttonpos`).  An empty cell or the self-cell are the
/// only valid landing spots for an unbutton.
fn normalize_unbutton_positions(file: &mut CustomKeysFile, slot_ids: &[GridSlotId]) {
    let button_positions: Vec<(String, crate::ButtonPosition)> = slot_ids
        .iter()
        .filter_map(|slot| {
            let GridSlotId::Ability(id) = slot else {
                return None;
            };
            let pos = file.binding(id.as_str())?.button_position().copied()?;
            Some((id.clone(), pos))
        })
        .collect();

    for (ability_id, button_pos) in &button_positions {
        let Some(unbutton_pos) = file
            .binding(ability_id.as_str())
            .and_then(|b| b.unbutton_position().copied())
        else {
            continue;
        };

        // Already at self-cell — valid, nothing to do.
        if unbutton_pos.column() == button_pos.column() && unbutton_pos.row() == button_pos.row() {
            continue;
        }

        // Collides with another ability's button position → move to self-cell.
        let collides = button_positions.iter().any(|(other_id, other_pos)| {
            !other_id.eq_ignore_ascii_case(ability_id)
                && other_pos.column() == unbutton_pos.column()
                && other_pos.row() == unbutton_pos.row()
        });

        if collides {
            let new_pos = crate::ButtonPosition::new(button_pos.column(), button_pos.row());
            if let Some(binding) = file.binding_or_default_mut(ability_id.as_str()) {
                binding.set_unbutton_position(Some(new_pos));
            }
        }
    }
}

pub fn slots_match(slot_a: &GridSlotId, slot_b: &GridSlotId) -> bool {
    match (slot_a, slot_b) {
        (GridSlotId::Ability(left), GridSlotId::Ability(right))
        | (GridSlotId::AbilityOff(left), GridSlotId::AbilityOff(right))
        | (GridSlotId::Command(left), GridSlotId::Command(right)) => {
            left.eq_ignore_ascii_case(right)
        }
        _ => false,
    }
}
