mod drag_state;
mod grid_cell;
mod tile_class;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use dioxus::prelude::*;
use warcraft_database::{BuildingTraits, ObjectLookup};
use warcraft_keybinds::{AbilityCell, ColumnIndex, CustomKeys, RowIndex};

use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::model::grid::{DragFollower, DraggingSlot, DropTargetCell, GridSlotId};
use crate::model::icons::IconUrl;
use crate::services::customkeys::positions::Positions;

use grid_cell::{GridCell, GridCellProps};
use tile_class::tile_class;

#[derive(Props, Clone, PartialEq)]
pub(crate) struct CommandGridSectionProps {
    pub(crate) heading: &'static str,
    pub(crate) slot_ids: Rc<[GridSlotId]>,
    pub(crate) loaded_keys: Signal<Option<CustomKeys>>,
    pub(crate) selected_slot: Signal<Option<GridSlotId>>,
    pub(crate) selected_from_research: Signal<bool>,
    pub(crate) selected_from_uprooted: Signal<bool>,
    pub(crate) tier_overrides: Signal<HashMap<String, usize>>,
    pub(crate) dragging_slot: Signal<Option<DraggingSlot>>,
    pub(crate) drop_target_cell: Signal<Option<DropTargetCell>>,
    pub(crate) drag_follower: Signal<Option<DragFollower>>,
    pub(crate) grid_layout: Signal<GridLayout>,
    #[props(default = false)]
    pub(crate) is_research_grid: bool,
    #[props(default = false)]
    pub(crate) is_uprooted_grid: bool,
    /// When true, drops onto cells already occupied by another slot are
    /// rejected outright instead of swapping. The off-state position
    /// picker uses this so dragging the toggle's off half can't displace
    /// another ability's on-state on the unit's command card.
    #[props(default = false)]
    pub(crate) prevent_swap_on_drop: bool,
    /// When non-empty, only slots whose `as_str()` matches one of these
    /// ids start a drag — other slots render in their cells but are
    /// display-only. Used by the off-state picker to keep the player from
    /// accidentally rearranging the unit's primary command card while
    /// editing one toggle's off position.
    #[props(default)]
    pub(crate) restrict_draggable_to: Vec<GridSlotId>,
    /// Unit ID of the host — used to block dragging of morph abilities on
    /// alternate-form units (e.g. Burrowed Crypt Fiend). Empty string
    /// disables the check (off-state picker, build menus without a unit).
    #[props(default)]
    pub(crate) host_unit_id: String,
}

