use super::model::UnitStatsPanelModel;
use crate::services::editor_state::context::use_editor_state;
use warcraft_api::DefenseType;
use warcraft_api::{
    ArmorFigure as Armor, AttackStatistics, EffectiveHitPoints, Evasion, HeroStatistics, HitPoints,
    HitPointsRegen, Mana, ManaRegen, UnitStatistics,
};

/// Every resolved stat figure the four columns render. The panel body destructures this
/// and places the four columns, handing each its figures as named fields.
pub(super) struct UnitStatsPanelPresentation {
    pub(super) hit_points: HitPoints,
    pub(super) hit_points_regen: HitPointsRegen,
    pub(super) mana: Mana,
    pub(super) mana_regen: ManaRegen,
    pub(super) attack: Option<AttackStatistics>,
    pub(super) armor: Armor,
    pub(super) defense_type: DefenseType,
    pub(super) effective_hit_points: EffectiveHitPoints,
    pub(super) evasion: Evasion,
    pub(super) hero: Option<HeroStatistics>,
}

/// Resolves every stat figure through the domain's [`UnitStatistics::compute`], then
/// hands the panel the resolved figures. All the arithmetic lives in the domain; this
/// only reads the resolved figures.
pub(super) fn use_unit_stats_panel(props: &UnitStatsPanelModel) -> UnitStatsPanelPresentation {
    let unit_combat = props.combat;
    let hero_attributes = props.hero_attributes;
    let selected_hero_level = use_editor_state().selected_hero_level();
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
    UnitStatsPanelPresentation {
        hit_points,
        hit_points_regen,
        mana,
        mana_regen,
        attack,
        armor,
        defense_type,
        effective_hit_points,
        evasion: resolved_evasion,
        hero,
    }
}

impl ddd::Presentation for UnitStatsPanelPresentation {
    type Model = UnitStatsPanelModel;
}
