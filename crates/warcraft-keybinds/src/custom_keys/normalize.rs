//! The normalization / materialization pipeline: the domain service that
//! turns a raw parsed `CustomKeys` into the fully-materialized, cascade-resolved
//! canonical form the aggregate always holds. Split out of the aggregate root as
//! an `impl CustomKeys` continuation; a child module sees the parent's private
//! fields and helpers.

use super::mirrors::{BUILD_COMMAND_MIRRORS, MORPH_ABILITY_MIRRORS};
use super::{BUNDLED_BASELINE, CustomKeys, GRID_COLUMNS, GRID_ROWS};
use crate::identity::slot::GridSlotId;
use crate::model::{ColumnIndex, GridCoordinate, Hotkey, RowIndex, WarcraftKeybinding};
use crate::unit::grids::UnitGrids;
use crate::unit::slots::UnitCommandSlots;
use std::collections::HashSet;
use std::sync::OnceLock;
use warcraft_api::{WarcraftObjectKind, WarcraftObjectMeta};
use warcraft_database::WARCRAFT_DATABASE;

impl CustomKeys {
    pub fn normalize(&self) -> Self {
        let mut result = Self::materialized_baseline().clone();
        let overlay_clone = self.clone();
        result.extend(overlay_clone);
        result.prune_non_button_abilities();
        result.mirror_build_commands_to_abilities();
        result.mirror_morph_abilities_to_unit_commands();
        result.sync_mirrored_off_states();
        result.materialize_upgrade_hotkey_tiers();
        result
    }

    /// Restores one hotkey token per tier for every multi-level upgrade. The
    /// live game binds each research tier from a comma-separated list
    /// (`Hotkey=F,F,F`); a single token binds only tier 1, silently dropping the
    /// follow-up upgrades' hotkeys. Templates and older files routinely store a
    /// single token, and `extend` overlays it onto the baseline, so normalize is
    /// the one place every flow (template apply, import, boot) passes through and
    /// must re-materialize the per-tier list. Leveled abilities are left alone —
    /// only [`WarcraftObject::upgrade_max_level`] objects replicate.
    fn materialize_upgrade_hotkey_tiers(&mut self) {
        for (object_id, keybinding) in self.entries.iter_mut() {
            let WarcraftKeybinding::Ability(ability_binding) = keybinding else {
                continue;
            };
            let object_code = object_id.value();
            let object_option = WARCRAFT_DATABASE.by_id(object_code);
            let Some(warcraft_object) = object_option else {
                continue;
            };
            let Some(tier_count) = warcraft_object.upgrade_max_level() else {
                continue;
            };
            if tier_count < 2 {
                continue;
            }
            let existing_main = ability_binding.hotkey();
            let main_replicated = Self::replicate_existing_hotkey(existing_main, tier_count);
            if let Some(new_hotkey) = main_replicated {
                ability_binding.set_hotkey(Some(new_hotkey));
            }
            let existing_research = ability_binding.research_hotkey();
            let research_replicated =
                Self::replicate_existing_hotkey(existing_research, tier_count);
            if let Some(new_hotkey) = research_replicated {
                ability_binding.set_research_hotkey(Some(new_hotkey));
            }
        }
    }

    /// Returns the per-tier replication of an existing hotkey's first token, or
    /// `None` when there is nothing to change (no hotkey, or already the right
    /// number of tiers).
    fn replicate_existing_hotkey(existing: Option<&Hotkey>, tier_count: usize) -> Option<Hotkey> {
        let hotkey = existing?;
        let token = hotkey.first_token()?;
        if hotkey.level_count() == tier_count {
            return None;
        }
        let replicated = Hotkey::replicated(token, tier_count);
        Some(replicated)
    }

    /// Collapses every toggle that does not get a second command card slot onto
    /// a single cell. Such a toggle keeps both states on one unit's grid, but a
    /// cell can render only one of them: the editor shows the off-state in the
    /// grid (`Buttonpos`) and edits the on-state (`Unbuttonpos`) only through a
    /// separate dialog. If that dialog-only on-state drifts off the grid cell
    /// (e.g. an overlay that set only `Buttonpos` — templates rarely carry an
    /// `Unbuttonpos`), it becomes an invisible blocker: the cascade collides
    /// with it but nothing in the grid shows why. Morph toggles (states on
    /// separate unit grids) and building alt-state toggles (two buttons on the
    /// building's grid) render both states, so they keep their own positions.
    fn sync_mirrored_off_states(&mut self) {
        let independent_off_slots = Self::abilities_with_independent_off_slot();
        for (object_id, keybinding) in self.entries.iter_mut() {
            let WarcraftKeybinding::Ability(ability_binding) = keybinding else {
                continue;
            };
            if ability_binding.unbutton_position().is_none() {
                continue;
            }
            let ability_lowercase = object_id.value().to_ascii_lowercase();
            if independent_off_slots.contains(&ability_lowercase) {
                continue;
            }
            let Some(button_position) = ability_binding.button_position().copied() else {
                continue;
            };
            ability_binding.set_unbutton_position(Some(button_position));
        }
    }

