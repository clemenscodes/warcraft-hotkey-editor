use super::props::AttributesColumnProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_api::PrimaryAttribute;
use warcraft_keybinds::{AttributeStatistic, HeroStatistics};

/// The attributes column's figures, shaped from the hero: the primary-attribute icon and
/// each of the three attributes with whether it is the hero's primary.
pub(super) struct AttributeFigures {
    pub(super) icon_src: Asset,
    pub(super) icon_alt: String,
    pub(super) strength: AttributeStatistic,
    pub(super) strength_is_primary: bool,
    pub(super) agility: AttributeStatistic,
    pub(super) agility_is_primary: bool,
    pub(super) intelligence: AttributeStatistic,
    pub(super) intelligence_is_primary: bool,
}

impl From<&HeroStatistics> for AttributeFigures {
    fn from(hero: &HeroStatistics) -> Self {
        let primary = hero.primary();
        let icon = StatIcon::from(primary);
        let icon_src = icon.asset();
        let primary_label = primary.to_string();
        let icon_alt = format!("{primary_label} primary attribute icon");
        let strength = hero.strength();
        let agility = hero.agility();
        let intelligence = hero.intelligence();
        let strength_is_primary = primary == PrimaryAttribute::Strength;
        let agility_is_primary = primary == PrimaryAttribute::Agility;
        let intelligence_is_primary = primary == PrimaryAttribute::Intelligence;
        Self {
            icon_src,
            icon_alt,
            strength,
            strength_is_primary,
            agility,
            agility_is_primary,
            intelligence,
            intelligence_is_primary,
        }
    }
}
