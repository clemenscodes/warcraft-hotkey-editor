use super::super::shared::stat_icon_frame::StatIconFrameProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_keybinds::HeroStatistics;

/// The hero attributes column's input: the hero's three attributes at the selected
/// level, or `None` for a non-hero unit (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct AttributesColumnProps {
    pub hero: Option<HeroStatistics>,
}

impl From<&HeroStatistics> for StatIconFrameProps {
    fn from(hero: &HeroStatistics) -> Self {
        let primary = hero.primary();
        let icon = StatIcon::from(primary);
        let src = icon.asset();
        let primary_label = primary.to_string();
        let alt = format!("{primary_label} primary attribute icon");
        Self { src, alt }
    }
}
