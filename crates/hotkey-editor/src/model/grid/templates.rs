use std::sync::OnceLock;

use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;
use warcraft_keybinds::{AbilityCell, CustomKeys, UnitCommandSlots};

use crate::model::grid::GridSlotId;
use crate::model::grid::{COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, GridLayout};
use crate::services::customkeys::positions::Positions;

pub(crate) struct BundledTemplate {
    name: &'static str,
    description: &'static str,
    content: fn() -> &'static str,
}

impl PartialEq for BundledTemplate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl BundledTemplate {
    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn description(&self) -> &'static str {
        self.description
    }

    pub(crate) fn content(&self) -> &'static str {
        (self.content)()
    }
}

pub(crate) const TEMPLATES: &[BundledTemplate] = &[
    BundledTemplate {
        name: "NEO (QWERTY)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTY keyboards",
        content: || include_str!("../../../templates/CustomKeys_Neo_QWERTY.txt"),
    },
    BundledTemplate {
        name: "NEO (QWERTZ)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTZ keyboards",
        content: || include_str!("../../../templates/CustomKeys_Neo_QWERTZ.txt"),
    },
    BundledTemplate {
        name: "NEO (AZERTY)",
        description: "Neo's (Back2Warcraft) optimized layout converted for AZERTY keyboards",
        content: || include_str!("../../../templates/CustomKeys_Neo_AZERTY.txt"),
    },
    BundledTemplate {
        name: "Default",
        description: "Stock Warcraft III hotkeys, exactly what the game ships with",
        content: || warcraft_keybinds::DEFAULT_CUSTOM_KEYS,
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
        static CACHE: OnceLock<Vec<ResolvedTemplate>> = OnceLock::new();
        CACHE.get_or_init(Self::compute_all).clone()
    }

    fn compute_all() -> Vec<ResolvedTemplate> {
        let archmage_id = WarcraftObjectId::new("Hamg");
        let command_slots: Vec<GridSlotId> = WARCRAFT_DATABASE
            .command_card(archmage_id)
            .filled_slots()
            .collect();
        let research_slots: Vec<GridSlotId> = WARCRAFT_DATABASE
            .research_menu(archmage_id)
            .into_iter()
            .flat_map(|card| card.filled_slots().collect::<Vec<_>>())
            .collect();
        TEMPLATES
            .iter()
            .map(|template| {
                let parsed_file = CustomKeys::from(template.content());
                let derived_grid = GridLayout::derived_from(&parsed_file);
                let mut preview_file = CustomKeys::from(warcraft_keybinds::DEFAULT_CUSTOM_KEYS);
                preview_file.extend(parsed_file);
                let command_card_cells = CellGrid::populate(&preview_file, &command_slots, false);
                let research_menu_cells = CellGrid::populate(&preview_file, &research_slots, true);
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
        file: &CustomKeys,
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
                        .map(|occupant| occupant.cell().clone())
                    })
                    .collect()
            })
            .collect()
    }
}
