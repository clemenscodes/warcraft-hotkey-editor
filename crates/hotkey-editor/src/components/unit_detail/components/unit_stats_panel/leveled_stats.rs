use num_traits::cast::cast;
use warcraft_api::{HeroAttributes, PrimaryAttribute, UnitAttack, UnitCombat};
use warcraft_database::WARCRAFT_GAMEPLAY_CONSTANTS;

pub(crate) struct LeveledStats {
    strength: u32,
    agility: u32,
    intelligence: u32,
    hit_points: u32,
    hit_points_regen: f32,
    mana: u32,
    mana_regen: f32,
    armor: f32,
    damage_min: u32,
    damage_max: u32,
}

impl LeveledStats {
    pub(crate) fn for_hero(combat: &UnitCombat, attributes: &HeroAttributes, level: u32) -> Self {
        let levels_added = level.saturating_sub(1);
        let levels_added_float: f32 = cast(levels_added).unwrap_or(0.0);
        let base_strength_float: f32 = cast(attributes.strength()).unwrap_or(0.0);
        let base_agility_float: f32 = cast(attributes.agility()).unwrap_or(0.0);
        let base_intelligence_float: f32 = cast(attributes.intelligence()).unwrap_or(0.0);
        let strength_total_float =
            base_strength_float + attributes.strength_per_level() * levels_added_float;
        let agility_total_float =
            base_agility_float + attributes.agility_per_level() * levels_added_float;
        let intelligence_total_float =
            base_intelligence_float + attributes.intelligence_per_level() * levels_added_float;
        let strength: u32 =
            cast::<f32, u32>(strength_total_float.floor()).unwrap_or(attributes.strength());
        let agility: u32 =
            cast::<f32, u32>(agility_total_float.floor()).unwrap_or(attributes.agility());
        let intelligence: u32 =
            cast::<f32, u32>(intelligence_total_float.floor()).unwrap_or(attributes.intelligence());
        let strength_delta = strength.saturating_sub(attributes.strength());
        let intelligence_delta = intelligence.saturating_sub(attributes.intelligence());
        let agility_delta_float: f32 =
            cast(agility.saturating_sub(attributes.agility())).unwrap_or(0.0);
        let strength_float: f32 = cast(strength).unwrap_or(0.0);
        let intelligence_float: f32 = cast(intelligence).unwrap_or(0.0);
        let hit_points = combat.hit_points()
            + strength_delta * WARCRAFT_GAMEPLAY_CONSTANTS.str_hit_point_bonus();
        let hit_points_regen = combat.hit_points_regen()
            + strength_float * WARCRAFT_GAMEPLAY_CONSTANTS.str_regen_bonus();
        let mana =
            attributes.mana() + intelligence_delta * WARCRAFT_GAMEPLAY_CONSTANTS.int_mana_bonus();
        let mana_regen = attributes.mana_regen()
            + intelligence_float * WARCRAFT_GAMEPLAY_CONSTANTS.int_regen_bonus();
        let armor =
            combat.armor() + agility_delta_float * WARCRAFT_GAMEPLAY_CONSTANTS.agi_defense_bonus();
        let primary_now = match attributes.primary() {
            PrimaryAttribute::Strength => strength,
            PrimaryAttribute::Agility => agility,
            PrimaryAttribute::Intelligence => intelligence,
        };
        let primary_base = match attributes.primary() {
            PrimaryAttribute::Strength => attributes.strength(),
            PrimaryAttribute::Agility => attributes.agility(),
            PrimaryAttribute::Intelligence => attributes.intelligence(),
        };
        let primary_delta = primary_now.saturating_sub(primary_base);
        let primary_delta_float: f32 = cast(primary_delta).unwrap_or(0.0);
        let attack_bonus_float: f32 =
            primary_delta_float * WARCRAFT_GAMEPLAY_CONSTANTS.str_attack_bonus();
        let primary_delta_attack: u32 =
            cast::<f32, u32>(attack_bonus_float.floor()).unwrap_or(primary_delta);
        let attack_min_base = combat.attack().map(UnitAttack::damage_min).unwrap_or(0);
        let attack_max_base = combat.attack().map(UnitAttack::damage_max).unwrap_or(0);
        Self {
            strength,
            agility,
            intelligence,
            hit_points,
            hit_points_regen,
            mana,
            mana_regen,
            armor,
            damage_min: attack_min_base + primary_delta_attack,
            damage_max: attack_max_base + primary_delta_attack,
        }
    }

    pub(crate) fn strength(&self) -> u32 {
        self.strength
    }

    pub(crate) fn agility(&self) -> u32 {
        self.agility
    }

    pub(crate) fn intelligence(&self) -> u32 {
        self.intelligence
    }

    pub(crate) fn hit_points(&self) -> u32 {
        self.hit_points
    }

    pub(crate) fn hit_points_regen(&self) -> f32 {
        self.hit_points_regen
    }

    pub(crate) fn mana(&self) -> u32 {
        self.mana
    }

    pub(crate) fn mana_regen(&self) -> f32 {
        self.mana_regen
    }

    pub(crate) fn armor(&self) -> f32 {
        self.armor
    }

    pub(crate) fn damage_min(&self) -> u32 {
        self.damage_min
    }

    pub(crate) fn damage_max(&self) -> u32 {
        self.damage_max
    }
}
