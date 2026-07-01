use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType, PrimaryAttribute};

const ICON_STRENGTH: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-str.png");
const ICON_AGILITY: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-agi.png");
const ICON_INTELLIGENCE: Asset = asset!("/assets/webui/infocard/infocard-heroattributes-int.png");
const ICON_ATTACK_MELEE: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-melee.png");

const ICON_ATTACK_PIERCING: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-attack-piercing.png");

const ICON_ATTACK_SIEGE: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-siege.png");
const ICON_ATTACK_MAGIC: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-magic.png");
const ICON_ATTACK_CHAOS: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-chaos.png");
const ICON_ATTACK_HERO: Asset = asset!("/assets/webui/infocard/infocard-neutral-attack-hero.png");
const ICON_ARMOR_SMALL: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-small.png");
const ICON_ARMOR_MEDIUM: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-medium.png");
const ICON_ARMOR_LARGE: Asset = asset!("/assets/webui/infocard/infocard-neutral-armor-large.png");

const ICON_ARMOR_FORTIFIED: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-armor-fortified.png");

const ICON_ARMOR_UNARMORED: Asset =
    asset!("/assets/webui/infocard/infocard-neutral-armor-unarmored.png");

const ICON_ARMOR_HERO: Asset = asset!("/assets/webui/infocard/infocard-armor-hero.png");

#[derive(Clone, Copy)]
pub struct StatIcon {
    asset: Asset,
}

impl StatIcon {
    fn new(asset: Asset) -> Self {
        Self { asset }
    }

    pub fn asset(&self) -> Asset {
        self.asset
    }
}

impl From<AttackType> for StatIcon {
    fn from(attack_type: AttackType) -> Self {
        let resolved = match attack_type {
            AttackType::Normal => ICON_ATTACK_MELEE,
            AttackType::Pierce => ICON_ATTACK_PIERCING,
            AttackType::Siege => ICON_ATTACK_SIEGE,
            AttackType::Magic | AttackType::Spells => ICON_ATTACK_MAGIC,
            AttackType::Chaos => ICON_ATTACK_CHAOS,
            AttackType::Hero => ICON_ATTACK_HERO,
            AttackType::Unknown => ICON_ATTACK_MELEE,
        };
        Self::new(resolved)
    }
}

impl From<DefenseType> for StatIcon {
    fn from(defense_type: DefenseType) -> Self {
        let resolved = match defense_type {
            DefenseType::Light => ICON_ARMOR_SMALL,
            DefenseType::Medium | DefenseType::Normal => ICON_ARMOR_MEDIUM,
            DefenseType::Heavy => ICON_ARMOR_LARGE,
            DefenseType::Fortified => ICON_ARMOR_FORTIFIED,
            DefenseType::Hero | DefenseType::Divine => ICON_ARMOR_HERO,
            DefenseType::Unarmored => ICON_ARMOR_UNARMORED,
        };
        Self::new(resolved)
    }
}

impl From<PrimaryAttribute> for StatIcon {
    fn from(primary: PrimaryAttribute) -> Self {
        let resolved = match primary {
            PrimaryAttribute::Strength => ICON_STRENGTH,
            PrimaryAttribute::Agility => ICON_AGILITY,
            PrimaryAttribute::Intelligence => ICON_INTELLIGENCE,
        };
        Self::new(resolved)
    }
}
