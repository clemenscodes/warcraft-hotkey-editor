use super::components::shared::stat_row::{StatRowProps, StatRowVariant};
use super::hooks::UnitStatsPanelModel;

/// The vitality column's rows, built from the resolved figures so the column body
/// is a pure loop. The hit-points and its regen row appear only when the unit has
/// hit points; the regen row shows its qualifier and gain, or a muted zero.
pub(super) fn vitality_rows(model: &UnitStatsPanelModel) -> Vec<StatRowProps> {
    let mut rows: Vec<StatRowProps> = Vec::new();
    if model.has_hp {
        let hit_points = StatRowProps {
            variant: StatRowVariant::Hp,
            is_regen: false,
            is_primary: false,
            label: "Hit Points".to_string(),
            qualifier: None,
            value: Some(model.display_hp_text.clone()),
            value_is_zero: false,
            gain: None,
            gain_is_zero: false,
        };
        rows.push(hit_points);
        let hit_points_regen = if model.has_regen {
            StatRowProps {
                variant: StatRowVariant::Default,
                is_regen: true,
                is_primary: false,
                label: "Regeneration".to_string(),
                qualifier: model.regen_qualifier,
                value: None,
                value_is_zero: false,
                gain: Some(model.regen_text.clone()),
                gain_is_zero: false,
            }
        } else {
            StatRowProps {
                variant: StatRowVariant::Default,
                is_regen: true,
                is_primary: false,
                label: "Regeneration".to_string(),
                qualifier: None,
                value: None,
                value_is_zero: false,
                gain: Some("+0.00".to_string()),
                gain_is_zero: true,
            }
        };
        rows.push(hit_points_regen);
    }
    let mana = StatRowProps {
        variant: StatRowVariant::Mana,
        is_regen: false,
        is_primary: false,
        label: "Mana".to_string(),
        qualifier: None,
        value: Some(model.mana_display_text.clone()),
        value_is_zero: !model.has_mana,
        gain: None,
        gain_is_zero: false,
    };
    rows.push(mana);
    let mana_regen = StatRowProps {
        variant: StatRowVariant::Mana,
        is_regen: true,
        is_primary: false,
        label: "Regeneration".to_string(),
        qualifier: None,
        value: None,
        value_is_zero: false,
        gain: Some(model.mana_regen_text.clone()),
        gain_is_zero: !model.has_mana_regen,
    };
    rows.push(mana_regen);
    rows
}

/// The defense column's label/value rows (the matchup row is rendered separately).
/// The evasion row appears only when the unit can evade; otherwise, when it also
/// has no attack, a blank row keeps the four columns aligned.
pub(super) fn defense_rows(model: &UnitStatsPanelModel) -> Vec<StatRowProps> {
    let mut rows: Vec<StatRowProps> = Vec::new();
    let armor = StatRowProps {
        variant: StatRowVariant::Default,
        is_regen: false,
        is_primary: false,
        label: "Armor".to_string(),
        qualifier: None,
        value: Some(model.armor_text.clone()),
        value_is_zero: false,
        gain: None,
        gain_is_zero: false,
    };
    rows.push(armor);
    let defense_type = StatRowProps {
        variant: StatRowVariant::Default,
        is_regen: false,
        is_primary: false,
        label: "Defense Type".to_string(),
        qualifier: None,
        value: Some(model.defense_label.clone()),
        value_is_zero: false,
        gain: None,
        gain_is_zero: false,
    };
    rows.push(defense_type);
    let effective_hit_points = StatRowProps {
        variant: StatRowVariant::Default,
        is_regen: false,
        is_primary: false,
        label: "Effective Hit Points".to_string(),
        qualifier: None,
        value: Some(model.effective_hit_points_text.clone()),
        value_is_zero: false,
        gain: None,
        gain_is_zero: false,
    };
    rows.push(effective_hit_points);
    if model.has_evasion {
        let evasion = StatRowProps {
            variant: StatRowVariant::Default,
            is_regen: false,
            is_primary: false,
            label: "Evasion".to_string(),
            qualifier: None,
            value: Some(model.evasion_text.clone()),
            value_is_zero: false,
            gain: None,
            gain_is_zero: false,
        };
        rows.push(evasion);
    } else if !model.has_attack {
        let spacer = StatRowProps {
            variant: StatRowVariant::Default,
            is_regen: false,
            is_primary: false,
            label: "\u{00a0}".to_string(),
            qualifier: None,
            value: None,
            value_is_zero: false,
            gain: None,
            gain_is_zero: false,
        };
        rows.push(spacer);
    }
    rows
}
