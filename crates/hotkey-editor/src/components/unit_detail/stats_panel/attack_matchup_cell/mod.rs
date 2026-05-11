use dioxus::prelude::*;
use num_traits::cast::cast;
use warcraft_api::{AttackType, DefenseType};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

#[derive(Props, Clone, PartialEq)]
pub(super) struct AttackMatchupCellProps {
    pub(super) defense_type: DefenseType,
    pub(super) attack_type: AttackType,
}

#[component]
pub(super) fn AttackMatchupCell(props: AttackMatchupCellProps) -> Element {
    let defense_type = props.defense_type;
    let attack_type = props.attack_type;
    let effectiveness = WARCRAFT_GAMEPLAY_CONSTANTS.damage_effectiveness(attack_type);
    let multiplier = effectiveness.against(defense_type);
    let percent_text = percent_label(multiplier);
    let cell_class = matchup_cell_class_attacking(multiplier);
    let defense_label = defense_type.to_string();
    let title_text = format!("vs {defense_label}");
    rsx! {
        div { class: cell_class, title: title_text,
            span { class: "matchup-label", {defense_label} }
            span { class: "matchup-value", {percent_text} }
        }
    }
}

fn matchup_cell_class_attacking(multiplier: f32) -> &'static str {
    if multiplier > 1.05 {
        "matchup-cell strong"
    } else if multiplier < 0.95 {
        "matchup-cell weak"
    } else {
        "matchup-cell"
    }
}

fn percent_label(multiplier: f32) -> String {
    let percent_int: i32 = cast::<f32, i32>((multiplier * 100.0).round()).unwrap_or(0);
    format!("{percent_int}%")
}