    /// Copies each worker build command's position and hotkey onto the build
    /// ability the live game renders for it (see [`BuildCommandMirror`]). The
    /// game ignores the command's position in a live match and reads the
    /// ability instead, so without this mirror it falls back to the ability's
    /// default cell and slides it on collision.
    fn mirror_build_commands_to_abilities(&mut self) {
        for mirror in BUILD_COMMAND_MIRRORS {
            let command_id = mirror.command_id();
            let command_name = command_id.value();
            let Some(command_binding) = self.command(command_name) else {
                continue;
            };
            let position_ref = command_binding.button_position();
            let button_position = position_ref.copied();
            let hotkey_ref = command_binding.hotkey();
            let hotkey = hotkey_ref.cloned();
            if button_position.is_none() && hotkey.is_none() {
                continue;
            }
            let ability_id = mirror.ability_id();
            let existing_binding = self.binding(ability_id).cloned();
            let mut ability_binding = existing_binding.unwrap_or_default();
            ability_binding.set_button_position(button_position);
            ability_binding.set_hotkey(hotkey);
            self.put_ability(ability_id, ability_binding);
        }
    }

    /// Copies each transform morph ability's hotkey and position onto the
    /// produced-unit section the live game reads (see [`MorphAbilityMirror`]).
    /// Without it the produced unit keeps its stale default hotkey, so the morph
    /// button binds the wrong key in game even though its rendered position
    /// follows the edited morph ability.
    fn mirror_morph_abilities_to_unit_commands(&mut self) {
        for mirror in MORPH_ABILITY_MIRRORS {
            let ability_id = mirror.ability_id();
            let Some(ability_binding) = self.binding(ability_id) else {
                continue;
            };
            let position_ref = ability_binding.button_position();
            let button_position = position_ref.copied();
            let hotkey_ref = ability_binding.hotkey();
            let hotkey = hotkey_ref.cloned();
            if button_position.is_none() && hotkey.is_none() {
                continue;
            }
            let produced_unit_id = mirror.produced_unit_id();
            let existing_binding = self.binding(produced_unit_id).cloned();
            let mut produced_binding = existing_binding.unwrap_or_default();
            produced_binding.set_button_position(button_position);
            produced_binding.set_hotkey(hotkey);
            self.put_ability(produced_unit_id, produced_binding);
        }
    }

    /// Drops ability bindings that never render as a real command button —
    /// internal shop/selection mechanics (Shop Sharing, Select Hero/Unit) and
    /// the Detector passive. The shipped baseline ships these as `[Aall]` etc.
    /// at `Buttonpos=0,0` with the Q hotkey, so without this prune every shop
    /// carries a phantom Q binding that collides with its real button in-game
    /// even though no command card displays it. A binding is removed only when
    /// its database object is an ability with no displayable icon, so genuine
    /// buttons such as Select User (`Anei`) are kept.
    fn prune_non_button_abilities(&mut self) {
        self.entries.retain(|object_id, binding| {
            if !matches!(binding, WarcraftKeybinding::Ability(_)) {
                return true;
            }
            let object_option = WARCRAFT_DATABASE.by_id(object_id.value());
            let Some(warcraft_object) = object_option else {
                return true;
            };
            if warcraft_object.kind() != WarcraftObjectKind::Ability {
                return true;
            }
            warcraft_object.has_displayable_icon()
        });
    }

