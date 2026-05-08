use dioxus::prelude::*;
use warcraft_api::AttackType;

use super::damage_matchup_row::DamageMatchupRow;

#[derive(Clone, PartialEq)]
pub(super) struct AttackDisplayData {
    damage_min: u32,
    damage_max: u32,
    attack_range: u32,
    speed_text: String,
    attack_type: AttackType,
    type_label: String,
    type_icon: Asset,
}

impl AttackDisplayData {
    pub(super) fn new(
        damage_min: u32,
        damage_max: u32,
        attack_range: u32,
        speed_text: String,
        attack_type: AttackType,
        type_label: String,
        type_icon: Asset,
    ) -> Self {
        Self {
            damage_min,
            damage_max,
            attack_range,
            speed_text,
            attack_type,
            type_label,
            type_icon,
        }
    }

    pub(super) fn damage_min(&self) -> u32 {
        self.damage_min
    }

    pub(super) fn damage_max(&self) -> u32 {
        self.damage_max
    }

    pub(super) fn attack_range(&self) -> u32 {
        self.attack_range
    }

    pub(super) fn speed_text(&self) -> &str {
        &self.speed_text
    }

    pub(super) fn attack_type(&self) -> AttackType {
        self.attack_type
    }

    pub(super) fn type_label(&self) -> &str {
        &self.type_label
    }

    pub(super) fn type_icon(&self) -> Asset {
        self.type_icon
    }
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct CombatColumnProps {
    pub(super) attack: AttackDisplayData,
}

#[component]
pub(super) fn CombatColumn(props: CombatColumnProps) -> Element {
    let attack = props.attack;
    let damage_min = attack.damage_min();
    let damage_max = attack.damage_max();
    let attack_range = attack.attack_range();
    let speed_text = attack.speed_text().to_owned();
    let attack_type = attack.attack_type();
    let type_label = attack.type_label().to_owned();
    let attack_range_text = attack_range.to_string();
    let type_icon = attack.type_icon();
    let type_icon_alt = format!("{type_label} attack icon");
    rsx! {
        div { class: "stat-column combat-column with-icon",
            div { class: "stat-icon-frame",
                img {
                    class: "stat-icon",
                    src: type_icon,
                    alt: type_icon_alt,
                }
            }
            div { class: "stat-rows",
                div { class: "stat-row",
                    span { class: "stat-row-label", "Damage" }
                    span { class: "stat-row-value", "{damage_min}\u{2013}{damage_max}" }
                }
                if attack_range > 0 {
                    div { class: "stat-row",
                        span { class: "stat-row-label", "Range" }
                        span { class: "stat-row-value", {attack_range_text} }
                    }
                }
                div { class: "stat-row",
                    span { class: "stat-row-label", "Attack Speed" }
                    span { class: "stat-row-value", {speed_text} }
                }
                div { class: "stat-row",
                    span { class: "stat-row-label", "Attack Type" }
                    span { class: "stat-row-value", {type_label} }
                }
                DamageMatchupRow { attack_type }
            }
        }
    }
}
