use std::sync::OnceLock;

use warcraft_api::WarcraftObjectMeta;
use warcraft_keybinds::CustomKeysFile;

use warcraft_api::UnitKind;

use crate::customkeys::baseline::BASELINE_CUSTOM_KEYS;
use crate::customkeys::upload_overlay::UploadOverlay;
use crate::domain::unit_kind::UnitKindHelpers;

use crate::domain::ability_cell::AbilityCell;
use crate::domain::command_catalog::CommandCatalog;
use crate::domain::grid_layout::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::domain::grid_slot::GridSlotId;
use crate::domain::object_lookup::ObjectLookup;
use crate::domain::positions::Positions;

const ARCHMAGE_UNIT_ID: &str = "Hamg";

#[derive(PartialEq)]
pub(crate) struct BundledTemplate {
    name: &'static str,
    description: &'static str,
    content: &'static str,
}

impl BundledTemplate {
    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.description
    }

    pub(crate) fn content(&self) -> &'static str {
        self.content
    }
}

pub(crate) const TEMPLATES: &[BundledTemplate] = &[
    BundledTemplate {
        name: "NEO (QWERTY)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTY keyboards",
        content: include_str!("../../templates/CustomKeys_Neo_QWERTY.txt"),
    },
    BundledTemplate {
        name: "NEO (QWERTZ)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTZ keyboards",
        content: include_str!("../../templates/CustomKeys_Neo_QWERTZ.txt"),
    },
    BundledTemplate {
        name: "NEO (AZERTY)",
        description: "Neo's (Back2Warcraft) optimized layout converted for AZERTY keyboards",
        content: include_str!("../../templates/CustomKeys_Neo_AZERTY.txt"),
    },
    BundledTemplate {
        name: "Default",
        description: "Stock Warcraft III hotkeys, exactly what the game ships with",
        content: include_str!("../../templates/CustomKeys.txt"),
    },
];

#[derive(Clone, PartialEq)]
pub(crate) struct ResolvedTemplate {
    template: &'static BundledTemplate,
    grid: GridLayout,
    command_card_cells: Vec<Vec<Option<AbilityCell>>>,
    research_menu_cells: Vec<Vec<Option<AbilityCell>>>,
}

impl ResolvedTemplate {
    pub(crate) fn template(&self) -> &'static BundledTemplate {
        self.template
    }

    pub(crate) fn grid(&self) -> &GridLayout {
        &self.grid
    }

    pub(crate) fn command_card_cell(&self, column: u8, row: u8) -> Option<&AbilityCell> {
        let row_index = usize::from(row);
        let column_index = usize::from(column);
        let row_slot = self.command_card_cells.get(row_index)?;
        row_slot.get(column_index)?.as_ref()
    }

    pub(crate) fn research_menu_cell(&self, column: u8, row: u8) -> Option<&AbilityCell> {
        let row_index = usize::from(row);
        let column_index = usize::from(column);
        let row_slot = self.research_menu_cells.get(row_index)?;
        row_slot.get(column_index)?.as_ref()
    }

    pub(crate) fn resolve_all() -> Vec<ResolvedTemplate> {
        let command_slots = ArchmageTemplate::command_card_slots();
        let research_slots = ArchmageTemplate::research_menu_slots();
        TEMPLATES
            .iter()
            .map(|template| {
                let parsed_file = CustomKeysFile::from(template.content);
                let derived_grid = GridLayout::derived_from(&parsed_file);
                let mut preview_file = CustomKeysFile::from(BASELINE_CUSTOM_KEYS);
                UploadOverlay::apply(&mut preview_file, &parsed_file);
                let command_card_cells = CellGrid::populate(&preview_file, command_slots, false);
                let research_menu_cells = CellGrid::populate(&preview_file, research_slots, true);
                ResolvedTemplate {
                    template,
                    grid: derived_grid,
                    command_card_cells,
                    research_menu_cells,
                }
            })
            .collect()
    }
}

struct CellGrid;

impl CellGrid {
    fn populate(
        file: &CustomKeysFile,
        slot_ids: &[GridSlotId],
        is_research_context: bool,
    ) -> Vec<Vec<Option<AbilityCell>>> {
        let row_count = usize::from(COMMAND_GRID_ROWS);
        let column_count = usize::from(COMMAND_GRID_COLUMNS);
        (0..row_count)
            .map(|row_index| {
                (0..column_count)
                    .map(|column_index| {
                        let row_u8 = u8::try_from(row_index).unwrap_or(0);
                        let column_u8 = u8::try_from(column_index).unwrap_or(0);
                        Positions::cell_for_position(
                            slot_ids,
                            Some(file),
                            is_research_context,
                            column_u8,
                            row_u8,
                        )
                    })
                    .collect()
            })
            .collect()
    }
}

static ARCHMAGE_COMMAND_CARD_SLOTS: OnceLock<Vec<GridSlotId>> = OnceLock::new();
static ARCHMAGE_RESEARCH_MENU_SLOTS: OnceLock<Vec<GridSlotId>> = OnceLock::new();

struct ArchmageTemplate;

impl ArchmageTemplate {
    fn command_card_slots() -> &'static [GridSlotId] {
        ARCHMAGE_COMMAND_CARD_SLOTS.get_or_init(Self::compute_command_card_slots)
    }

    fn research_menu_slots() -> &'static [GridSlotId] {
        ARCHMAGE_RESEARCH_MENU_SLOTS.get_or_init(Self::compute_research_menu_slots)
    }

    fn compute_command_card_slots() -> Vec<GridSlotId> {
        let Some(unit_object) = ObjectLookup::by_id(ARCHMAGE_UNIT_ID) else {
            return Vec::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Vec::new();
        };
        let primary_commands =
            CommandCatalog::primary_commands_for(unit_meta, unit_object.race(), ARCHMAGE_UNIT_ID);
        let regular_abilities = unit_meta.abilities();
        let hero_abilities = unit_meta.hero_abilities();
        let mut slots: Vec<GridSlotId> = Vec::new();
        for command_name in primary_commands {
            if !ObjectLookup::has_icon(command_name) {
                continue;
            }
            slots.push(GridSlotId::command(command_name));
        }
        for ability_id in regular_abilities.iter().chain(hero_abilities.iter()) {
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id.value()));
        }
        let unit_is_hero = UnitKindHelpers::effective_kind(unit_meta) == UnitKind::Hero;
        if unit_is_hero
            && !hero_abilities.is_empty()
            && let Some(select_skill_command) = CommandCatalog::known_command("CmdSelectSkill")
            && ObjectLookup::has_icon(select_skill_command)
        {
            slots.push(GridSlotId::command(select_skill_command));
        }
        slots
    }

    fn compute_research_menu_slots() -> Vec<GridSlotId> {
        let Some(unit_object) = ObjectLookup::by_id(ARCHMAGE_UNIT_ID) else {
            return Vec::new();
        };
        let WarcraftObjectMeta::Unit(unit_meta) = unit_object.meta() else {
            return Vec::new();
        };
        let hero_abilities = unit_meta.hero_abilities();
        if hero_abilities.is_empty() {
            return Vec::new();
        }
        let mut slots: Vec<GridSlotId> = Vec::new();
        for ability_id in hero_abilities.iter() {
            if !ObjectLookup::has_icon(ability_id.value()) {
                continue;
            }
            slots.push(GridSlotId::ability(ability_id.value()));
        }
        if let Some(cancel_command) = CommandCatalog::known_command("CmdCancel")
            && ObjectLookup::has_icon(cancel_command)
        {
            slots.push(GridSlotId::command(cancel_command));
        }
        slots
    }
}
