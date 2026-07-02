use super::data;
use super::props::AttackDisplayData;

/// One label/value line in the combat column.
pub(super) struct CombatLine {
    pub(super) label: &'static str,
    pub(super) value: String,
}

/// The combat column's rows in order, including the range and damage-per-second
/// rows only when they apply. The conditionals live here, in the data, so the
/// column body is a pure loop.
pub(super) fn combat_lines(attack: &AttackDisplayData) -> Vec<CombatLine> {
    let mut lines: Vec<CombatLine> = Vec::new();
    let damage_value = attack.damage_text().to_owned();
    let damage = CombatLine {
        label: data::DAMAGE,
        value: damage_value,
    };
    lines.push(damage);
    let attack_range = attack.attack_range();
    if attack_range > 0 {
        let range_value = attack_range.to_string();
        let range = CombatLine {
            label: data::RANGE,
            value: range_value,
        };
        lines.push(range);
    }
    let speed_value = attack.speed_text().to_owned();
    let speed = CombatLine {
        label: data::ATTACK_SPEED,
        value: speed_value,
    };
    lines.push(speed);
    if let Some(dps_text) = attack.damage_per_second_text() {
        let dps_value = dps_text.to_owned();
        let dps = CombatLine {
            label: data::DAMAGE_PER_SECOND,
            value: dps_value,
        };
        lines.push(dps);
    }
    let type_value = attack.type_label().to_owned();
    let type_line = CombatLine {
        label: data::ATTACK_TYPE,
        value: type_value,
    };
    lines.push(type_line);
    lines
}
