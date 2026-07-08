use super::components::attributes_column::AttributesColumnProps;
use super::components::combat_column::CombatColumnProps;
use super::components::defense_column::DefenseColumnProps;
use super::components::vitality_column::VitalityColumnProps;
use super::props::UnitStatsPanelProps;
use warcraft_keybinds::UnitStatistics;

/// Every child column's finished props for the stats panel. The panel body
/// destructures this and only places the four columns.
pub(super) struct UnitStatsPanelModel {
    pub(super) vitality: VitalityColumnProps,
    pub(super) combat: CombatColumnProps,
    pub(super) defense: DefenseColumnProps,
    pub(super) attributes: AttributesColumnProps,
}

/// Resolves every stat figure through the domain's [`UnitStatistics::compute`], then
/// shapes each column's props. All the arithmetic lives in the domain; this only reads
/// the resolved figures and wraps them into column props.
pub(super) fn use_unit_stats_panel(props: &UnitStatsPanelProps) -> UnitStatsPanelModel {
    let unit_combat = props.combat;
    let hero_attributes = props.hero_attributes;
    let selected_hero_level = props.selected_hero_level;
    let evasion = props.evasion;
    let current_level = if hero_attributes.is_some() {
        selected_hero_level()
    } else {
        1
    };
    let evasion_chance = evasion.chance();
    let hero_attributes_reference = hero_attributes.as_ref();
    let statistics = UnitStatistics::compute(
        &unit_combat,
        hero_attributes_reference,
        current_level,
        evasion_chance,
    );
    let hit_points = statistics.hit_points();
    let hit_points_regen = statistics.hit_points_regen();
    let mana = statistics.mana();
    let mana_regen = statistics.mana_regen();
    let armor = statistics.armor();
    let defense_type = statistics.defense_type();
    let effective_hit_points = statistics.effective_hit_points();
    let resolved_evasion = statistics.evasion();
    let attack = statistics.attack();
    let hero = statistics.hero();
    let vitality = VitalityColumnProps {
        hit_points,
        hit_points_regen,
        mana,
        mana_regen,
    };
    let combat = CombatColumnProps { attack };
    let defense = DefenseColumnProps {
        armor,
        defense_type,
        effective_hit_points,
        evasion: resolved_evasion,
    };
    let attributes = AttributesColumnProps { hero };
    UnitStatsPanelModel {
        vitality,
        combat,
        defense,
        attributes,
    }
}
