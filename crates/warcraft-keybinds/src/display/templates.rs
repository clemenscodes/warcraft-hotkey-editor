//! Bundled keybind presets and their paint-ready previews. The preset texts are
//! domain data (CustomKeys content); resolving each into a renderable grid is
//! domain work. The UI only paints the resulting tiles.

use crate::custom_keys::CustomKeys;
use crate::custom_keys::DEFAULT_CUSTOM_KEYS;
use crate::display::grid_behavior::{CommandBehavior, ResearchBehavior};
use crate::display::rendered_grid::{CommandGridRenderInput, RenderedTile};
use crate::grid::layout::GridLayout;
use crate::identity::slot::GridSlotId;
use crate::unit::slots::UnitCommandSlots;
use std::collections::HashMap;
use std::sync::OnceLock;
use warcraft_api::WarcraftObjectId;
use warcraft_database::WARCRAFT_DATABASE;

/// A bundled preset: a name, a one-line description, and the CustomKeys text it
/// ships.
#[derive(Clone, Copy, Debug)]
pub struct BundledTemplate {
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
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn description(&self) -> &'static str {
        self.description
    }

    pub fn content(&self) -> &'static str {
        (self.content)()
    }
}

const TEMPLATES: &[BundledTemplate] = &[
    BundledTemplate {
        name: "Default",
        description: "Stock Warcraft III hotkeys, exactly what the game ships with",
        content: || DEFAULT_CUSTOM_KEYS,
    },
    BundledTemplate {
        name: "Clemens (QWERTY)",
        description: "Clemens' DotA-like layout for QWERTY keyboards",
        content: || include_str!("../../templates/CustomKeys_Clemens_DotA_QWERTY.txt"),
    },
    BundledTemplate {
        name: "Clemens (QWERTZ)",
        description: "Clemens' DotA-like layout for QWERTZ keyboards",
        content: || include_str!("../../templates/CustomKeys_Clemens_DotA_QWERTZ.txt"),
    },
    BundledTemplate {
        name: "Clemens (AZERTY)",
        description: "Clemens' DotA-like layout for AZERTY keyboards",
        content: || include_str!("../../templates/CustomKeys_Clemens_DotA_AZERTY.txt"),
    },
    BundledTemplate {
        name: "NEO (QWERTY)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTY keyboards",
        content: || include_str!("../../templates/CustomKeys_Neo_QWERTY.txt"),
    },
    BundledTemplate {
        name: "NEO (QWERTZ)",
        description: "Neo's (Back2Warcraft) optimized layout adapted for QWERTZ keyboards",
        content: || include_str!("../../templates/CustomKeys_Neo_QWERTZ.txt"),
    },
    BundledTemplate {
        name: "NEO (AZERTY)",
        description: "Neo's (Back2Warcraft) optimized layout converted for AZERTY keyboards",
        content: || include_str!("../../templates/CustomKeys_Neo_AZERTY.txt"),
    },
];

/// A preset resolved into paint-ready previews of a representative unit's command
/// card and research menu, plus the layout the preset derives.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedTemplate {
    template: &'static BundledTemplate,
    grid: GridLayout,
    command_tiles: Vec<RenderedTile>,
    research_tiles: Vec<RenderedTile>,
}

impl ResolvedTemplate {
    pub fn name(&self) -> &'static str {
        self.template.name()
    }

    pub fn description(&self) -> &'static str {
        self.template.description()
    }

    /// The preset's raw CustomKeys text, applied verbatim when the user picks it.
    pub fn content(&self) -> &'static str {
        self.template.content()
    }

    pub fn grid(&self) -> &GridLayout {
        &self.grid
    }

    pub fn command_tiles(&self) -> &[RenderedTile] {
        &self.command_tiles
    }

    pub fn research_tiles(&self) -> &[RenderedTile] {
        &self.research_tiles
    }

    /// Every bundled preset, resolved once and cached.
    pub fn resolve_all() -> Vec<Self> {
        static CACHE: OnceLock<Vec<ResolvedTemplate>> = OnceLock::new();
        CACHE.get_or_init(Self::compute_all).clone()
    }

    fn compute_all() -> Vec<Self> {
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
        let no_tiers = HashMap::new();
        let no_restrict: [GridSlotId; 0] = [];
        TEMPLATES
            .iter()
            .map(|template| {
                let parsed_file = CustomKeys::parse_raw(template.content());
                let derived_grid = GridLayout::derived_from(&parsed_file);
                let mut preview_file = CustomKeys::parse_raw(DEFAULT_CUSTOM_KEYS);
                preview_file.extend(parsed_file);
                let command_input = CommandGridRenderInput::new(
                    &command_slots,
                    derived_grid,
                    None,
                    false,
                    &no_tiers,
                    &no_restrict,
                );
                let command_behavior = CommandBehavior;
                let command_tiles =
                    preview_file.rendered_command_grid(&command_behavior, &command_input);
                let research_input = CommandGridRenderInput::new(
                    &research_slots,
                    derived_grid,
                    None,
                    false,
                    &no_tiers,
                    &no_restrict,
                );
                let research_behavior = ResearchBehavior;
                let research_tiles =
                    preview_file.rendered_command_grid(&research_behavior, &research_input);
                Self {
                    template,
                    grid: derived_grid,
                    command_tiles,
                    research_tiles,
                }
            })
            .collect()
    }
}

impl ddd::ReadModel for ResolvedTemplate {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_every_bundled_preset_with_a_populated_command_card() {
        let resolved = ResolvedTemplate::resolve_all();
        assert_eq!(resolved.len(), TEMPLATES.len());
        for template in &resolved {
            assert!(!template.command_tiles().is_empty());
            let has_occupant = template
                .command_tiles()
                .iter()
                .any(|tile| tile.occupant().is_some());
            assert!(has_occupant, "{} has no command occupants", template.name());
        }
    }

    #[test]
    fn the_first_preset_is_the_stock_default() {
        let resolved = ResolvedTemplate::resolve_all();
        let first = resolved.first().expect("at least one preset");
        assert_eq!(first.name(), "Default");
        assert_eq!(first.content(), DEFAULT_CUSTOM_KEYS);
    }
}
