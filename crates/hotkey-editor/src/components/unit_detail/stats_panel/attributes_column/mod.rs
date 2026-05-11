use dioxus::prelude::*;

use super::attribute_row::AttributeRow;

#[derive(Clone, PartialEq)]
pub(super) struct HeroDisplayData {
    pub(super) primary_icon: Asset,
    pub(super) primary_label: String,
    pub(super) strength_value: u32,
    pub(super) strength_per_level: f32,
    pub(super) agility_value: u32,
    pub(super) agility_per_level: f32,
    pub(super) intelligence_value: u32,
    pub(super) intelligence_per_level: f32,
    pub(super) primary_is_strength: bool,
    pub(super) primary_is_agility: bool,
    pub(super) primary_is_intelligence: bool,
}

#[derive(Props, Clone, PartialEq)]
pub(super) struct AttributesColumnProps {
    pub(super) hero: HeroDisplayData,
}

#[component]
pub(super) fn AttributesColumn(props: AttributesColumnProps) -> Element {
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
