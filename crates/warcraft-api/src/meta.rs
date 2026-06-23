use crate::object::{GridCoordinate, ItemClass, UnitKind, WarcraftObjectId};

#[derive(Default, Debug, Clone, Copy)]
pub struct CommandMeta {
    default_button_position: Option<GridCoordinate>,
    tip: Option<&'static str>,
    ubertip: Option<&'static str>,
}

impl CommandMeta {
    pub const fn new(default_button_position: Option<GridCoordinate>) -> Self {
        Self {
            default_button_position,
            tip: None,
            ubertip: None,
        }
    }

    pub const fn with_text(
        default_button_position: Option<GridCoordinate>,
        tip: Option<&'static str>,
        ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            default_button_position,
            tip,
            ubertip,
        }
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn tip(&self) -> Option<&'static str> {
        self.tip
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitProduction {
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
}

impl UnitProduction {
    pub const EMPTY: UnitProduction = UnitProduction {
        researches: &[],
        builds: &[],
        trains: &[],
        sell_items: &[],
        sell_units: &[],
    };

    pub const fn new(
        researches: &'static [WarcraftObjectId],
        builds: &'static [WarcraftObjectId],
        trains: &'static [WarcraftObjectId],
        sell_items: &'static [WarcraftObjectId],
        sell_units: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            researches,
            builds,
            trains,
            sell_items,
            sell_units,
        }
    }

    pub fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnitFlags {
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
}

impl UnitFlags {
    pub const EDITOR_ONLY: UnitFlags = UnitFlags {
        is_campaign: false,
        is_in_editor: true,
        is_hidden_in_editor: false,
        is_special: false,
    };

    pub const fn new(
        is_campaign: bool,
        is_in_editor: bool,
        is_hidden_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            is_campaign,
            is_in_editor,
            is_hidden_in_editor,
            is_special,
        }
    }

    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryAttribute {
    #[default]
    Strength,
    Agility,
    Intelligence,
}

impl PrimaryAttribute {
    pub fn parse(raw: &str) -> Option<PrimaryAttribute> {
        let normalized = raw.trim().to_ascii_uppercase();
        match normalized.as_str() {
            "STR" => Some(PrimaryAttribute::Strength),
            "AGI" => Some(PrimaryAttribute::Agility),
            "INT" => Some(PrimaryAttribute::Intelligence),
            _ => None,
        }
    }
}

impl std::fmt::Display for PrimaryAttribute {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            PrimaryAttribute::Strength => "Strength",
            PrimaryAttribute::Agility => "Agility",
            PrimaryAttribute::Intelligence => "Intelligence",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    Normal,
    Pierce,
    Siege,
    Magic,
    Chaos,
    Hero,
    Spells,
    #[default]
    Unknown,
}

impl AttackType {
    pub fn parse(raw: &str) -> AttackType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "normal" => AttackType::Normal,
            "pierce" => AttackType::Pierce,
            "siege" => AttackType::Siege,
            "magic" => AttackType::Magic,
            "chaos" => AttackType::Chaos,
            "hero" => AttackType::Hero,
            "spells" => AttackType::Spells,
            _ => AttackType::Unknown,
        }
    }
}

impl std::fmt::Display for AttackType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            AttackType::Normal => "Normal",
            AttackType::Pierce => "Piercing",
            AttackType::Siege => "Siege",
            AttackType::Magic => "Magic",
            AttackType::Chaos => "Chaos",
            AttackType::Hero => "Hero",
            AttackType::Spells => "Spells",
            AttackType::Unknown => "Unknown",
        };
        formatter.write_str(label)
    }
}

// Mirrors the `weapTp1` column in `unitweapons.slk`. This is distinct from
// `AttackType` (the damage type): the weapon type controls attack behavior, and
// the two artillery variants are the ones that grant a unit the Attack Ground
// command in-game.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeaponType {
    Normal,
    Instant,
    Artillery,
    ArtilleryLine,
    Missile,
    MissileSplash,
    MissileBounce,
    MissileLine,
    None,
    #[default]
    Unknown,
}

impl WeaponType {
    pub fn parse(raw: &str) -> WeaponType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "normal" => WeaponType::Normal,
            "instant" => WeaponType::Instant,
            "artillery" => WeaponType::Artillery,
            "aline" => WeaponType::ArtilleryLine,
            "missile" => WeaponType::Missile,
            "msplash" => WeaponType::MissileSplash,
            "mbounce" => WeaponType::MissileBounce,
            "mline" => WeaponType::MissileLine,
            "none" => WeaponType::None,
            _ => WeaponType::Unknown,
        }
    }

    pub fn targets_ground(&self) -> bool {
        matches!(self, WeaponType::Artillery | WeaponType::ArtilleryLine)
    }
}

impl std::fmt::Display for WeaponType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            WeaponType::Normal => "Normal",
            WeaponType::Instant => "Instant",
            WeaponType::Artillery => "Artillery (Targets Ground)",
            WeaponType::ArtilleryLine => "Artillery (Line)",
            WeaponType::Missile => "Missile",
            WeaponType::MissileSplash => "Missile (Splash)",
            WeaponType::MissileBounce => "Missile (Bounce)",
            WeaponType::MissileLine => "Missile (Line)",
            WeaponType::None => "None",
            WeaponType::Unknown => "Unknown",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefenseType {
    Light,
    Medium,
    Heavy,
    Fortified,
    Normal,
    Hero,
    Divine,
    #[default]
    Unarmored,
}

