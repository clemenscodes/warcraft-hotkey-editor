use super::components::attributes_column::AttributesColumnProps;
use super::components::combat_column::CombatColumnProps;
use super::components::shared::stat_icon_frame::StatIconFrameProps;
use super::components::shared::stat_row::StatRowProps;
use super::kinds::{
    ArmorKind, DefenseTypeKind, EffectiveHitPointsKind, HitPointsKind, HitPointsRegenKind,
    ManaKind, ManaRegenKind,
};
use super::props::UnitStatsPanelProps;
use super::stat_icon::StatIcon;
use warcraft_api::DefenseType;
use warcraft_keybinds::{Evasion, UnitStatistics};

/// Every child's finished props for the stats panel: the vitality and defense rows the
/// panel renders inline, the defense icon and matchup inputs, and the two guarded
/// columns. The panel body destructures this and only places it.
pub(super) struct UnitStatsPanelModel {
    pub(super) hit_points_row: StatRowProps<HitPointsKind>,
    pub(super) hit_points_regen_row: StatRowProps<HitPointsRegenKind>,
    pub(super) mana_row: StatRowProps<ManaKind>,
    pub(super) mana_regen_row: StatRowProps<ManaRegenKind>,
    pub(super) armor_row: StatRowProps<ArmorKind>,
    pub(super) defense_type_row: StatRowProps<DefenseTypeKind>,
    pub(super) effective_hit_points_row: StatRowProps<EffectiveHitPointsKind>,
    pub(super) evasion: Evasion,
    pub(super) defense_type: DefenseType,
    pub(super) defense_icon: StatIconFrameProps,
    pub(super) combat: CombatColumnProps,
    pub(super) attributes: AttributesColumnProps,
}

/// Resolves every stat figure through the domain's [`UnitStatistics::compute`], then
/// shapes each child's props. All the arithmetic lives in the domain; this only reads
/// the resolved figures and wraps them into row props.
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
    let hit_points_row = StatRowProps::<HitPointsKind> { value: hit_points };
    let hit_points_regen_row = StatRowProps::<HitPointsRegenKind> {
        value: hit_points_regen,
    };
    let mana_row = StatRowProps::<ManaKind> { value: mana };
    let mana_regen_row = StatRowProps::<ManaRegenKind> { value: mana_regen };
    let armor_row = StatRowProps::<ArmorKind> { value: armor };
    let defense_type_row = StatRowProps::<DefenseTypeKind> {
        value: defense_type,
    };
    let effective_hit_points_row = StatRowProps::<EffectiveHitPointsKind> {
        value: effective_hit_points,
    };
    let defense_icon_kind = StatIcon::from(defense_type);
    let defense_icon_source = defense_icon_kind.asset();
    let defense_label = defense_type.to_string();
    let defense_icon_alt = format!("{defense_label} defense icon");
    let defense_icon = StatIconFrameProps {
        src: defense_icon_source,
        alt: defense_icon_alt,
    };
    let combat = CombatColumnProps { attack };
    let attributes = AttributesColumnProps { hero };
    UnitStatsPanelModel {
        hit_points_row,
        hit_points_regen_row,
        mana_row,
        mana_regen_row,
        armor_row,
        defense_type_row,
        effective_hit_points_row,
        evasion: resolved_evasion,
        defense_type,
        defense_icon,
        combat,
        attributes,
    }
}