    fn materialized_baseline() -> &'static Self {
        static CACHE: OnceLock<CustomKeys> = OnceLock::new();
        CACHE.get_or_init(|| {
            let mut file = Self::parse_raw(BUNDLED_BASELINE);
            file.materialize_default_positions();
            file.materialize_shop_item_positions();
            file
        })
    }

    pub fn serialize(&self, baseline: &str) -> String {
        let mut export_file = Self::parse_raw(baseline);
        let overlay_clone = self.clone();
        export_file.extend(overlay_clone);
        export_file.materialize_default_positions();
        export_file.materialize_shop_item_positions();
        export_file.prune_non_button_abilities();
        export_file.to_string()
    }

    /// Ability ids (lowercase) whose two toggle states each get their own
    /// command card slot, so both stay independently visible and positionable.
    /// Two cases qualify: morph toggles, where morphing swaps in a separate unit
    /// id that has its own command grid, and building alt-state toggles (Call To
    /// Arms / Back To Work), which sit on the building's grid as two distinct
    /// buttons. Every other toggle keeps both states on one unit's grid, where a
    /// cell can render only one of them — the editor shows the off-state in the
    /// grid and edits the on-state in a separate dialog — so its dialog-only
    /// state must track the one shown in the grid.
    fn abilities_with_independent_off_slot() -> &'static HashSet<String> {
        static CACHE: OnceLock<HashSet<String>> = OnceLock::new();
        CACHE.get_or_init(|| {
            let mut independent: HashSet<String> = HashSet::new();
            for unit_id in WARCRAFT_DATABASE.all_unit_ids() {
                let unit_grids = UnitGrids::for_unit(unit_id);
                for named_grid in unit_grids.grids() {
                    for slot in named_grid.card().filled_slots() {
                        if let GridSlotId::AbilityOff(ability_id) = slot {
                            let lowercase = ability_id.value().to_ascii_lowercase();
                            independent.insert(lowercase);
                        }
                    }
                }
            }
            independent
        })
    }

    fn materialize_default_positions(&mut self) {
        for (object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let default_button = warcraft_object.default_button_position();
            let default_research = warcraft_object.default_research_button_position();
            match warcraft_object.kind() {
                WarcraftObjectKind::Command => continue,
                WarcraftObjectKind::Ability => {
                    if !warcraft_object.has_displayable_icon() {
                        continue;
                    }
                    let needs_origin_fallback = default_button.is_none()
                        && default_research.is_none()
                        && !warcraft_object.is_passive_ability()
                        && matches!(
                            warcraft_object.meta(),
                            WarcraftObjectMeta::Ability(meta)
                            if meta.has_off_state()
                        );
                    if default_button.is_none()
                        && default_research.is_none()
                        && !needs_origin_fallback
                    {
                        continue;
                    }
                    let canonical_id = *object_id;
                    let Some(binding) = self.binding_or_default_mut(canonical_id) else {
                        continue;
                    };
                    if binding.button_position().is_none() {
                        if let Some(position) = default_button {
                            binding.set_button_position(Some(position));
                        } else if needs_origin_fallback {
                            let origin = GridCoordinate::new(ColumnIndex::Zero, RowIndex::Zero);
                            binding.set_button_position(Some(origin));
                        }
                    }
                    if binding.research_button_position().is_none()
                        && let Some(position) = default_research
                    {
                        binding.set_research_button_position(Some(position));
                    }
                    if binding.unbutton_position().is_none()
                        && !warcraft_object.is_passive_ability()
                        && let WarcraftObjectMeta::Ability(ability_meta) = warcraft_object.meta()
                        && ability_meta.has_off_state()
                    {
                        let database_off = ability_meta.off_button_position();
                        if let Some(off_position) = database_off {
                            binding.set_unbutton_position(Some(off_position));
                        } else if let Some(button_position) = binding.button_position() {
                            let position_copy = *button_position;
                            binding.set_unbutton_position(Some(position_copy));
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    fn materialize_shop_item_positions(&mut self) {
        for (_object_id, warcraft_object) in WARCRAFT_DATABASE.iter() {
            let WarcraftObjectMeta::Unit(unit_meta) = warcraft_object.meta() else {
                continue;
            };
            let sell_items = unit_meta.sell_items();
            let sell_units = unit_meta.sell_units();
            if sell_items.is_empty() && sell_units.is_empty() {
                continue;
            }
            let mut occupied_positions: HashSet<GridCoordinate> = HashSet::new();
            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let existing_position = position_ref.copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }
            for unit_id_object in sell_units {
                let unit_id = unit_id_object.value();
                let unit_binding = self.binding(unit_id);
                let position_ref = unit_binding.and_then(|binding| binding.button_position());
                let existing_position = position_ref.copied();
                if let Some(position) = existing_position {
                    occupied_positions.insert(position);
                }
            }
            for item_id_object in sell_items {
                let item_id = item_id_object.value();
                let item_binding = self.binding(item_id);
                let position_ref = item_binding.and_then(|binding| binding.button_position());
                let has_position = position_ref.is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                let item_canonical_id = *item_id_object;
                if let Some(item_binding) = self.binding_or_default_mut(item_canonical_id) {
                    item_binding.set_button_position(Some(free_position));
                }
            }
            for unit_id_object in sell_units {
                let unit_id = unit_id_object.value();
                let unit_binding = self.binding(unit_id);
                let position_ref = unit_binding.and_then(|binding| binding.button_position());
                let has_position = position_ref.is_some();
                if has_position {
                    continue;
                }
                let Some(free_position) = Self::next_free_grid_cell(&occupied_positions) else {
                    continue;
                };
                occupied_positions.insert(free_position);
                let unit_canonical_id = *unit_id_object;
                if let Some(unit_binding) = self.binding_or_default_mut(unit_canonical_id) {
                    unit_binding.set_button_position(Some(free_position));
                }
            }
        }
    }

    fn next_free_grid_cell(occupied_positions: &HashSet<GridCoordinate>) -> Option<GridCoordinate> {
        for row in 0..GRID_ROWS {
            for column in 0..GRID_COLUMNS {
                let Ok(column) = ColumnIndex::try_from(column) else {
                    continue;
                };
                let Ok(row) = RowIndex::try_from(row) else {
                    continue;
                };
                let candidate = GridCoordinate::new(column, row);
                if !occupied_positions.contains(&candidate) {
                    return Some(candidate);
                }
            }
        }
        None
    }
}