impl DefenseType {
    pub fn parse(raw: &str) -> DefenseType {
        let normalized = raw.trim().to_ascii_lowercase();
        match normalized.as_str() {
            "small" | "light" => DefenseType::Light,
            "medium" => DefenseType::Medium,
            "large" | "heavy" => DefenseType::Heavy,
            "fort" | "fortified" => DefenseType::Fortified,
            "normal" => DefenseType::Normal,
            "hero" => DefenseType::Hero,
            "divine" => DefenseType::Divine,
            _ => DefenseType::Unarmored,
        }
    }

    pub const fn all() -> [DefenseType; 8] {
        [
            DefenseType::Light,
            DefenseType::Medium,
            DefenseType::Heavy,
            DefenseType::Fortified,
            DefenseType::Normal,
            DefenseType::Hero,
            DefenseType::Divine,
            DefenseType::Unarmored,
        ]
    }
}

impl std::fmt::Display for DefenseType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let label = match self {
            DefenseType::Light => "Light",
            DefenseType::Medium => "Medium",
            DefenseType::Heavy => "Heavy",
            DefenseType::Fortified => "Fortified",
            DefenseType::Normal => "Normal",
            DefenseType::Hero => "Hero",
            DefenseType::Divine => "Divine",
            DefenseType::Unarmored => "Unarmored",
        };
        formatter.write_str(label)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitAttack {
    damage_min: u32,
    damage_max: u32,
    range: u32,
    cooldown_seconds: f32,
    attack_type: AttackType,
    weapon_type: WeaponType,
}

impl UnitAttack {
    pub const fn new(
        damage_min: u32,
        damage_max: u32,
        range: u32,
        cooldown_seconds: f32,
        attack_type: AttackType,
        weapon_type: WeaponType,
    ) -> Self {
        Self {
            damage_min,
            damage_max,
            range,
            cooldown_seconds,
            attack_type,
            weapon_type,
        }
    }

    pub fn damage_min(&self) -> u32 {
        self.damage_min
    }

    pub fn damage_max(&self) -> u32 {
        self.damage_max
    }

    pub fn range(&self) -> u32 {
        self.range
    }

    pub fn cooldown_seconds(&self) -> f32 {
        self.cooldown_seconds
    }

    pub fn attack_type(&self) -> AttackType {
        self.attack_type
    }

    pub fn weapon_type(&self) -> WeaponType {
        self.weapon_type
    }

    pub fn targets_ground(&self) -> bool {
        self.weapon_type.targets_ground()
    }
}

// Mirrors the `regenType` column in `unitbalance.slk`. Controls when HP
// regeneration is active; the rate (`hit_points_regen`) is the per-second
// value WHILE active, with no day/night multiplier on top of it.
//
//   Always — regenerates anywhere, anytime (Human, Orc, neutral creeps).
//   Night  — regenerates only between dusk and dawn (Night Elf).
//   Blight — regenerates only while standing on blight (Undead).
//   None   — does not regenerate HP at all (some neutral structures /
//            mechanical creeps).
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegenType {
    #[default]
    Always,
    Night,
    Blight,
    None,
}

