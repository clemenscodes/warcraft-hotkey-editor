use super::damage_matchup_row::DamageMatchupRow;
use dioxus::prelude::*;
use warcraft_api::AttackType;

#[derive(Clone, PartialEq)]
pub struct AttackDisplayData {
    damage_text: String,
    attack_range: u32,
    speed_text: String,
    damage_per_second_text: Option<String>,
    attack_type: AttackType,
    type_label: String,
    type_icon: Asset,
}

impl AttackDisplayData {
    pub fn new(
        damage_text: String,
        attack_range: u32,
        speed_text: String,
        damage_per_second_text: Option<String>,
        attack_type: AttackType,
        type_label: String,
        type_icon: Asset,
    ) -> Self {
        Self {
            damage_text,
            attack_range,
            speed_text,
            damage_per_second_text,
            attack_type,
            type_label,
            type_icon,
        }
    }

    pub fn damage_per_second_text(&self) -> Option<&str> {
        self.damage_per_second_text.as_deref()
    }

    pub fn damage_text(&self) -> &str {
        &self.damage_text
    }

    pub fn attack_range(&self) -> u32 {
        self.attack_range
    }

    pub fn speed_text(&self) -> &str {
        &self.speed_text
    }

    pub fn attack_type(&self) -> AttackType {
        self.attack_type
    }

    pub fn type_label(&self) -> &str {
        &self.type_label
    }

    pub fn type_icon(&self) -> Asset {
        self.type_icon
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnProps {
    pub attack: AttackDisplayData,
}

#[component]
pub fn CombatColumn(props: CombatColumnProps) -> Element {
    let attack = props.attack;
    let damage_text = attack.damage_text().to_owned();
    let attack_range = attack.attack_range();
    let speed_text = attack.speed_text().to_owned();
    let damage_per_second_text = attack.damage_per_second_text().map(str::to_owned);
    let attack_type = attack.attack_type();
    let type_label = attack.type_label().to_owned();
    let attack_range_text = attack_range.to_string();
    let type_icon = attack.type_icon();
    let type_icon_alt = format!("{type_label} attack icon");
    rsx! {
        div { class: "stat-column combat-column with-icon",
            div { class: "stat-icon-frame",
                img { class: "stat-icon", src: type_icon, alt: type_icon_alt }
            }
            div { class: "stat-rows",
                div { class: "stat-row",
                    span { class: "stat-row-label", "Damage" }
                    span { class: "stat-row-value", {damage_text} }
                }
                if attack_range > 0 {
                    div { class: "stat-row",
                        span { class: "stat-row-label", "Range" }
                        span { class: "stat-row-value", { attack_range_text
                                } }
                    }
                }
                div { class: "stat-row",
                    span { class: "stat-row-label", "Attack Speed" }
                    span { class: "stat-row-value", {speed_text} }
                }
                if let Some(dps_text) = damage_per_second_text {
                    div { class: "stat-row",
                        span { class: "stat-row-label", "Damage per Second" }
                        span { class: "stat-row-value", {dps_text} }
                    }
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
