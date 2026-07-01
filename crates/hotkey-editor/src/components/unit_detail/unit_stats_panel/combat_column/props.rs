use super::super::stat_icon_frame::StatIconFrameProps;
use dioxus::prelude::*;
use warcraft_api::AttackType;

/// The resolved attack figures the combat column renders.
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
}

#[derive(Props, Clone, PartialEq)]
pub struct CombatColumnProps {
    pub attack: AttackDisplayData,
}

impl From<&AttackDisplayData> for StatIconFrameProps {
    fn from(attack: &AttackDisplayData) -> Self {
        let src = attack.type_icon;
        let alt = format!("{} attack icon", attack.type_label);
        Self { src, alt }
    }
}