impl RegenType {
    pub fn parse(raw_value: &str) -> Self {
        match raw_value.trim().to_ascii_lowercase().as_str() {
            "always" => Self::Always,
            "night" => Self::Night,
            "blight" => Self::Blight,
            "none" | "" | "-" | "_" => Self::None,
            _ => Self::Always,
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct UnitCombat {
    hit_points: u32,
    hit_points_regen: f32,
    regen_type: RegenType,
    armor: f32,
    defense_type: DefenseType,
    attack: Option<UnitAttack>,
    mana_pool: Option<ManaPool>,
}

impl UnitCombat {
    pub const EMPTY: UnitCombat = UnitCombat {
        hit_points: 0,
        hit_points_regen: 0.0,
        regen_type: RegenType::Always,
        armor: 0.0,
        defense_type: DefenseType::Unarmored,
        attack: None,
        mana_pool: None,
    };

    pub const fn new(
        hit_points: u32,
        hit_points_regen: f32,
        regen_type: RegenType,
        armor: f32,
        defense_type: DefenseType,
        attack: Option<UnitAttack>,
    ) -> Self {
        Self {
            hit_points,
            hit_points_regen,
            regen_type,
            armor,
            defense_type,
            attack,
            mana_pool: None,
        }
    }

    pub const fn with_mana_pool(mut self, mana_pool: ManaPool) -> Self {
        self.mana_pool = Some(mana_pool);
        self
    }

    pub fn hit_points(&self) -> u32 {
        self.hit_points
    }

    pub fn hit_points_regen(&self) -> f32 {
        self.hit_points_regen
    }

    pub fn regen_type(&self) -> RegenType {
        self.regen_type
    }

    pub fn armor(&self) -> f32 {
        self.armor
    }

    pub fn defense_type(&self) -> DefenseType {
        self.defense_type
    }

    pub fn attack(&self) -> Option<&UnitAttack> {
        self.attack.as_ref()
    }

    pub fn mana_pool(&self) -> Option<ManaPool> {
        self.mana_pool
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ManaPool {
    mana: u32,
    mana_regen: f32,
}

impl ManaPool {
    pub const fn new(mana: u32, mana_regen: f32) -> Self {
        Self { mana, mana_regen }
    }

    pub fn mana(&self) -> u32 {
        self.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_regen
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeBase {
    strength: u32,
    agility: u32,
    intelligence: u32,
}

impl AttributeBase {
    pub const fn new(strength: u32, agility: u32, intelligence: u32) -> Self {
        Self {
            strength,
            agility,
            intelligence,
        }
    }

    pub fn strength(&self) -> u32 {
        self.strength
    }

    pub fn agility(&self) -> u32 {
        self.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.intelligence
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AttributeGrowth {
    strength_per_level: f32,
    agility_per_level: f32,
    intelligence_per_level: f32,
}

impl AttributeGrowth {
    pub const fn new(
        strength_per_level: f32,
        agility_per_level: f32,
        intelligence_per_level: f32,
    ) -> Self {
        Self {
            strength_per_level,
            agility_per_level,
            intelligence_per_level,
        }
    }

    pub fn strength_per_level(&self) -> f32 {
        self.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.intelligence_per_level
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HeroAttributes {
    mana_pool: ManaPool,
    base: AttributeBase,
    growth: AttributeGrowth,
    primary: PrimaryAttribute,
}

impl HeroAttributes {
    pub const fn new(
        mana_pool: ManaPool,
        base: AttributeBase,
        growth: AttributeGrowth,
        primary: PrimaryAttribute,
    ) -> Self {
        Self {
            mana_pool,
            base,
            growth,
            primary,
        }
    }

    pub fn mana_pool(&self) -> ManaPool {
        self.mana_pool
    }

    pub fn base(&self) -> AttributeBase {
        self.base
    }

    pub fn growth(&self) -> AttributeGrowth {
        self.growth
    }

    pub fn mana(&self) -> u32 {
        self.mana_pool.mana
    }

    pub fn mana_regen(&self) -> f32 {
        self.mana_pool.mana_regen
    }

    pub fn strength(&self) -> u32 {
        self.base.strength
    }

    pub fn agility(&self) -> u32 {
        self.base.agility
    }

    pub fn intelligence(&self) -> u32 {
        self.base.intelligence
    }

    pub fn primary(&self) -> PrimaryAttribute {
        self.primary
    }

    pub fn strength_per_level(&self) -> f32 {
        self.growth.strength_per_level
    }

    pub fn agility_per_level(&self) -> f32 {
        self.growth.agility_per_level
    }

    pub fn intelligence_per_level(&self) -> f32 {
        self.growth.intelligence_per_level
    }
}

#[derive(Default, Debug, Clone)]
pub struct UnitMeta {
    unit_kind: UnitKind,
    build_time: u32,
    /// In-game tech tier (`level` column of `unitbalance.slk`); lower = available
    /// earlier. Used to order the unit list by in-game availability.
    level: u32,
    /// Gold cost (`goldcost` column of `unitbalance.slk`). Used to order
    /// buildings by tier (the chain base's cost groups upgrade chains).
    gold_cost: u32,
    abilities: &'static [WarcraftObjectId],
    hero_abilities: &'static [WarcraftObjectId],
    researches: &'static [WarcraftObjectId],
    builds: &'static [WarcraftObjectId],
    trains: &'static [WarcraftObjectId],
    sell_items: &'static [WarcraftObjectId],
    sell_units: &'static [WarcraftObjectId],
    is_campaign: bool,
    is_in_editor: bool,
    is_hidden_in_editor: bool,
    is_special: bool,
    combat: UnitCombat,
    hero_attributes: Option<HeroAttributes>,
}

impl UnitMeta {
    pub const fn new(unit_kind: UnitKind, build_time: u32) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities: &[],
            hero_abilities: &[],
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_abilities(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign: false,
            is_in_editor: true,
            is_hidden_in_editor: false,
            is_special: false,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        is_campaign: bool,
        is_in_editor: bool,
        is_special: bool,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: &[],
            builds: &[],
            trains: &[],
            sell_items: &[],
            sell_units: &[],
            is_campaign,
            is_in_editor,
            is_hidden_in_editor: false,
            is_special,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_full_and_extras(
        unit_kind: UnitKind,
        build_time: u32,
        abilities: &'static [WarcraftObjectId],
        hero_abilities: &'static [WarcraftObjectId],
        production: UnitProduction,
        flags: UnitFlags,
    ) -> Self {
        Self {
            unit_kind,
            build_time,
            level: 0,
            gold_cost: 0,
            abilities,
            hero_abilities,
            researches: production.researches,
            builds: production.builds,
            trains: production.trains,
            sell_items: production.sell_items,
            sell_units: production.sell_units,
            is_campaign: flags.is_campaign,
            is_in_editor: flags.is_in_editor,
            is_hidden_in_editor: flags.is_hidden_in_editor,
            is_special: flags.is_special,
            combat: UnitCombat::EMPTY,
            hero_attributes: None,
        }
    }

    pub const fn with_combat(mut self, combat: UnitCombat) -> Self {
        self.combat = combat;
        self
    }

    pub const fn with_hero_attributes(mut self, hero_attributes: HeroAttributes) -> Self {
        self.hero_attributes = Some(hero_attributes);
        self
    }

    pub const fn with_level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub const fn with_gold_cost(mut self, gold_cost: u32) -> Self {
        self.gold_cost = gold_cost;
        self
    }

    pub fn unit_kind(&self) -> UnitKind {
        self.unit_kind
    }

    pub fn build_time(&self) -> u32 {
        self.build_time
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn gold_cost(&self) -> u32 {
        self.gold_cost
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn hero_abilities(&self) -> &'static [WarcraftObjectId] {
        self.hero_abilities
    }

    pub fn builds(&self) -> &'static [WarcraftObjectId] {
        self.builds
    }

    pub fn trains(&self) -> &'static [WarcraftObjectId] {
        self.trains
    }

    pub fn is_campaign(&self) -> bool {
        self.is_campaign
    }

    pub fn is_in_editor(&self) -> bool {
        self.is_in_editor
    }

    pub fn is_hidden_in_editor(&self) -> bool {
        self.is_hidden_in_editor
    }

    pub fn is_special(&self) -> bool {
        self.is_special
    }

    pub fn researches(&self) -> &'static [WarcraftObjectId] {
        self.researches
    }

    pub fn sell_items(&self) -> &'static [WarcraftObjectId] {
        self.sell_items
    }

    pub fn sell_units(&self) -> &'static [WarcraftObjectId] {
        self.sell_units
    }

    pub fn is_melee_visible(&self) -> bool {
        self.is_in_editor && !self.is_campaign
    }

    pub fn combat(&self) -> &UnitCombat {
        &self.combat
    }

    pub fn hero_attributes(&self) -> Option<&HeroAttributes> {
        self.hero_attributes.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageEffectiveness {
    // Eight multipliers, one per defense type, in the order returned by
    // `DefenseType::all()`: Light, Medium, Heavy, Fortified, Normal, Hero,
    // Divine, Unarmored. Sourced from `DamageBonus*` lines in
    // `war3.w3mod:units/miscgame.txt`.
    multipliers: [f32; 8],
}

impl DamageEffectiveness {
    pub const fn new(multipliers: [f32; 8]) -> Self {
        Self { multipliers }
    }

    pub fn against(&self, defense_type: DefenseType) -> f32 {
        let defense_types = DefenseType::all();
        let mut iterator_index = 0;
        while iterator_index < defense_types.len() {
            if defense_types[iterator_index] == defense_type {
                return self.multipliers[iterator_index];
            }
            iterator_index += 1;
        }
        1.0
    }

    pub fn multipliers(&self) -> &[f32; 8] {
        &self.multipliers
    }
}

impl Default for DamageEffectiveness {
    fn default() -> Self {
        Self::new([1.0; 8])
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrengthBonuses {
    attack_bonus: f32,
    hit_point_bonus: u32,
    regen_bonus: f32,
}

impl StrengthBonuses {
    pub const fn new(attack_bonus: f32, hit_point_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            attack_bonus,
            hit_point_bonus,
            regen_bonus,
        }
    }

    pub fn attack_bonus(&self) -> f32 {
        self.attack_bonus
    }

    pub fn hit_point_bonus(&self) -> u32 {
        self.hit_point_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntelligenceBonuses {
    mana_bonus: u32,
    regen_bonus: f32,
}

impl IntelligenceBonuses {
    pub const fn new(mana_bonus: u32, regen_bonus: f32) -> Self {
        Self {
            mana_bonus,
            regen_bonus,
        }
    }

    pub fn mana_bonus(&self) -> u32 {
        self.mana_bonus
    }

    pub fn regen_bonus(&self) -> f32 {
        self.regen_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AgilityBonuses {
    defense_bonus: f32,
    attack_speed_bonus: f32,
}

impl AgilityBonuses {
    pub const fn new(defense_bonus: f32, attack_speed_bonus: f32) -> Self {
        Self {
            defense_bonus,
            attack_speed_bonus,
        }
    }

    pub fn defense_bonus(&self) -> f32 {
        self.defense_bonus
    }

    pub fn attack_speed_bonus(&self) -> f32 {
        self.attack_speed_bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageMatrix {
    normal: DamageEffectiveness,
    pierce: DamageEffectiveness,
    siege: DamageEffectiveness,
    magic: DamageEffectiveness,
    chaos: DamageEffectiveness,
    spells: DamageEffectiveness,
    hero: DamageEffectiveness,
}

impl DamageMatrix {
    pub const fn new(
        normal: DamageEffectiveness,
        pierce: DamageEffectiveness,
        siege: DamageEffectiveness,
        magic: DamageEffectiveness,
        chaos: DamageEffectiveness,
        spells: DamageEffectiveness,
        hero: DamageEffectiveness,
    ) -> Self {
        Self {
            normal,
            pierce,
            siege,
            magic,
            chaos,
            spells,
            hero,
        }
    }

    pub fn effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        match attack_type {
            AttackType::Normal => self.normal,
            AttackType::Pierce => self.pierce,
            AttackType::Siege => self.siege,
            AttackType::Magic => self.magic,
            AttackType::Chaos => self.chaos,
            AttackType::Spells => self.spells,
            AttackType::Hero => self.hero,
            AttackType::Unknown => DamageEffectiveness::new([1.0; 8]),
        }
    }

    pub fn normal(&self) -> DamageEffectiveness {
        self.normal
    }

    pub fn pierce(&self) -> DamageEffectiveness {
        self.pierce
    }

    pub fn siege(&self) -> DamageEffectiveness {
        self.siege
    }

    pub fn magic(&self) -> DamageEffectiveness {
        self.magic
    }

    pub fn chaos(&self) -> DamageEffectiveness {
        self.chaos
    }

    pub fn spells(&self) -> DamageEffectiveness {
        self.spells
    }

    pub fn hero(&self) -> DamageEffectiveness {
        self.hero
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GameplayConstants {
    // Defaults below mirror the standard WC3 Reforged values from
    // war3.w3mod:units/miscgame.txt; used only when extraction is missing
    // the field, so the runtime never sees an all-zero `GameplayConstants`.
    strength_bonuses: StrengthBonuses,
    intelligence_bonuses: IntelligenceBonuses,
    agility_bonuses: AgilityBonuses,
    max_hero_level: u32,
    damage_matrix: DamageMatrix,
}

impl GameplayConstants {
    pub const fn new(
        strength_bonuses: StrengthBonuses,
        intelligence_bonuses: IntelligenceBonuses,
        agility_bonuses: AgilityBonuses,
        max_hero_level: u32,
        damage_matrix: DamageMatrix,
    ) -> Self {
        Self {
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            max_hero_level,
            damage_matrix,
        }
    }

    pub fn strength_bonuses(&self) -> StrengthBonuses {
        self.strength_bonuses
    }

    pub fn intelligence_bonuses(&self) -> IntelligenceBonuses {
        self.intelligence_bonuses
    }

    pub fn agility_bonuses(&self) -> AgilityBonuses {
        self.agility_bonuses
    }

    pub fn damage_matrix(&self) -> DamageMatrix {
        self.damage_matrix
    }

    pub fn damage_effectiveness(&self, attack_type: AttackType) -> DamageEffectiveness {
        self.damage_matrix.effectiveness(attack_type)
    }

    pub fn str_attack_bonus(&self) -> f32 {
        self.strength_bonuses.attack_bonus
    }

    pub fn str_hit_point_bonus(&self) -> u32 {
        self.strength_bonuses.hit_point_bonus
    }

    pub fn str_regen_bonus(&self) -> f32 {
        self.strength_bonuses.regen_bonus
    }

    pub fn int_mana_bonus(&self) -> u32 {
        self.intelligence_bonuses.mana_bonus
    }

    pub fn int_regen_bonus(&self) -> f32 {
        self.intelligence_bonuses.regen_bonus
    }

    pub fn agi_defense_bonus(&self) -> f32 {
        self.agility_bonuses.defense_bonus
    }

    pub fn agi_attack_speed_bonus(&self) -> f32 {
        self.agility_bonuses.attack_speed_bonus
    }

    pub fn max_hero_level(&self) -> u32 {
        self.max_hero_level
    }
}

impl Default for GameplayConstants {
    fn default() -> Self {
        // SMALL, MEDIUM, LARGE, FORT, NORMAL, HERO, DIVINE, NONE — matches
        // miscgame.txt DamageBonus* line order.
        let damage_normal =
            DamageEffectiveness::new([1.00, 1.50, 1.00, 0.70, 1.00, 1.00, 0.05, 1.00]);
        let damage_pierce =
            DamageEffectiveness::new([2.00, 0.75, 1.00, 0.35, 1.00, 0.50, 0.05, 1.50]);
        let damage_siege =
            DamageEffectiveness::new([1.00, 0.50, 1.00, 1.50, 1.00, 0.50, 0.05, 1.50]);
        let damage_magic =
            DamageEffectiveness::new([1.25, 0.75, 2.00, 0.35, 1.00, 0.50, 0.05, 1.00]);
        let damage_chaos =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00, 1.00]);
        let damage_spells =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 1.00, 1.00, 0.70, 0.05, 1.00]);
        let damage_hero =
            DamageEffectiveness::new([1.00, 1.00, 1.00, 0.50, 1.00, 1.00, 0.05, 1.00]);
        let damage_matrix = DamageMatrix::new(
            damage_normal,
            damage_pierce,
            damage_siege,
            damage_magic,
            damage_chaos,
            damage_spells,
            damage_hero,
        );
        let strength_bonuses = StrengthBonuses::new(1.0, 25, 0.05);
        let intelligence_bonuses = IntelligenceBonuses::new(15, 0.05);
        let agility_bonuses = AgilityBonuses::new(0.30, 0.02);
        Self::new(
            strength_bonuses,
            intelligence_bonuses,
            agility_bonuses,
            10,
            damage_matrix,
        )
    }
}

#[derive(Default, Debug, Clone)]
pub struct AbilityMeta {
    max_level: usize,
    is_ultimate: bool,
    cooldowns: [u32; 4],
    /// Per-level chance to evade an attack (0.0..=1.0), one slot per ability
    /// level. Non-zero only for evasion abilities (Evasion `AEev`, Drunken
    /// Brawler `ANdb`); every other ability leaves this `[0.0; 4]`. Sourced
    /// from the real `abilitydata.slk` data field, not the tooltip text.
    evasion_chances: [f32; 4],
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
    ubertip: Option<&'static str>,
    research_ubertip: Option<&'static str>,
    code: Option<&'static str>,
    morph_target_unit: Option<WarcraftObjectId>,
    off_button_position: Option<GridCoordinate>,
    off_tip: Option<&'static str>,
    off_ubertip: Option<&'static str>,
    off_icon: Option<&'static str>,
}

impl AbilityMeta {
    pub const fn new(max_level: usize, is_ultimate: bool, cooldowns: [u32; 4]) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [0.0; 4],
            default_button_position: None,
            default_research_button_position: None,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_defaults(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<GridCoordinate>,
        default_research_button_position: Option<GridCoordinate>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [0.0; 4],
            default_button_position,
            default_research_button_position,
            ubertip: None,
            research_ubertip: None,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_ubertips(
        max_level: usize,
        is_ultimate: bool,
        cooldowns: [u32; 4],
        default_button_position: Option<GridCoordinate>,
        default_research_button_position: Option<GridCoordinate>,
        ubertip: Option<&'static str>,
        research_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            max_level,
            is_ultimate,
            cooldowns,
            evasion_chances: [0.0; 4],
            default_button_position,
            default_research_button_position,
            ubertip,
            research_ubertip,
            code: None,
            morph_target_unit: None,
            off_button_position: None,
            off_tip: None,
            off_ubertip: None,
            off_icon: None,
        }
    }

    pub const fn with_code(mut self, code: Option<&'static str>) -> Self {
        self.code = code;
        self
    }

    pub const fn with_morph_target(mut self, target: Option<WarcraftObjectId>) -> Self {
        self.morph_target_unit = target;
        self
    }

    pub const fn with_evasion_chances(mut self, evasion_chances: [f32; 4]) -> Self {
        self.evasion_chances = evasion_chances;
        self
    }

    pub const fn with_off_state(
        mut self,
        off_button_position: Option<GridCoordinate>,
        off_tip: Option<&'static str>,
        off_ubertip: Option<&'static str>,
        off_icon: Option<&'static str>,
    ) -> Self {
        self.off_button_position = off_button_position;
        self.off_tip = off_tip;
        self.off_ubertip = off_ubertip;
        self.off_icon = off_icon;
        self
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        self.ubertip
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        self.research_ubertip
    }

    /// Game-mechanic class as listed in `units/abilitydata.slk`'s `code`
    /// column. Independent of the per-unit alias — e.g. multiple aliases
    /// can resolve to `code = "Apit"` (Purchase Item / shop button).
    pub fn code(&self) -> Option<&'static str> {
        self.code
    }

    /// For one-way morph abilities (Avenger Form, Crow Form, etc.) the
    /// unit id this ability transforms its caster into. Sourced from the
    /// `UnitID1` column of `abilitydata.slk`.
    pub fn morph_target_unit(&self) -> Option<&WarcraftObjectId> {
        self.morph_target_unit.as_ref()
    }

    /// Off-state button position for toggleable abilities (e.g. Defend on
    /// the Footman). Some abilities place their "deactivate" cell at a
    /// different grid slot when active. Sourced from `UnButtonpos=` in
    /// `abilityfunc.txt`.
    pub fn off_button_position(&self) -> Option<GridCoordinate> {
        self.off_button_position
    }

    /// Off-state short tooltip — the label shown while the ability is
    /// active (e.g. "Stop Defending" while Defend is on). Sourced from
    /// `UnTip=` in `abilityfunc.txt`.
    pub fn off_tip(&self) -> Option<&'static str> {
        self.off_tip
    }

    /// Off-state long description — `UnUbertip=` in `abilityfunc.txt`.
    pub fn off_ubertip(&self) -> Option<&'static str> {
        self.off_ubertip
    }

    /// Off-state icon path (`UnArt=` in `abilityfunc.txt`). Different art
    /// from the on-state icon for toggle abilities like Defend, whose
    /// active state shows a distinct "Stop Defending" art.
    pub fn off_icon(&self) -> Option<&'static str> {
        self.off_icon
    }

    /// Returns true if the ability has any off-state data in the database.
    /// One-shot abilities (e.g. Healing Wave) have all four off-state fields
    /// set to None and must not receive a materialized unbutton_position.
    pub fn has_off_state(&self) -> bool {
        let position_set = self.off_button_position.is_some();
        let tip_set = self.off_tip.is_some();
        let ubertip_set = self.off_ubertip.is_some();
        let icon_set = self.off_icon.is_some();
        position_set || tip_set || ubertip_set || icon_set
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        self.default_button_position
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        self.default_research_button_position
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }

    pub fn is_ultimate(&self) -> bool {
        self.is_ultimate
    }

    pub fn cooldown_for_level(&self, level: usize) -> Option<u32> {
        if level == 0 || level > self.max_level {
            None
        } else {
            Some(self.cooldowns[level - 1])
        }
    }

    pub fn base_cooldown(&self) -> u32 {
        self.cooldowns[0]
    }

    pub fn cooldowns(&self) -> [u32; 4] {
        self.cooldowns
    }

    /// Per-level chance to evade an attack (0.0..=1.0). `[0.0; 4]` for any
    /// ability that is not an evasion ability.
    pub fn evasion_chances(&self) -> [f32; 4] {
        self.evasion_chances
    }

    /// Chance to evade at a given ability level (1-based), or `None` when the
    /// level is out of range. Levels beyond `max_level` are not real.
    pub fn evasion_chance_for_level(&self, level: usize) -> Option<f32> {
        if level == 0 || level > self.max_level {
            None
        } else {
            Some(self.evasion_chances[level - 1])
        }
    }

    /// True if the ability grants any evasion at any level.
    pub fn has_evasion(&self) -> bool {
        self.evasion_chances.iter().any(|chance| *chance > 0.0)
    }
}

#[derive(Default, Debug, Clone)]
pub struct UpgradeMeta {
    max_level: usize,
}

impl UpgradeMeta {
    pub fn new(max_level: usize) -> Self {
        Self { max_level }
    }

    pub fn max_level(&self) -> usize {
        self.max_level
    }
}

#[derive(Default, Debug, Clone)]
pub struct ItemMeta {
    class: ItemClass,
    abilities: &'static [WarcraftObjectId],
    cooldown_id: Option<WarcraftObjectId>,
}

impl ItemMeta {
    pub fn new(
        class: ItemClass,
        abilities: &'static [WarcraftObjectId],
        cooldown_id: Option<WarcraftObjectId>,
    ) -> Self {
        Self {
            class,
            abilities,
            cooldown_id,
        }
    }

    pub fn cooldown_id(&self) -> Option<WarcraftObjectId> {
        self.cooldown_id
    }

    pub fn abilities(&self) -> &'static [WarcraftObjectId] {
        self.abilities
    }

    pub fn class(&self) -> &ItemClass {
        &self.class
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::object::{ColumnIndex, GridCoordinate, RowIndex, WarcraftObjectId};

    // WeaponType

    #[test]
    fn weapon_type_parse_artillery_variants() {
        assert_eq!(WeaponType::parse("artillery"), WeaponType::Artillery);
        assert_eq!(WeaponType::parse("aline"), WeaponType::ArtilleryLine);
        assert_eq!(WeaponType::parse("MISSILE"), WeaponType::Missile);
        assert_eq!(WeaponType::parse("nonsense"), WeaponType::Unknown);
    }

    #[test]
    fn weapon_type_targets_ground_only_for_artillery() {
        assert!(WeaponType::Artillery.targets_ground());
        assert!(WeaponType::ArtilleryLine.targets_ground());
        assert!(!WeaponType::Missile.targets_ground());
        assert!(!WeaponType::Normal.targets_ground());
    }

    #[test]
    fn unit_attack_targets_ground_reflects_weapon_type() {
        let artillery_attack =
            UnitAttack::new(50, 60, 1000, 2.0, AttackType::Siege, WeaponType::Artillery);
        let normal_attack =
            UnitAttack::new(10, 12, 90, 1.5, AttackType::Normal, WeaponType::Normal);
        assert!(artillery_attack.targets_ground());
        assert!(!normal_attack.targets_ground());
    }

    // DefenseType

    #[test]
    fn defense_type_parse_aliases() {
        assert_eq!(DefenseType::parse("small"), DefenseType::Light);
        assert_eq!(DefenseType::parse("large"), DefenseType::Heavy);
        assert_eq!(DefenseType::parse("fort"), DefenseType::Fortified);
        assert_eq!(DefenseType::parse("divine"), DefenseType::Divine);
    }

    #[test]
    fn defense_type_all_has_eight_entries() {
        assert_eq!(DefenseType::all().len(), 8);
    }

    #[test]
    fn defense_type_all_contains_every_variant() {
        let all = DefenseType::all();
        assert!(all.contains(&DefenseType::Light));
        assert!(all.contains(&DefenseType::Unarmored));
        assert!(all.contains(&DefenseType::Divine));
    }

    // AttackType

    #[test]
    fn attack_type_parse_known_values() {
        assert_eq!(AttackType::parse("normal"), AttackType::Normal);
        assert_eq!(AttackType::parse("chaos"), AttackType::Chaos);
        assert_eq!(AttackType::parse("spells"), AttackType::Spells);
    }

    #[test]
    fn attack_type_parse_unknown_falls_back_to_unknown() {
        assert_eq!(AttackType::parse("garbage"), AttackType::Unknown);
    }

    // PrimaryAttribute

    #[test]
    fn primary_attribute_parse_case_insensitive() {
        assert_eq!(
            PrimaryAttribute::parse("str"),
            Some(PrimaryAttribute::Strength)
        );
        assert_eq!(
            PrimaryAttribute::parse("AGI"),
            Some(PrimaryAttribute::Agility)
        );
        assert_eq!(
            PrimaryAttribute::parse("int"),
            Some(PrimaryAttribute::Intelligence)
        );
    }

    #[test]
    fn primary_attribute_parse_unknown_is_none() {
        assert_eq!(PrimaryAttribute::parse("xyz"), None);
    }

    #[test]
    fn primary_attribute_display_is_full_name() {
        assert_eq!(PrimaryAttribute::Strength.to_string(), "Strength");
        assert_eq!(PrimaryAttribute::Agility.to_string(), "Agility");
        assert_eq!(PrimaryAttribute::Intelligence.to_string(), "Intelligence");
    }

    // RegenType

    #[test]
    fn regen_type_parse_known_values() {
        assert_eq!(RegenType::parse("always"), RegenType::Always);
        assert_eq!(RegenType::parse("night"), RegenType::Night);
        assert_eq!(RegenType::parse("blight"), RegenType::Blight);
        assert_eq!(RegenType::parse("none"), RegenType::None);
    }

    #[test]
    fn regen_type_parse_empty_and_dash_are_none() {
        assert_eq!(RegenType::parse(""), RegenType::None);
        assert_eq!(RegenType::parse("-"), RegenType::None);
        assert_eq!(RegenType::parse("_"), RegenType::None);
    }

    // DamageEffectiveness

    #[test]
    fn damage_effectiveness_against_returns_correct_multiplier() {
        let all_twos = DamageEffectiveness::new([2.0; 8]);
        assert_eq!(all_twos.against(DefenseType::Light), 2.0);
        assert_eq!(all_twos.against(DefenseType::Divine), 2.0);
    }

    #[test]
    fn damage_effectiveness_default_is_all_ones() {
        let effectiveness = DamageEffectiveness::default();
        for &multiplier in effectiveness.multipliers() {
            assert_eq!(multiplier, 1.0);
        }
    }

    // GameplayConstants

    #[test]
    fn gameplay_constants_default_has_reasonable_max_hero_level() {
        let constants = GameplayConstants::default();
        assert_eq!(constants.max_hero_level(), 10);
    }

    #[test]
    fn gameplay_constants_default_str_per_hp_is_nonzero() {
        let constants = GameplayConstants::default();
        assert!(constants.str_hit_point_bonus() > 0);
    }

    #[test]
    fn damage_matrix_chaos_is_effective_against_all_armor() {
        let constants = GameplayConstants::default();
        let chaos = constants.damage_effectiveness(AttackType::Chaos);
        for &multiplier in chaos.multipliers() {
            assert_eq!(multiplier, 1.0);
        }
    }

    // AbilityMeta

    #[test]
    fn ability_meta_cooldown_for_level_returns_none_for_zero() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(0), None);
    }

    #[test]
    fn ability_meta_cooldown_for_level_returns_none_beyond_max() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(4), None);
    }

    #[test]
    fn ability_meta_cooldown_for_valid_levels() {
        let meta = AbilityMeta::new(3, false, [10, 8, 6, 0]);
        assert_eq!(meta.cooldown_for_level(1), Some(10));
        assert_eq!(meta.cooldown_for_level(2), Some(8));
        assert_eq!(meta.cooldown_for_level(3), Some(6));
    }

    #[test]
    fn ability_meta_base_cooldown_is_level_one() {
        let meta = AbilityMeta::new(3, false, [15, 10, 5, 0]);
        assert_eq!(meta.base_cooldown(), 15);
    }

    #[test]
    fn ability_meta_with_morph_target_stores_id() {
        let target = WarcraftObjectId::new("Hamg");
        let meta = AbilityMeta::new(1, false, [0; 4]).with_morph_target(Some(target));
        assert_eq!(meta.morph_target_unit().map(|id| id.value()), Some("Hamg"));
    }

    #[test]
    fn ability_meta_with_off_state_stores_all_fields() {
        let position = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        let meta = AbilityMeta::new(1, false, [0; 4]).with_off_state(
            Some(position),
            Some("Stop Defending"),
            Some("Deactivates defend"),
            Some("passivebuttons/btndefend.blp"),
        );
        let expected_coordinate = GridCoordinate::new(ColumnIndex::Three, RowIndex::Two);
        assert_eq!(meta.off_button_position(), Some(expected_coordinate));
        assert_eq!(meta.off_tip(), Some("Stop Defending"));
        assert_eq!(meta.off_ubertip(), Some("Deactivates defend"));
        assert_eq!(meta.off_icon(), Some("passivebuttons/btndefend.blp"));
    }

    // UnitMeta

    #[test]
    fn unit_meta_new_defaults_to_in_editor_not_campaign() {
        let meta = UnitMeta::new(UnitKind::Soldier, 60);
        assert!(meta.is_in_editor());
        assert!(!meta.is_campaign());
    }

    #[test]
    fn unit_meta_is_melee_visible_only_when_in_editor_and_not_campaign() {
        let visible = UnitMeta::new(UnitKind::Soldier, 60);
        assert!(visible.is_melee_visible());
        let campaign = UnitMeta::with_full(UnitKind::Hero, 0, &[], &[], true, true, false);
        assert!(!campaign.is_melee_visible());
    }

    // UpgradeMeta

    #[test]
    fn upgrade_meta_stores_max_level() {
        let meta = UpgradeMeta::new(3);
        assert_eq!(meta.max_level(), 3);
    }
}
