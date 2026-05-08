use dioxus::prelude::*;
use warcraft_api::{AttackType, DefenseType, PrimaryAttribute};

// `infocard-neutral-*` and `infocard-heroattributes-*` textures are the
// creep-panel / observer-panel variants — they have no opaque level box
// baked into the corner (verified via alpha-channel inspection: BR alpha
// ~0.38 vs ~0.91 for the upgrade-tier counterparts).
//
// Hero / Divine defense reuse the gold knight helm with blue eyes
// (`infocard-neutral-attack-hero`) — same imagery in-game for both armor
// types.
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
// Hero/Divine share the same gold knight helm imagery. `infocard-armor-hero.dds`
// bakes in a level-overlay box at bottom-right; CSS `mask-image` on `.stat-icon`
// hides that box at render time.
const ICON_ARMOR_HERO: Asset = asset!("/assets/webui/infocard/infocard-armor-hero.png");

#[derive(Clone, Copy)]
pub(crate) struct StatIcon {
    asset: Asset,
}

impl StatIcon {
    fn new(asset: Asset) -> Self {
        Self { asset }
    }

    pub(crate) fn asset(&self) -> Asset {
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
        // Hero/Divine share the same icon — neither has an `infocard-neutral-armor-*` variant in CASC.
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
