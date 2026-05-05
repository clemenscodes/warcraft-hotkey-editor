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
        resolve_container(candidate_slots, custom_keys, is_research_context, false);
    let matching_entry = resolved_entries
        .iter()
        .find(|(slot_id, _)| slots_match(slot_id, slot))?;
    matching_entry.1
}

pub fn resolve_container(
    candidate_slots: &[GridSlotId],
    custom_keys: Option<&CustomKeysFile>,
    is_research_context: bool,
    cascade_explicit: bool,
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
        let assigned = if cascade_explicit {
            match explicit_position {
                Some(pos) if position_occupied(&occupied_positions, pos) => {
                    next_free_cell(pos.row(), &occupied_positions)
                }
                other => other,
            }
        } else {
            explicit_position
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

/// Resolve and write back cascade positions for every ability slot in
/// `slot_ids`. After this call the positions stored in `file` are
/// self-consistent — no further fixup at render time.
pub fn fully_normalize(file: &mut CustomKeysFile) {
    for unit_id in UnitSlots::all_unit_ids() {
        let cmd_card = UnitSlots::command_card_for(unit_id);
        if !cmd_card.is_empty() {
            write_container_resolved(file, &cmd_card, false);
        }
        if let Some(build_menu) = UnitSlots::build_menu_for(unit_id) {
            write_container_resolved(file, &build_menu, false);
        }
        if let Some(uprooted_menu) = UnitSlots::uprooted_menu_for(unit_id) {
            write_container_resolved(file, &uprooted_menu, false);
        }
        if let Some(research_menu) = UnitSlots::research_menu_for(unit_id) {
            write_container_resolved(file, &research_menu, true);
        }
    }
}

pub fn write_container_resolved(
    file: &mut CustomKeysFile,
    slot_ids: &[GridSlotId],
    is_research: bool,
) {
    let resolved = resolve_container(slot_ids, Some(file), is_research, true);
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
