use dioxus::prelude::{ReadableExt, Signal, WritableExt};
use warcraft_api::{ButtonPosition, WarcraftObjectMeta};
use warcraft_keybinds::CustomKeysFile;

use crate::domain::ability_cell::{AbilityCell, BindingHotkey};
use crate::domain::command_catalog::CommandCatalog;
use crate::domain::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::domain::grid_slot::GridSlotId;
use crate::domain::object_lookup::ObjectLookup;

#[derive(Clone)]
struct ResolvedSlot {
    slot_id: GridSlotId,
    position: Option<ButtonPosition>,
}

pub(crate) struct Positions;

impl Positions {
    pub(crate) fn current_for(
        slot: &GridSlotId,
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<ButtonPosition> {
        match slot {
            GridSlotId::Ability(ability_id) => {
                Self::current_for_ability(ability_id, custom_keys, is_research_context)
            }
            GridSlotId::AbilityOff(ability_id) => {
                Self::current_for_ability_off(ability_id, custom_keys)
            }
            GridSlotId::Command(command_name) => {
                Self::current_for_command(command_name, custom_keys)
            }
        }
    }

    /// Off-state position for a toggle ability. Reads `Unbuttonpos` from the
    /// player's CustomKeys override first; falls through to the SLK default
    /// (`AbilityMeta::off_button_position`, parsed from `Unbuttonpos` in the
    /// game's `abilityfunc.txt`). Used by the override card to surface the
    /// off-state position for toggle abilities — they live alongside the
    /// regular `Ability` slot rather than as a second cell, so this is a
    /// direct accessor rather than going through `current_for`.
    pub(crate) fn current_for_ability_off(
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

    pub(crate) fn current_for_ability(
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

    pub(crate) fn current_for_command(
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
        Self::default_command_position(command_name)
    }

    pub(crate) fn default_command_position(command_name: &str) -> Option<ButtonPosition> {
        let warcraft_object = ObjectLookup::by_id(command_name)?;
        let WarcraftObjectMeta::Command(command_meta) = warcraft_object.meta() else {
            return None;
        };
        command_meta.default_button_position()
    }

    pub(crate) fn should_auto_position(slot: &GridSlotId) -> bool {
        let GridSlotId::Ability(ability_id) = slot else {
            return false;
        };
        let Some(warcraft_object) = ObjectLookup::by_id(ability_id) else {
            return false;
        };
        !matches!(warcraft_object.meta(), WarcraftObjectMeta::Ability(_))
    }

    pub(crate) fn resolved_for(
        slot: &GridSlotId,
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<ButtonPosition> {
        let resolved_entries =
            Self::resolve_container(candidate_slots, custom_keys, is_research_context);
        // Match on the full variant, not just the underlying id string —
        // `Ability("Adef")` and `AbilityOff("Adef")` share the same as_str
        // but are distinct slots that resolve to the on-state and off-state
        // positions respectively.
        let matching_entry = resolved_entries
            .iter()
            .find(|entry| slots_match(&entry.slot_id, slot))?;
        matching_entry.position
    }

    fn resolve_container(
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Vec<ResolvedSlot> {
        let mut entries: Vec<ResolvedSlot> = Vec::with_capacity(candidate_slots.len());
        let mut occupied_positions: Vec<ButtonPosition> = Vec::new();

        for candidate_slot in candidate_slots {
            let placeholder_entry = ResolvedSlot {
                slot_id: candidate_slot.clone(),
                position: None,
            };
            entries.push(placeholder_entry);
        }

        // Commands (Move, Stop, Attack, …) are placed first so their positions
        // are already occupied when abilities cascade.  A cascaded ability must
        // never land on a command slot — commands are first-class grid citizens.
        for entry in entries.iter_mut() {
            if !matches!(entry.slot_id, GridSlotId::Command(_)) {
                continue;
            }
            if CommandCatalog::is_context_command(&entry.slot_id) {
                continue;
            }
            let assigned_position = Self::resolve_with_cascade(
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
            let assigned_position = Self::resolve_with_cascade(
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
            let explicit_position =
                Self::current_for(&entry.slot_id, custom_keys, is_research_context);
            let Some(position_value) = explicit_position else {
                continue;
            };
            if Self::position_occupied(&occupied_positions, position_value) {
                continue;
            }
            entry.position = Some(position_value);
            occupied_positions.push(position_value);
        }

        entries
    }

    fn resolve_with_cascade(
        slot: &GridSlotId,
        occupied_positions: &[ButtonPosition],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
    ) -> Option<ButtonPosition> {
        let explicit_position = Self::current_for(slot, custom_keys, is_research_context);
        match explicit_position {
            Some(position_value) => {
                if Self::position_occupied(occupied_positions, position_value) {
                    Self::next_free_cell(position_value.row(), occupied_positions)
                } else {
                    Some(position_value)
                }
            }
            None => {
                if Self::should_auto_position(slot) || is_research_context {
                    Self::next_free_cell(0, occupied_positions)
                } else {
                    None
                }
            }
        }
    }

    fn position_occupied(occupied_positions: &[ButtonPosition], candidate: ButtonPosition) -> bool {
        occupied_positions.iter().any(|existing| {
            existing.column() == candidate.column() && existing.row() == candidate.row()
        })
    }

    /// Cascade within `preferred_row` first (left-to-right), then fall back
    /// to a full grid scan.  The game engine cascades within the same row when
    /// a button's desired position is already occupied, so we must do the same.
    fn next_free_cell(
        preferred_row: u8,
        occupied_positions: &[ButtonPosition],
    ) -> Option<ButtonPosition> {
        for column in 0..COMMAND_GRID_COLUMNS {
            let candidate = ButtonPosition::new(column, preferred_row);
            if !Self::position_occupied(occupied_positions, candidate) {
                return Some(candidate);
            }
        }
        for row in 0..COMMAND_GRID_ROWS {
            if row == preferred_row {
                continue;
            }
            for column in 0..COMMAND_GRID_COLUMNS {
                let candidate = ButtonPosition::new(column, row);
                if !Self::position_occupied(occupied_positions, candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }

    /// Returns both the resolved cell *and* the originating slot id at a
    /// given grid coordinate. Callers that need to identify which slot owns
    /// the cell (selection, drag/drop, click handlers) must compare the
    /// returned `GridSlotId` directly — looking the slot up later by the
    /// cell's `object_id` string would conflate `Ability("Adef")` with
    /// `AbilityOff("Adef")`, since both share the same id but represent
    /// different buttons (Defend vs. Stop Defend).
    pub(crate) fn cell_for_position(
        candidate_slots: &[GridSlotId],
        custom_keys: Option<&CustomKeysFile>,
        is_research_context: bool,
        column: u8,
        row: u8,
    ) -> Option<(GridSlotId, AbilityCell)> {
        for slot in candidate_slots {
            let Some(position) =
                Self::resolved_for(slot, candidate_slots, custom_keys, is_research_context)
            else {
                continue;
            };
            if position.column() == column && position.row() == row {
                let cell = match slot {
                    GridSlotId::Ability(ability_id) => {
                        let binding = custom_keys.and_then(|file| file.binding(ability_id));
                        AbilityCell::for_ability(ability_id, binding)
                    }
                    GridSlotId::AbilityOff(ability_id) => {
                        let binding = custom_keys.and_then(|file| file.binding(ability_id));
                        AbilityCell::for_ability_off(ability_id, binding)
                    }
                    GridSlotId::Command(command_name) => {
                        let binding = custom_keys.and_then(|file| file.command(command_name));
                        AbilityCell::for_command(command_name, binding)
                    }
                };
                return Some((slot.clone(), cell));
            }
        }
        None
    }

    /// Writes the off-state button position for a toggle ability without
    /// touching the on-state `Buttonpos` or `Hotkey`. Used by the
    /// override card's mini grid picker — the player drags the *off*
    /// half of Defend / Burrow / Bear Form to a new cell, and only
    /// `Unbuttonpos` should change. The on-state binding stays exactly
    /// where the main command card shows it. Currently unused at the
    /// callsite (the picker now goes through `move_or_swap` via the
    /// reused `CommandGridSection`); kept for click-to-place fallbacks
    /// and external scripts that want to write only the off position.
    #[allow(dead_code)]
    pub(crate) fn assign_off_position(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        ability_id: &str,
        column: u8,
        row: u8,
    ) {
        let new_position = warcraft_keybinds::ButtonPosition::new(column, row);
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        if let Some(binding) = file.binding_or_default_mut(ability_id) {
            binding.set_unbutton_position(Some(new_position));
        }
    }

    pub(crate) fn assign(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        layout: GridLayout,
        slot: &GridSlotId,
        column: u8,
        row: u8,
        is_research_context: bool,
    ) {
        let Some(letter) = layout.letter_at(column, row) else {
            return;
        };
        let new_position = warcraft_keybinds::ButtonPosition::new(column, row);
        let letter_string = letter.to_string();

        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));
        match slot {
            GridSlotId::Ability(ability_id) => {
                let is_passive = ObjectLookup::is_passive_ability(ability_id);
                if let Some(binding) = file.binding_or_default_mut(ability_id) {
                    if is_research_context {
                        binding.set_research_button_position(Some(new_position));
                        binding.set_research_hotkey(Some(letter_string));
                    } else {
                        binding.set_button_position(Some(new_position));
                        if !is_passive {
                            binding.set_hotkey(Some(letter_string));
                        }
                    }
                }
            }
            GridSlotId::AbilityOff(ability_id) => {
                // Off-state slots write `Unbuttonpos` and `Unhotkey` only —
                // the on-state's `Buttonpos` / `Hotkey` live on the
                // sibling `Ability` slot for the same id and stay put when
                // the player drags the off-state half.
                if let Some(binding) = file.binding_or_default_mut(ability_id) {
                    binding.set_unbutton_position(Some(new_position));
                    binding.set_unhotkey(Some(letter_string));
                }
            }
            GridSlotId::Command(command_name) => {
                if let Some(binding) = file.command_or_default_mut(command_name) {
                    binding.set_button_position(Some(new_position));
                    binding.set_hotkey(Some(letter_string));
                    binding.set_unbutton_position(Some(new_position));
                }
            }
        }
    }

    pub(crate) fn move_or_swap(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        request: MoveRequest<'_>,
    ) {
        let read_guard = custom_keys_signal.read();
        let custom_keys = read_guard.as_ref();
        let moving_old_position = Self::resolved_for(
            request.moving_slot,
            request.slot_ids,
            custom_keys,
            request.is_research_context,
        );
        let displaced_pair = Self::cell_for_position(
            request.slot_ids,
            custom_keys,
            request.is_research_context,
            request.target_column,
            request.target_row,
        );
        // Block only when the target is *empty* (no on-state occupant) but
        // already claimed by another ability's off-state — swapping onto an
        // occupied cell is handled below and co-moves the off-state.
        let off_state_blocks = displaced_pair.is_none()
            && !request.is_research_context
            && request.slot_ids.iter().any(|slot| {
                let GridSlotId::Ability(ability_id) = slot else {
                    return false;
                };
                // The moving toggle is always allowed to land on its own
                // off-state cell (the two halves overlap by default).
                if ability_id.eq_ignore_ascii_case(request.moving_slot.as_str()) {
                    return false;
                }
                Self::current_for_ability_off(ability_id, custom_keys).is_some_and(|off_pos| {
                    off_pos.column() == request.target_column && off_pos.row() == request.target_row
                })
            });
        // Pre-compute co-location flags while the read guard is still live.
        // When an ability's off-state sits on the same cell as its on-state
        // (the natural default layout), a swap should carry the off-state
        // along so the two halves stay together.
        let moving_off_colocated = !request.prevent_co_move
            && match (request.moving_slot, &moving_old_position) {
                (GridSlotId::Ability(id), Some(old_pos)) => {
                    Self::current_for_ability_off(id, custom_keys).is_some_and(|off_pos| {
                        off_pos.column() == old_pos.column() && off_pos.row() == old_pos.row()
                    })
                }
                _ => false,
            };
        let displaced_off_colocated = match &displaced_pair {
            Some((GridSlotId::Ability(id), _)) => Self::current_for_ability_off(id, custom_keys)
                .is_some_and(|off_pos| {
                    off_pos.column() == request.target_column && off_pos.row() == request.target_row
                }),
            _ => false,
        };
        drop(read_guard);

        if off_state_blocks {
            return;
        }

        let displaced_slot_option = displaced_pair.map(|(slot, _cell)| slot);
        // No-op when dropping a slot onto its own position (full variant +
        // id match — a `Ability("Adef")` drop onto `(0,2)` shouldn't
        // self-cancel just because `AbilityOff("Adef")` happens to live
        // there too).
        if let Some(ref displaced_slot) = displaced_slot_option
            && slots_match(displaced_slot, request.moving_slot)
        {
            return;
        }
        // The off-state position picker passes `prevent_swap` to keep
        // the off half from displacing another ability — drags onto
        // someone else's cell are rejected outright instead of swapping.
        // Overlap with the moving slot's *own on-state* (matching id,
        // different variant) is fine — that's the default toggle layout.
        if request.prevent_swap
            && let Some(ref displaced_slot) = displaced_slot_option
            && !displaced_slot
                .as_str()
                .eq_ignore_ascii_case(request.moving_slot.as_str())
        {
            return;
        }

        Self::assign(
            custom_keys_signal,
            request.layout,
            request.moving_slot,
            request.target_column,
            request.target_row,
            request.is_research_context,
        );

        // Co-move the dragged ability's off-state whenever it was at the
        // same cell — applies to empty targets as well as swaps so the
        // off-state doesn't stay stranded at the vacated source cell.
        if moving_off_colocated && let GridSlotId::Ability(moving_id) = request.moving_slot {
            Self::assign(
                custom_keys_signal,
                request.layout,
                &GridSlotId::AbilityOff(moving_id.clone()),
                request.target_column,
                request.target_row,
                false,
            );
        }

        if !request.prevent_swap
            && let (Some(displaced_slot), Some(old_position)) =
                (displaced_slot_option, moving_old_position)
        {
            let old_column = old_position.column();
            let old_row = old_position.row();
            Self::assign(
                custom_keys_signal,
                request.layout,
                &displaced_slot,
                old_column,
                old_row,
                request.is_research_context,
            );
            // Carry the displaced ability's off-state to the vacated cell
            // when the two halves were co-located at the drop target.
            if displaced_off_colocated && let GridSlotId::Ability(displaced_id) = &displaced_slot {
                Self::assign(
                    custom_keys_signal,
                    request.layout,
                    &GridSlotId::AbilityOff(displaced_id.clone()),
                    old_column,
                    old_row,
                    false,
                );
            }
        }
    }

    /// Resolve position conflicts in `file` so the stored `Buttonpos` values
    /// exactly match what the display would show. Must be called after every
    /// `UploadOverlay::apply` so the export/preview always mirrors the editor.
    ///
    /// Command positions are seeded first (they always win); then abilities are
    /// processed in file order — any ability whose stored position is already
    /// taken is cascaded to the next free cell on the same row and its
    /// `Buttonpos` is updated in place.
    pub(crate) fn normalize_button_positions(file: &mut CustomKeysFile) {
        let command_names: Vec<String> = file
            .commands_in_order()
            .map(|entry| entry.name().to_string())
            .collect();

        let mut occupied: Vec<ButtonPosition> = command_names
            .iter()
            .filter_map(|name| {
                let db_button = ObjectLookup::by_id(name).and_then(|o| o.default_button_position());
                let binding = file.command_or_default_mut(name)?;
                binding
                    .button_position()
                    .map(|p| ButtonPosition::new(p.column(), p.row()))
                    .or(db_button)
            })
            .collect();

        let ability_ids: Vec<String> = file
            .bindings_in_order()
            .map(|entry| entry.id().to_string())
            .collect();

        for id in &ability_ids {
            if ObjectLookup::is_passive_ability(id) {
                continue;
            }
            let Some(binding) = file.binding_or_default_mut(id) else {
                continue;
            };
            let Some(pos) = binding
                .button_position()
                .map(|p| ButtonPosition::new(p.column(), p.row()))
            else {
                continue;
            };
            if Self::position_occupied(&occupied, pos) {
                if let Some(new_pos) = Self::next_free_cell(pos.row(), &occupied) {
                    occupied.push(new_pos);
                    binding.set_button_position(Some(warcraft_keybinds::ButtonPosition::new(
                        new_pos.column(),
                        new_pos.row(),
                    )));
                }
            } else {
                occupied.push(pos);
            }
        }
    }

    pub(crate) fn apply_grid_to_all_known_objects(
        custom_keys_signal: &mut Signal<Option<CustomKeysFile>>,
        layout: GridLayout,
    ) -> usize {
        let mut changed_count: usize = 0;
        let mut writable_guard = custom_keys_signal.write();
        let file = writable_guard.get_or_insert_with(|| CustomKeysFile::from(""));

        // Iterate every binding actually present in the file. The previous
        // implementation walked WARCRAFT_DATABASE only and filtered by
        // `has_icon`, so any binding for an id we don't ship metadata for
        // (templates carry plenty — e.g. `Pick Shop Buyer` in Neo) was
        // skipped and kept its template-original hotkey, producing
        // collisions with the freshly-grid-aligned bindings of database
        // objects sharing the same column/row.
        let ability_ids: Vec<String> = file
            .bindings_in_order()
            .map(|entry| entry.id().to_string())
            .collect();
        let command_names: Vec<String> = file
            .commands_in_order()
            .map(|entry| entry.name().to_string())
            .collect();

        // Seed occupied positions with every command slot (Move, Stop, Attack,
        // …) so the ability cascade can never land on a command position.
        // Commands are resolved first — they are not lower priority than
        // abilities.
        let mut occupied_positions: Vec<ButtonPosition> = command_names
            .iter()
            .filter_map(|name| {
                let database_object = ObjectLookup::by_id(name);
                let database_button =
                    database_object.and_then(|object| object.default_button_position());
                let binding = file.command_or_default_mut(name)?;
                binding
                    .button_position()
                    .map(|p| ButtonPosition::new(p.column(), p.row()))
                    .or(database_button)
            })
            .collect();

        for ability_id in &ability_ids {
            let database_object = ObjectLookup::by_id(ability_id);
            let database_button =
                database_object.and_then(|object| object.default_button_position());
            let database_research =
                database_object.and_then(|object| object.default_research_button_position());
            let is_passive = ObjectLookup::is_passive_ability(ability_id);
            let Some(binding) = file.binding_or_default_mut(ability_id) else {
                continue;
            };

            if !is_passive {
                let explicit_button = binding
                    .button_position()
                    .map(|position| ButtonPosition::new(position.column(), position.row()));
                let resolved_position = match explicit_button {
                    Some(position) if !Self::position_occupied(&occupied_positions, position) => {
                        occupied_positions.push(position);
                        Some(position)
                    }
                    Some(position) => {
                        // Two abilities share the same explicit Buttonpos (e.g.
                        // a template assigned conflicting positions). Cascade to
                        // the next free cell on the same row and update the
                        // stored position so the hotkey and visual stay in sync.
                        let cascaded = Self::next_free_cell(position.row(), &occupied_positions);
                        if let Some(pos) = cascaded {
                            occupied_positions.push(pos);
                            let new_pos =
                                warcraft_keybinds::ButtonPosition::new(pos.column(), pos.row());
                            binding.set_button_position(Some(new_pos));
                        }
                        cascaded
                    }
                    None => {
                        let fallback = database_button;
                        let cascaded = match fallback {
                            Some(preferred)
                                if Self::position_occupied(&occupied_positions, preferred) =>
                            {
                                Self::next_free_cell(preferred.row(), &occupied_positions)
                            }
                            other => other,
                        };
                        if let Some(position) = cascaded {
                            occupied_positions.push(position);
                        }
                        cascaded
                    }
                };
                if let Some(position) = resolved_position
                    && let Some(letter) = layout.letter_at(position.column(), position.row())
                    && BindingHotkey::accepts_grid_letter(binding.hotkey())
                {
                    let new_hotkey = letter.to_string();
                    if binding.hotkey() != Some(new_hotkey.as_str()) {
                        binding.set_hotkey(Some(new_hotkey));
                        changed_count += 1;
                    }
                }
            }

            let research_position = binding
                .research_button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()))
                .or(database_research);
            if let Some(position) = research_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && BindingHotkey::accepts_grid_letter(binding.research_hotkey())
            {
                let new_research_hotkey = letter.to_string();
                if binding.research_hotkey() != Some(new_research_hotkey.as_str()) {
                    binding.set_research_hotkey(Some(new_research_hotkey));
                    changed_count += 1;
                }
            }
        }

        for command_name in &command_names {
            let database_object = ObjectLookup::by_id(command_name);
            let database_button =
                database_object.and_then(|object| object.default_button_position());
            let Some(binding) = file.command_or_default_mut(command_name) else {
                continue;
            };
            let cast_position = binding
                .button_position()
                .map(|position| ButtonPosition::new(position.column(), position.row()))
                .or(database_button);
            if let Some(position) = cast_position
                && let Some(letter) = layout.letter_at(position.column(), position.row())
                && BindingHotkey::accepts_grid_letter(binding.hotkey())
            {
                let new_hotkey = letter.to_string();
                if binding.hotkey() != Some(new_hotkey.as_str()) {
                    binding.set_hotkey(Some(new_hotkey));
                    changed_count += 1;
                }
            }
        }

        changed_count
    }
}

pub(crate) struct MoveRequest<'a> {
    layout: GridLayout,
    slot_ids: &'a [GridSlotId],
    moving_slot: &'a GridSlotId,
    target_column: u8,
    target_row: u8,
    is_research_context: bool,
    /// When true, drops onto a cell occupied by *another* slot are
    /// rejected (no swap). Used by the off-state position picker —
    /// dragging the off half of a toggle should never displace another
    /// ability's on-state. Drops onto the host's own on-state cell are
    /// always allowed regardless of this flag (overlap is the natural
    /// default for toggle abilities).
    prevent_swap: bool,
    /// When true, suppress the automatic co-movement of an ability's
    /// off-state when the on-state is dragged. Used for grids where the
    /// off-state is independently positionable in a separate grid (e.g.
    /// the uprooted-panel's Root slot and the rooted-panel's Uproot slot).
    prevent_co_move: bool,
}

impl<'a> MoveRequest<'a> {
    pub(crate) fn new(
        layout: GridLayout,
        slot_ids: &'a [GridSlotId],
        moving_slot: &'a GridSlotId,
        target_column: u8,
        target_row: u8,
        is_research_context: bool,
    ) -> Self {
        Self {
            layout,
            slot_ids,
            moving_slot,
            target_column,
            target_row,
            is_research_context,
            prevent_swap: false,
            prevent_co_move: false,
        }
    }

    /// Currently unused — the off-state position picker is click-to-place
    /// and writes through `assign_off_position` directly, never going
    /// through `move_or_swap`. Kept for the future drag-and-drop picker
    /// that would feed an `AbilityOff` slot into `CommandGridSection`.
    #[allow(dead_code)]
    pub(crate) fn with_prevent_swap(mut self, prevent: bool) -> Self {
        self.prevent_swap = prevent;
        self
    }

    pub(crate) fn with_prevent_co_move(mut self, prevent: bool) -> Self {
        self.prevent_co_move = prevent;
        self
    }
}

/// Compares two slots by full variant *and* id, case-insensitive on the id.
fn slots_match(slot_a: &GridSlotId, slot_b: &GridSlotId) -> bool {
    match (slot_a, slot_b) {
        (GridSlotId::Ability(left), GridSlotId::Ability(right))
        | (GridSlotId::AbilityOff(left), GridSlotId::AbilityOff(right))
        | (GridSlotId::Command(left), GridSlotId::Command(right)) => {
            left.eq_ignore_ascii_case(right)
        }
        _ => false,
    }
}
