use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::stat_icon::StatIcon;
use dioxus::prelude::*;
use warcraft_api::AttackType;
use warcraft_keybinds::{AttackRange, AttackSpeed, AttackStatistics, DamagePerSecond, DamageRange};

/// The combat column's figures, shaped out of the body from the unit's attack: the
/// icon source and alt text, the five stat values (three always shown, the guarded
/// range and damage-per-second), and the attack type that drives the matchup grid.
pub(super) struct CombatFigures {
    pub(super) icon_src: Asset,
    pub(super) icon_alt: String,
    pub(super) damage: DamageRange,
    pub(super) range: AttackRange,
    pub(super) speed: AttackSpeed,
    pub(super) damage_per_second: Option<DamagePerSecond>,
    pub(super) attack_type: AttackType,
}

impl From<&AttackStatistics> for CombatFigures {
    fn from(attack: &AttackStatistics) -> Self {
        let attack_type = attack.attack_type();
        let icon = StatIcon::from(attack_type);
        let icon_src = icon.asset();
        let type_label = attack_type.to_string();
        let icon_alt = format!("{type_label} attack icon");
        let damage = attack.damage();
        let range = attack.range();
        let speed = attack.speed();
        let damage_per_second = attack.damage_per_second();
        Self {
            icon_src,
            icon_alt,
            damage,
            range,
            speed,
            damage_per_second,
            attack_type,
        }
    }
}
