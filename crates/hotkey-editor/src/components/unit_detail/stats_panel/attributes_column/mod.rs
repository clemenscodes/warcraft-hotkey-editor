use dioxus::prelude::*;

use super::attribute_row::AttributeRow;

#[derive(Clone, PartialEq)]
pub struct HeroDisplayData {
    primary_icon: Asset,
    primary_label: String,
    strength_value: u32,
    strength_per_level: f32,
    agility_value: u32,
    agility_per_level: f32,
    intelligence_value: u32,
    intelligence_per_level: f32,
    primary_is_strength: bool,
    primary_is_agility: bool,
    primary_is_intelligence: bool,
}

impl HeroDisplayData {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        primary_icon: Asset,
        primary_label: String,
        strength_value: u32,
        strength_per_level: f32,
        agility_value: u32,
        agility_per_level: f32,
        intelligence_value: u32,
        intelligence_per_level: f32,
        primary_is_strength: bool,
        primary_is_agility: bool,
        primary_is_intelligence: bool,
    ) -> Self {
        Self {
            primary_icon,
            primary_label,
            strength_value,
            strength_per_level,
            agility_value,
            agility_per_level,
            intelligence_value,
            intelligence_per_level,
            primary_is_strength,
            primary_is_agility,
            primary_is_intelligence,
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AttributesColumnProps {
    pub hero: HeroDisplayData,
}

#[component]
pub fn AttributesColumn(props: AttributesColumnProps) -> Element {
    let hero = props.hero;
    rsx! {
        div { class: "stat-column attributes-column with-icon",
            div { class: "stat-icon-frame",
                img {
                    class: "stat-icon",
                    src: hero.primary_icon,
                    alt: "{hero.primary_label} primary attribute icon",
                }
            }
            div { class: "stat-rows",
                AttributeRow {
                    label: "Strength",
                    value: hero.strength_value,
                    per_level: hero.strength_per_level,
                    is_primary: hero.primary_is_strength,
                }
                AttributeRow {
                    label: "Agility",
                    value: hero.agility_value,
                    per_level: hero.agility_per_level,
                    is_primary: hero.primary_is_agility,
                }
                AttributeRow {
                    label: "Intelligence",
                    value: hero.intelligence_value,
                    per_level: hero.intelligence_per_level,
                    is_primary: hero.primary_is_intelligence,
                }
            }
        }
    }
}
