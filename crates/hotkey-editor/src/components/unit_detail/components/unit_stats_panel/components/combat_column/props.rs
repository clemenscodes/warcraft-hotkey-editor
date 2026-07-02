use super::super::shared::stat_icon_frame::StatIconFrameProps;
use crate::components::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_keybinds::AttackStatistics;

/// The combat column's input: the unit's attack profile, or `None` when it cannot
/// attack (the column then renders nothing).
#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnProps {
    pub attack: Option<AttackStatistics>,
}

impl From<&AttackStatistics> for StatIconFrameProps {
    fn from(attack: &AttackStatistics) -> Self {
        let attack_type = attack.attack_type();
        let icon = StatIcon::from(attack_type);
        let src = icon.asset();
        let type_label = attack_type.to_string();
        let alt = format!("{type_label} attack icon");
        Self { src, alt }
    }
}