#[component]
pub(crate) fn CommandGridSection(props: CommandGridSectionProps) -> Element {
    let read_guard = props.loaded_keys.read();
    let custom_keys_option = read_guard.as_ref();
    let layout_snapshot = *props.grid_layout.read();
    let active_slot = *props.selected_slot.read();
    let active_selection_is_research = *props.selected_from_research.read();

    let select_slot = props.selected_slot;
    let select_from_research = props.selected_from_research;
    let select_from_uprooted = props.selected_from_uprooted;
    let tier_overrides = props.tier_overrides;
    let is_research_grid = props.is_research_grid;
    let is_uprooted_grid = props.is_uprooted_grid;
    let dragging_slot = props.dragging_slot;
    let drop_target_cell = props.drop_target_cell;
    let drag_follower = props.drag_follower;
    let keys_signal = props.loaded_keys;
    let slot_ids_cloned = props.slot_ids.clone();
    let heading_text = props.heading;
    let prevent_swap_on_drop = props.prevent_swap_on_drop;
    let restrict_draggable_to: Rc<[GridSlotId]> = props.restrict_draggable_to.clone().into();
    let host_unit_id = props.host_unit_id.clone();
    let host_is_alt_form =
        !host_unit_id.is_empty() && BuildingTraits::unit_starts_in_toggle_alt_state(&host_unit_id);

    let conflicting_hotkeys: HashSet<String> = {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for row in 0..COMMAND_GRID_ROWS {
            for column in 0..COMMAND_GRID_COLUMNS {
                let cell_with_slot = Positions::cell_for_position(
                    &slot_ids_cloned,
                    custom_keys_option,
                    is_research_grid,
                    column,
                    row,
                );
                let letter = cell_with_slot.as_ref().and_then(|occupant| {
                    let cell = occupant.cell();
                    let token = if is_research_grid {
                        cell.binding_research_hotkey()
                            .or_else(|| cell.binding_hotkey())
                    } else {
                        cell.binding_hotkey()
                    };
                    token.map(|token| token.display_label())
                });
                if let Some(letter_label) = letter {
                    *counts.entry(letter_label).or_insert(0) += 1;
                }
            }
        }
        counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(key, _)| key)
            .collect()
    };

    rsx! {
        div { class: "command-section",
            h3 { class: "command-section-heading", "{heading_text}" }
            div { class: "grid-tiles",
                for row in 0..COMMAND_GRID_ROWS {
                    for column in 0..COMMAND_GRID_COLUMNS {
                        {
                            let cell_with_slot = Positions::cell_for_position(
                                &slot_ids_cloned,
                                custom_keys_option,
                                is_research_grid,
                                column,
                                row,
                            );
                            let raw_occupant_slot: Option<GridSlotId> =
                                cell_with_slot.as_ref().map(|occupant| occupant.slot_id());
                            // Show the off-state appearance when either:
                            // (a) a morph ability is on the unit it morphs INTO
                            //     (e.g. Bear Form on the bear unit → "Night Elf Form")
                            // (b) the host unit starts in the toggle alt-state and the
                            //     ability has an alt-state (e.g. militia's "Back to Work")
                            let morph_reverse_cell: Option<AbilityCell> =
                                raw_occupant_slot.as_ref().and_then(|slot| {
                                    let GridSlotId::Ability(ability_id) = slot else {
                                        return None;
                                    };
                                    let ability_id_str = ability_id.value();
                                    let binding =
                                        custom_keys_option.and_then(|file| file.binding(ability_id_str));
                                    if let Some(target) =
                                        ObjectLookup::morph_target_unit(ability_id_str)
                                        && target.eq_ignore_ascii_case(&host_unit_id) {
                                            return Some(AbilityCell::for_ability_off(
                                                *ability_id, binding,
                                            ));
                                        }
                                    if !is_uprooted_grid
                                        && BuildingTraits::unit_starts_in_toggle_alt_state(
                                            &host_unit_id,
                                        )
                                        && BuildingTraits::ability_has_alt_state(ability_id_str)
                                    {
                                        return Some(AbilityCell::for_ability_off(
                                            *ability_id, binding,
                                        ));
                                    }
                                    None
                                });
                            // Promote to AbilityOff when the tile is showing the
                            // reverse half of a toggle (Unburrow, Night Elf Form).
                            // This makes drag-and-drop write Unbuttonpos instead of
                            // Buttonpos and wires selection to the off-state inspector.
                            let occupant_slot: Option<GridSlotId> = if morph_reverse_cell.is_some() {
                                raw_occupant_slot.map(|slot| {
                                    let slot_object_id = slot.id();
                                    GridSlotId::ability_off(slot_object_id)
                                })
                            } else {
                                raw_occupant_slot
                            };
                            let cell_option: Option<&AbilityCell> = morph_reverse_cell
                                .as_ref()
                                .or_else(|| cell_with_slot.as_ref().map(|occupant| occupant.cell()));
                            let col_index = ColumnIndex::try_from(column).ok();
                            let row_index = RowIndex::try_from(row).ok();
                            let derived_letter = col_index
                                .zip(row_index)
                                .and_then(|(col, row_idx)| layout_snapshot.letter_at(col, row_idx));
                            let is_selected = match (&occupant_slot, active_slot.as_ref()) {
                                (Some(occupant), Some(active)) => {
                                    occupant == active
                                        && active_selection_is_research == is_research_grid
                                }
                                _ => false,
                            };
                            let is_command_cell = matches!(occupant_slot, Some(GridSlotId::Command(_)));
                            let (drag_in_progress_from_this_section, dragging_id_str) = {
                                let guard = dragging_slot.read();
                                match guard.as_ref().filter(|detail| detail.source_section() == heading_text) {
                                    Some(detail) => (true, Some(detail.slot_id().as_str().to_string())),
                                    None => (false, None),
                                }
                            };
                            let is_being_dragged = match (dragging_slot.read().as_ref(), &occupant_slot) {
                                (Some(dragging), Some(occupant)) => {
                                    dragging.slot_id() == occupant && dragging.source_section() == heading_text
                                }
                                _ => false,
                            };
                            let is_drop_target_cell = drag_in_progress_from_this_section
                                && drop_target_cell
                                    .read()
                                    .as_ref()
                                    .map(|target| {
                                        target.section() == heading_text
                                            && target.column() == column
                                            && target.row() == row
                                    })
                                    .unwrap_or(false);
                            // True when this empty cell is claimed by another
                            // ability's off-state — disallowed as a drop
                            // target so the off-state isn't silently displaced.
                            // The dragging ability itself is always exempt: it
                            // may always land on its own off-state cell.
                            let is_off_state_blocked = !is_research_grid
                                && drag_in_progress_from_this_section
                                && cell_option.is_none()
                                && slot_ids_cloned.iter().any(|slot| {
                                    let GridSlotId::Ability(ability_id) = slot else {
                                        return false;
                                    };
                                    if dragging_id_str.as_deref().is_some_and(|id| {
                                        ability_id.value().eq_ignore_ascii_case(id)
                                    }) {
                                        return false;
                                    }
                                    let bound_ability_id = *ability_id;
                                    Positions::current_for_ability_off(
                                        bound_ability_id,
                                        custom_keys_option,
                                    )
                                    .is_some_and(|off_pos| {
                                        off_pos.column().as_u8() == column
                                            && off_pos.row().as_u8() == row
                                    })
                                });
                            let class_name = tile_class(
                                cell_option.is_some(),
                                is_selected,
                                drag_in_progress_from_this_section,
                                is_command_cell,
                                is_being_dragged,
                                is_drop_target_cell,
                                is_off_state_blocked,
                            );
                            let cell_object_id_option = cell_option.map(|cell| cell.object_id());
                            let cell_tier_index = cell_object_id_option
                                .and_then(|id| tier_overrides.read().get(id.value()).copied())
                                .unwrap_or(0);
                            let cell_database_object = cell_object_id_option
                                .and_then(|id| ObjectLookup::by_id(id.value()));
                            let cell_tier_name = cell_database_object
                                .and_then(|warcraft_object| {
                                    warcraft_object.names().get(cell_tier_index).copied()
                                })
                                .map(String::from);
                            let cell_tier_icon = cell_database_object
                                .and_then(|warcraft_object| {
                                    warcraft_object.icons().get(cell_tier_index).copied()
                                })
                                .map(|raw_icon| IconUrl::from_database_path(raw_icon.trim()));
                            let label_text = if morph_reverse_cell.is_some() {
                                cell_option
                                    .as_ref()
                                    .map(|cell| cell.display_name().to_string())
                                    .unwrap_or_default()
                            } else {
                                cell_tier_name
                                    .clone()
                                    .or_else(|| cell_option.as_ref().map(|cell| cell.display_name().to_string()))
                                    .unwrap_or_default()
                            };
                            let icon_src_option = if cell_tier_index > 0 {
                                cell_tier_icon
                                    .or_else(|| cell_option.as_ref().and_then(|cell| cell.icon_path().map(IconUrl::from_icon_path)))
                            } else {
                                cell_option
                                    .as_ref()
                                    .and_then(|cell| cell.icon_path().map(IconUrl::from_icon_path))
                                    .or(cell_tier_icon)
                            };
                            let binding_letter_option = cell_option.as_ref().and_then(|cell| {
                                let token = if is_research_grid {
                                    cell.binding_research_hotkey()
                                        .or_else(|| cell.binding_hotkey())
                                } else {
                                    cell.binding_hotkey()
                                };
                                token.map(|value| value.display_label())
                            });
                            let is_passive_on_command_grid = !is_research_grid
                                && cell_object_id_option
                                    .map(|id| ObjectLookup::is_passive_ability(id.value()))
                                    .unwrap_or(false);
                            let displayed_letter: Option<String> = binding_letter_option
                                .clone()
                                .or_else(|| derived_letter.map(|character| character.to_string()));
                            let is_hotkey_conflict = displayed_letter
                                .as_ref()
                                .map(|label| conflicting_hotkeys.contains(label.as_str()))
                                .unwrap_or(false);
                            let hotkey_overlay_class = if is_hotkey_conflict {
                                "hotkey-overlay conflict"
                            } else if is_passive_on_command_grid {
                                "hotkey-overlay passive"
                            } else {
                                "hotkey-overlay"
                            };
                            let slot_ids_for_drop = slot_ids_cloned.clone();
                            let is_focusable_cell = cell_option.is_some();
                            // Tile is draggable iff there's no allow-list (the
                            // normal command card) or this tile's occupant
                            // matches one of the allowed slots (the picker
                            // dialog only lets the player grab the toggle's
                            // off half). AbilityOff slots are already the
                            // independent off-state half; only block the
                            // on-state Ability variant when it would write
                            // Buttonpos for a position that should be
                            // Unbuttonpos (one-way morphs without alt-state).
                            let is_morph_on_alt_form = matches!(occupant_slot, Some(GridSlotId::Ability(_)))
                                && occupant_slot
                                    .as_ref()
                                    .map(|slot| {
                                        let target_option =
                                            ObjectLookup::morph_target_unit(slot.as_str());
                                        let morphs_to_host = target_option.is_some_and(|target| {
                                            target.eq_ignore_ascii_case(&host_unit_id)
                                        });
                                        let alt_form_morph =
                                            host_is_alt_form && target_option.is_some();
                                        morphs_to_host || alt_form_morph
                                    })
                                    .unwrap_or(false);
                            let tile_is_draggable = !is_morph_on_alt_form
                                && (restrict_draggable_to.is_empty()
                                    || occupant_slot
                                        .as_ref()
                                        .map(|slot| {
                                            restrict_draggable_to
                                                .iter()
                                                .any(|allowed| allowed == slot)
                                        })
                                        .unwrap_or(false));
                            let restrict_draggable_to_for_cell = Rc::clone(&restrict_draggable_to);
                            let cell_props = GridCellProps {
                                class_name,
                                column,
                                row,
                                heading_text,
                                icon_src_option,
                                label_text,
                                displayed_letter,
                                hotkey_overlay_class,
                                is_focusable: is_focusable_cell,
                                tile_is_draggable,
                                is_research_grid,
                                is_uprooted_grid,
                                is_passive_on_command_grid,
                                is_command_cell,
                                prevent_swap_on_drop,
                                layout_snapshot,
                                restrict_draggable_to: restrict_draggable_to_for_cell,
                                selected_slot: select_slot,
                                selected_from_research: select_from_research,
                                selected_from_uprooted: select_from_uprooted,
                                dragging_slot,
                                drop_target_cell,
                                drag_follower,
                                keys_signal,
                                slot_ids_for_drop,
                                occupant_slot,
                            };
                            rsx! {
                                GridCell { ..cell_props }
                            }
                        }
                    }
                }
            }
        }
    }
}
