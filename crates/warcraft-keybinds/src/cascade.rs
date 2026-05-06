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
    let resolved_entries = resolve_container(candidate_slots, custom_keys, is_research_context);
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
            matches!(
                e.slot_id,
                GridSlotId::Ability(_) | GridSlotId::AbilityOff(_)
            )
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
        if let Some(pos) = explicit_position
            && let Some(idx) = reserved_positions
                .iter()
                .position(|p| p.column() == pos.column() && p.row() == pos.row())
        {
            reserved_positions.remove(idx);
        }
        let assigned = match explicit_position {
            Some(pos) if position_occupied(&occupied_positions, pos) => {
                next_free_cell_not_reserved(pos.row(), &occupied_positions, &reserved_positions)
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
    let is_blocked = |c: ButtonPosition| {
        position_occupied(occupied_positions, c) || position_occupied(reserved_positions, c)
    };
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
    // Ability button positions are NOT written back here.  The cascade
    // algorithm assigns positions per-unit to resolve visual collisions, but
    // the same ability can appear in multiple units with different ideal
    // positions.  Writing a cascade position globally would corrupt units
    // where the ability's stored position was perfectly valid.  The display
    // path (resolve_container / resolved_for) computes cascade on the fly
    // per unit, so no write-back is needed for correctness.
    //
    // We still write back AbilityOff (UnButtonpos) and Command positions.
    // UnButtonpos normalization uses the cascade-resolved Buttonpos values
    // (not the stored ones) so it correctly places off-state buttons even
    // when two abilities share the same stored Buttonpos in this container.
    let resolved = resolve_container(slot_ids, Some(file), is_research);
    for (slot_id, vis_pos_opt) in &resolved {
        let Some(vis_pos) = vis_pos_opt else {
            continue;
        };
        let new_pos = crate::ButtonPosition::new(vis_pos.column(), vis_pos.row());
        match slot_id {
            GridSlotId::Ability(_) => {
                // Intentionally skipped — see comment above.
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
        // Build cascade-resolved Buttonpos table so the unbutton normalizer can
        // detect collisions even when two abilities share the same stored position.
        let resolved_button_pos: Vec<(String, ButtonPosition)> = resolved
            .iter()
            .filter_map(|(slot, pos_opt)| {
                let GridSlotId::Ability(id) = slot else {
                    return None;
                };
                Some((id.clone(), (*pos_opt)?))
            })
            .collect();
        normalize_unbutton_positions(file, &resolved_button_pos);
    }
}

/// For each on-state ability in this container whose `UnButtonpos` collides
/// with another ability's cascade-resolved `Buttonpos`, move the `UnButtonpos`
/// to the ability's own cascade-resolved position (the self-cell).
///
/// `resolved_button_pos` is the output of `resolve_container` — it contains
/// cascade-aware positions rather than raw stored values, which matters when
/// two abilities share the same stored `Buttonpos` in this container.
fn normalize_unbutton_positions(
    file: &mut CustomKeysFile,
    resolved_button_pos: &[(String, ButtonPosition)],
) {
    for (ability_id, self_pos) in resolved_button_pos {
        let Some(unbutton_pos) = file
            .binding(ability_id.as_str())
            .and_then(|b| b.unbutton_position().copied())
        else {
            continue;
        };

        // Already at self-cell — valid, nothing to do.
        if unbutton_pos.column() == self_pos.column() && unbutton_pos.row() == self_pos.row() {
            continue;
        }

        // Collides with another ability's resolved button position → move to self-cell.
        let collides = resolved_button_pos.iter().any(|(other_id, other_pos)| {
            !other_id.eq_ignore_ascii_case(ability_id)
                && other_pos.column() == unbutton_pos.column()
                && other_pos.row() == unbutton_pos.row()
        });

        if collides {
            let new_pos = crate::ButtonPosition::new(self_pos.column(), self_pos.row());
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
