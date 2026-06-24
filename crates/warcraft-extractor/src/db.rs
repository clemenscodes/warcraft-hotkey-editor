use warcraft_api::{
    AbilityMeta, AttackType, AttributeBase, AttributeGrowth, CommandMeta, DefenseType,
    GameplayConstants, GridCoordinate, HeroAttributes, ItemMeta, ManaPool, ObjectMap,
    PrimaryAttribute, Race, RegenType, UnitAttack, UnitCombat, UnitFlags, UnitKind, UnitMeta,
    UnitProduction, UpgradeMeta, WarcraftDatabase, WarcraftObject, WarcraftObjectId,
    WarcraftObjectKind, WarcraftObjectMeta, WarcraftObjectText, WeaponType,
};

struct WarcraftObjectIdentity {
    id: WarcraftObjectId,
    names: &'static [&'static str],
    icons: &'static [&'static str],
    kind: WarcraftObjectKind,
    race: Option<Race>,
}

impl WarcraftObjectIdentity {
    fn new(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
        }
    }
}

use crate::{
    AbilityDefaultsDatabase, AbilityMetadataDatabase, AbilitySkins, CampaignAbilityStringsDatabase,
    CampaignUnitStringsDatabase, CommandDefaultsDatabase, CommonAbilityStringsDatabase,
    CommonUnitStringsDatabase, DataTablesDatabase, DefaultPositionsDatabase, ExtractResult,
    HeroDatabase, HumanAbilityStringsDatabase, HumanUnitStringsDatabase, HumanUpgradeArtDatabase,
    HumanUpgradeNameDatabase, ItemAbilityStringsDatabase, ItemDatabase, ItemSkins,
    ItemUnitStringsDatabase, NeutralAbilityStringsDatabase, NeutralUnitStringsDatabase,
    NightelfAbilityStringsDatabase, NightelfUnitStringsDatabase, NightelfUpgradeArtDatabase,
    NightelfUpgradeNameDatabase, ObjectTextDatabase, OrcAbilityStringsDatabase,
    OrcUnitStringsDatabase, OrcUpgradeArtDatabase, OrcUpgradeNameDatabase, SystemKeybindsDatabase,
    UndeadAbilityStringsDatabase, UndeadUnitStringsDatabase, UndeadUpgradeArtDatabase,
    UndeadUpgradeNameDatabase, UnitAbilitiesDatabase, UnitDataDatabase, UnitDatabase, UnitSkins,
    UnitUiFlagsDatabase, UpgradeSwapDatabase, race_from_unit_id, upgrades::RaceUpgradeEntry,
};

impl From<WarcraftDataAggregation> for WarcraftDatabase {
    fn from(mut value: WarcraftDataAggregation) -> Self {
        value.split_toggle_passive_positions();
        let objects = value.get_ids();
        Self::new(objects)
    }
}

/// A source-form ability suppression that Rule 2 applies to a transformed unit:
/// every ability the base form (`base_unit_id`) carries is dropped from the
/// transformed unit, except the transform ability itself (`transform_ability_id`).
#[derive(Debug, Clone, PartialEq, Eq)]
struct TransformSuppression {
    transform_ability_id: String,
    base_unit_id: String,
}

impl TransformSuppression {
    fn transform_ability_id(&self) -> &str {
        &self.transform_ability_id
    }

    fn base_unit_id(&self) -> &str {
        &self.base_unit_id
    }
}

#[derive(Default, Debug, Clone)]
pub struct WarcraftDataAggregation {
    heroes: HeroDatabase,
    units: UnitDatabase,
    unit_abilities: UnitAbilitiesDatabase,
    ability_metadata: AbilityMetadataDatabase,
    upgrade_swaps: UpgradeSwapDatabase,
    unit_data: UnitDataDatabase,
    unit_ui_flags: UnitUiFlagsDatabase,
    command_defaults: CommandDefaultsDatabase,
    ability_defaults: AbilityDefaultsDatabase,
    data_tables: DataTablesDatabase,
    default_positions: DefaultPositionsDatabase,
    object_texts: ObjectTextDatabase,
    system_keybinds: SystemKeybindsDatabase,
    items: ItemDatabase,
    upgrades: UpgradeDatabase,
    skins: SkinDatabase,
    strings: StringsDatabase,
    gameplay_constants: GameplayConstants,
}

impl WarcraftDataAggregation {
    pub fn heroes(&self) -> &HeroDatabase {
        &self.heroes
    }

    pub fn units(&self) -> &UnitDatabase {
        &self.units
    }

    pub fn unit_abilities(&self) -> &UnitAbilitiesDatabase {
        &self.unit_abilities
    }

    pub fn ability_defaults(&self) -> &AbilityDefaultsDatabase {
        &self.ability_defaults
    }

    pub fn gameplay_constants(&self) -> &GameplayConstants {
        &self.gameplay_constants
    }

    pub fn data_tables(&self) -> &DataTablesDatabase {
        &self.data_tables
    }

    pub fn object_texts(&self) -> &ObjectTextDatabase {
        &self.object_texts
    }

    pub fn default_positions(&self) -> &DefaultPositionsDatabase {
        &self.default_positions
    }

    pub fn system_keybinds(&self) -> &SystemKeybindsDatabase {
        &self.system_keybinds
    }

    pub fn ability_metadata(&self) -> &AbilityMetadataDatabase {
        &self.ability_metadata
    }

    pub fn upgrade_swaps(&self) -> &UpgradeSwapDatabase {
        &self.upgrade_swaps
    }

    fn object_text_lookup(&self, id: &str) -> Option<&crate::ObjectText> {
        if let Some(direct) = self.object_texts.get(id) {
            return Some(direct);
        }
        for (key, value) in self.object_texts.iter() {
            if key.eq_ignore_ascii_case(id) {
                return Some(value);
            }
        }
        None
    }

    fn resolve_object_tip_levels(&self, id: &str) -> Vec<String> {
        let Some(entry) = self.object_text_lookup(id) else {
            return Vec::new();
        };
        entry
            .tip_levels()
            .iter()
            .map(|raw| self.substitute_placeholders(raw))
            .map(|text| strip_wc3_format_codes(&text))
            .collect()
    }

    fn resolve_object_ubertip_levels(&self, id: &str) -> Vec<String> {
        let Some(entry) = self.object_text_lookup(id) else {
            return Vec::new();
        };
        entry
            .ubertip_levels()
            .iter()
            .map(|raw| self.substitute_placeholders(raw))
            .map(|text| strip_wc3_format_codes(&text))
            .collect()
    }

    fn build_object_with_text(
        &self,
        lookup_id: &str,
        identity: WarcraftObjectIdentity,
        meta: WarcraftObjectMeta,
        unit_ubertip_override: Option<String>,
    ) -> WarcraftObject {
        let tip_strings = self.resolve_object_tip_levels(lookup_id);
        let ubertip_strings = match unit_ubertip_override {
            Some(ubertip) => vec![ubertip],
            None => self.resolve_object_ubertip_levels(lookup_id),
        };
        let tip_static = Self::leak_str_slice(&tip_strings);
        let ubertip_static = Self::leak_str_slice(&ubertip_strings);
        let un_tip_static = self
            .resolve_object_un_tip(lookup_id)
            .map(|text| Self::leak(&text));
        let un_ubertip_static = self
            .resolve_object_un_ubertip(lookup_id)
            .map(|text| Self::leak(&text));
        let has_alt = un_tip_static.is_some() || un_ubertip_static.is_some();
        let has_text = !tip_static.is_empty() || !ubertip_static.is_empty();
        let object = if has_alt {
            let text = WarcraftObjectText::with_alt(
                tip_static,
                ubertip_static,
                un_tip_static,
                un_ubertip_static,
            );
            WarcraftObject::with_text(
                identity.id,
                identity.names,
                identity.icons,
                identity.kind,
                identity.race,
                meta,
                text,
            )
        } else if has_text {
            let text = WarcraftObjectText::new(tip_static, ubertip_static);
            WarcraftObject::with_text(
                identity.id,
                identity.names,
                identity.icons,
                identity.kind,
                identity.race,
                meta,
                text,
            )
        } else {
            WarcraftObject::new(
                identity.id,
                identity.names,
                identity.icons,
                identity.kind,
                identity.race,
                meta,
            )
        };
        let position_entry = self.lookup_default_position(lookup_id);
        object
            .with_default_position(position_entry.and_then(|entry| entry.button_position()))
            .with_default_research_position(
                position_entry.and_then(|entry| entry.research_button_position()),
            )
    }

    fn lookup_default_position(&self, id: &str) -> Option<&crate::DefaultPositionEntry> {
        if let Some(direct) = self.default_positions.get(id) {
            return Some(direct);
        }
        for (key, value) in self.default_positions.iter() {
            if key.eq_ignore_ascii_case(id) {
                return Some(value);
            }
        }
        None
    }

    fn leak_str_slice(values: &[String]) -> &'static [&'static str] {
        if values.is_empty() {
            return &[];
        }
        let leaked: Vec<&'static str> = values.iter().map(|value| Self::leak(value)).collect();
        Box::leak(leaked.into_boxed_slice())
    }

    fn resolve_object_un_tip(&self, id: &str) -> Option<String> {
        let raw = self.object_text_lookup(id)?.un_tip()?;
        Some(self.substitute_placeholders(raw))
    }

    fn resolve_object_un_ubertip(&self, id: &str) -> Option<String> {
        let raw = self.object_text_lookup(id)?.un_ubertip()?;
        Some(self.substitute_placeholders(raw))
    }

    pub fn items(&self) -> &ItemDatabase {
        &self.items
    }

    pub fn upgrades(&self) -> &UpgradeDatabase {
        &self.upgrades
    }

    fn get_ability_icon_by_id(&self, id: &str) -> Option<String> {
        Self::case_insensitive_get(self.skins.ability.on_icons(), id)
    }

    fn get_ability_off_icon_by_id(&self, id: &str) -> Option<String> {
        Self::case_insensitive_get(self.skins.ability.off_icons(), id)
    }

    fn get_unit_icon_by_id(&self, id: &str) -> Option<String> {
        Self::case_insensitive_get(&self.skins.unit, id)
    }

    fn get_item_icon_by_id(&self, id: &str) -> Option<String> {
        Self::case_insensitive_get(&self.skins.item, id)
    }

    fn case_insensitive_get(
        map: &std::collections::BTreeMap<String, String>,
        id: &str,
    ) -> Option<String> {
        if let Some(direct) = map.get(id) {
            return Some(direct.clone());
        }
        for (key, value) in map.iter() {
            if key.eq_ignore_ascii_case(id) {
                return Some(value.clone());
            }
        }
        None
    }

    fn resolve_unit_name(&self, race: Race, id: &str) -> Option<&str> {
        match race {
            Race::Human => self.strings.human_unit_strings.get(id),
            Race::Nightelf => self.strings.nightelf_unit_strings.get(id),
            Race::Orc => self.strings.orc_unit_strings.get(id),
            Race::Undead => self.strings.undead_unit_strings.get(id),
            Race::Neutral => self.strings.neutral_unit_strings.get(id),
        }
        .or_else(|| self.strings.campaign_unit_strings.get(id))
        .map(|unit_string| unit_string.value())
    }

    fn resolve_ability_name(&self, race: Option<Race>, id: &str) -> Option<&str> {
        let primary_lookup = match race {
            Some(Race::Human) => {
                Self::ability_string_lookup(&self.strings.human_ability_strings, id)
            }
            Some(Race::Nightelf) => {
                Self::ability_string_lookup(&self.strings.nightelf_ability_strings, id)
            }
            Some(Race::Orc) => Self::ability_string_lookup(&self.strings.orc_ability_strings, id),
            Some(Race::Undead) => {
                Self::ability_string_lookup(&self.strings.undead_ability_strings, id)
            }
            Some(Race::Neutral) => {
                Self::ability_string_lookup(&self.strings.neutral_ability_strings, id)
            }
            None => Self::ability_string_lookup(&self.strings.item_ability_strings, id),
        };
        primary_lookup
            .or_else(|| Self::ability_string_lookup(&self.strings.common_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.human_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.nightelf_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.orc_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.undead_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.neutral_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.item_ability_strings, id))
            .or_else(|| Self::ability_string_lookup(&self.strings.campaign_ability_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.human_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.nightelf_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.orc_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.undead_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.neutral_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.item_unit_strings, id))
            .or_else(|| Self::unit_string_lookup(&self.strings.campaign_unit_strings, id))
    }

    fn unit_string_lookup<'table>(
        table: &'table crate::RaceUnitStringsDatabase,
        id: &str,
    ) -> Option<&'table str> {
        if let Some(direct) = table.get(id) {
            return Some(direct.value());
        }
        for (key, value) in table.iter() {
            if key.eq_ignore_ascii_case(id) {
                return Some(value.value());
            }
        }
        None
    }

    fn unit_ubertip_lookup<'table>(
        table: &'table crate::RaceUnitStringsDatabase,
        id: &str,
    ) -> Option<&'table str> {
        if let Some(direct) = table.get(id) {
            return direct.ubertip();
        }
        for (key, value) in table.iter() {
            if key.eq_ignore_ascii_case(id) {
                return value.ubertip();
            }
        }
        None
    }

    fn resolve_unit_ubertip(&self, race: Race, id: &str) -> Option<String> {
        let raw = match race {
            Race::Human => Self::unit_ubertip_lookup(&self.strings.human_unit_strings, id),
            Race::Nightelf => Self::unit_ubertip_lookup(&self.strings.nightelf_unit_strings, id),
            Race::Orc => Self::unit_ubertip_lookup(&self.strings.orc_unit_strings, id),
            Race::Undead => Self::unit_ubertip_lookup(&self.strings.undead_unit_strings, id),
            Race::Neutral => Self::unit_ubertip_lookup(&self.strings.neutral_unit_strings, id),
        }
        .or_else(|| Self::unit_ubertip_lookup(&self.strings.campaign_unit_strings, id))
        .or_else(|| Self::unit_ubertip_lookup(&self.strings.common_unit_strings, id))?;
        let substituted = self.substitute_placeholders(raw);
        Some(strip_wc3_format_codes(&substituted))
    }

    fn ability_string_lookup<'table>(
        table: &'table crate::RaceAbilityStringsDatabase,
        id: &str,
    ) -> Option<&'table str> {
        if let Some(direct) = table.get(id) {
            return Some(direct.value());
        }
        for (key, value) in table.iter() {
            if key.eq_ignore_ascii_case(id) {
                return Some(value.value());
            }
        }
        None
    }

    fn ability_ubertip_lookup<'table>(
        table: &'table crate::RaceAbilityStringsDatabase,
        id: &str,
    ) -> Option<&'table str> {
        if let Some(direct) = table.get(id) {
            return direct.ubertip();
        }
        for (key, value) in table.iter() {
            if key.eq_ignore_ascii_case(id) {
                return value.ubertip();
            }
        }
        None
    }

    fn ability_research_ubertip_lookup<'table>(
        table: &'table crate::RaceAbilityStringsDatabase,
        id: &str,
    ) -> Option<&'table str> {
        if let Some(direct) = table.get(id) {
            return direct.research_ubertip();
        }
        for (key, value) in table.iter() {
            if key.eq_ignore_ascii_case(id) {
                return value.research_ubertip();
            }
        }
        None
    }

    fn resolve_ability_ubertip(&self, race: Option<Race>, id: &str) -> Option<&str> {
        let primary_lookup = match race {
            Some(Race::Human) => {
                Self::ability_ubertip_lookup(&self.strings.human_ability_strings, id)
            }
            Some(Race::Nightelf) => {
                Self::ability_ubertip_lookup(&self.strings.nightelf_ability_strings, id)
            }
            Some(Race::Orc) => Self::ability_ubertip_lookup(&self.strings.orc_ability_strings, id),
            Some(Race::Undead) => {
                Self::ability_ubertip_lookup(&self.strings.undead_ability_strings, id)
            }
            Some(Race::Neutral) => {
                Self::ability_ubertip_lookup(&self.strings.neutral_ability_strings, id)
            }
            None => Self::ability_ubertip_lookup(&self.strings.item_ability_strings, id),
        };
        primary_lookup
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.common_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.human_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.nightelf_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.orc_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.undead_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.neutral_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.item_ability_strings, id))
            .or_else(|| Self::ability_ubertip_lookup(&self.strings.campaign_ability_strings, id))
    }

    fn resolve_ability_research_ubertip(&self, race: Option<Race>, id: &str) -> Option<&str> {
        let primary_lookup = match race {
            Some(Race::Human) => {
                Self::ability_research_ubertip_lookup(&self.strings.human_ability_strings, id)
            }
            Some(Race::Nightelf) => {
                Self::ability_research_ubertip_lookup(&self.strings.nightelf_ability_strings, id)
            }
            Some(Race::Orc) => {
                Self::ability_research_ubertip_lookup(&self.strings.orc_ability_strings, id)
            }
            Some(Race::Undead) => {
                Self::ability_research_ubertip_lookup(&self.strings.undead_ability_strings, id)
            }
            Some(Race::Neutral) => {
                Self::ability_research_ubertip_lookup(&self.strings.neutral_ability_strings, id)
            }
            None => Self::ability_research_ubertip_lookup(&self.strings.item_ability_strings, id),
        };
        primary_lookup
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.common_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.human_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.nightelf_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.orc_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.undead_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.neutral_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.item_ability_strings, id)
            })
            .or_else(|| {
                Self::ability_research_ubertip_lookup(&self.strings.campaign_ability_strings, id)
            })
    }

    fn resolve_item_name(&self, id: &str) -> Option<&str> {
        self.strings
            .item_unit_strings
            .get(id)
            .map(|item_string| item_string.value())
    }

    fn normalize_icon_path(&self, path: &str) -> String {
        let path = path.to_lowercase();

        path.strip_prefix("replaceabletextures/")
            .unwrap_or(&path)
            .to_string()
    }

    fn item_form_abilities_for_race(&self, race: Race) -> Vec<String> {
        // Collect ability codes from the command-card abilList of any hero of this
        // race, but only for abilities whose race matches this race. Item-form
        // abilities (item=1) whose code matches one of these are racial passives
        // that the game shows in the hero research panel for all heroes of that race.
        let race_str = match race {
            Race::Human => "human",
            Race::Nightelf => "nightelf",
            Race::Orc => "orc",
            Race::Undead => "undead",
            Race::Neutral => "creeps",
        };
        let mut hero_abil_codes: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        if let Some(kinds) = self.units.get(&race)
            && let Some(heroes) = kinds.get(&UnitKind::Hero)
        {
            for hero_id in heroes.keys() {
                if let Some(entry) = self.unit_abilities.get(hero_id.as_str()) {
                    for ability_id in entry.abilities() {
                        let abil_race = self.data_table_lookup(ability_id, "race");
                        if abil_race
                            .as_deref()
                            .is_some_and(|r| r.trim().eq_ignore_ascii_case(race_str))
                            && let Some(code) = self.data_table_lookup(ability_id, "code")
                        {
                            hero_abil_codes.insert(code.trim().to_ascii_lowercase());
                        }
                    }
                }
            }
        }
        self.data_tables
            .iter()
            .filter(|(_, cols)| {
                cols.get("item").map(|i| i.trim() == "1").unwrap_or(false)
                    && cols
                        .get("code")
                        .map(|c| hero_abil_codes.contains(&c.trim().to_ascii_lowercase()))
                        .unwrap_or(false)
            })
            .map(|(alias, _)| alias.clone())
            .collect()
    }

    fn data_table_lookup(&self, entity_id: &str, field_name: &str) -> Option<String> {
        let direct_match = self.data_tables.get(entity_id);
        let entity_entry = match direct_match {
            Some(entry) => entry,
            None => self
                .data_tables
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(entity_id))
                .map(|(_, entry)| entry)?,
        };
        let direct_field = entity_entry.get(field_name);
        let value_str = match direct_field {
            Some(value) => value.as_str(),
            None => entity_entry
                .iter()
                .find(|(key, _)| key.eq_ignore_ascii_case(field_name))
                .map(|(_, value)| value.as_str())?,
        };
        let trimmed = value_str.trim().trim_matches('"');
        if trimmed.is_empty() || trimmed == "-" || trimmed == "_" {
            return None;
        }
        Some(trimmed.to_string())
    }

    fn build_unit_combat(&self, id: &str) -> UnitCombat {
        let hit_points = self
            .data_table_lookup(id, "realHP")
            .or_else(|| self.data_table_lookup(id, "HP"))
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or(0);
        let hit_points_regen = self
            .data_table_lookup(id, "regenHP")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let regen_type = self
            .data_table_lookup(id, "regenType")
            .map(|raw_value| RegenType::parse(&raw_value))
            .unwrap_or_default();
        let armor = self
            .data_table_lookup(id, "realdef")
            .or_else(|| self.data_table_lookup(id, "def"))
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let defense_type = self
            .data_table_lookup(id, "defType")
            .map(|raw_value| DefenseType::parse(&raw_value))
            .unwrap_or_default();
        let weapons_enabled = self
            .data_table_lookup(id, "weapsOn")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or(0);
        let attack = if weapons_enabled > 0 {
            self.build_unit_attack(id)
        } else {
            None
        };
        let mana = self
            .data_table_lookup(id, "realM")
            .or_else(|| self.data_table_lookup(id, "Mana"))
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or(0);
        let mana_regen = self
            .data_table_lookup(id, "regenMana")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let mut combat = UnitCombat::new(
            hit_points,
            hit_points_regen,
            regen_type,
            armor,
            defense_type,
            attack,
        );
        if mana > 0 {
            combat = combat.with_mana_pool(ManaPool::new(mana, mana_regen));
        }
        combat
    }

    fn build_unit_attack(&self, id: &str) -> Option<UnitAttack> {
        let base_damage_min = self
            .data_table_lookup(id, "mindmg1")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or_else(|| {
                let damage_base = self
                    .data_table_lookup(id, "dmgplus1")
                    .and_then(|raw_value| raw_value.parse::<u32>().ok())
                    .unwrap_or(0);
                let dice_count = self
                    .data_table_lookup(id, "dice1")
                    .and_then(|raw_value| raw_value.parse::<u32>().ok())
                    .unwrap_or(0);
                damage_base + dice_count
            });
        let base_damage_max = self
            .data_table_lookup(id, "maxdmg1")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or_else(|| {
                let damage_base = self
                    .data_table_lookup(id, "dmgplus1")
                    .and_then(|raw_value| raw_value.parse::<u32>().ok())
                    .unwrap_or(0);
                let dice_count = self
                    .data_table_lookup(id, "dice1")
                    .and_then(|raw_value| raw_value.parse::<u32>().ok())
                    .unwrap_or(0);
                let dice_sides = self
                    .data_table_lookup(id, "sides1")
                    .and_then(|raw_value| raw_value.parse::<u32>().ok())
                    .unwrap_or(0);
                damage_base + dice_count * dice_sides
            });
        if base_damage_min == 0 && base_damage_max == 0 {
            return None;
        }
        let primary_bonus = self
            .build_hero_attributes(id)
            .map(|hero_attributes| match hero_attributes.primary() {
                PrimaryAttribute::Strength => hero_attributes.strength(),
                PrimaryAttribute::Agility => hero_attributes.agility(),
                PrimaryAttribute::Intelligence => hero_attributes.intelligence(),
            })
            .unwrap_or(0);
        let damage_min = base_damage_min + primary_bonus;
        let damage_max = base_damage_max + primary_bonus;
        let attack_range = self
            .data_table_lookup(id, "rangeN1")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())
            .unwrap_or(0);
        let cooldown_seconds = self
            .data_table_lookup(id, "cool1")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let attack_type = self
            .data_table_lookup(id, "atkType1")
            .map(|raw_value| AttackType::parse(&raw_value))
            .unwrap_or_default();
        let weapon_type = self
            .data_table_lookup(id, "weapTp1")
            .map(|raw_value| WeaponType::parse(&raw_value))
            .unwrap_or_default();
        let unit_attack = UnitAttack::new(
            damage_min,
            damage_max,
            attack_range,
            cooldown_seconds,
            attack_type,
            weapon_type,
        );
        Some(unit_attack)
    }

    fn build_hero_attributes(&self, id: &str) -> Option<HeroAttributes> {
        let strength = self
            .data_table_lookup(id, "STR")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())?;
        let agility = self
            .data_table_lookup(id, "AGI")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())?;
        let intelligence = self
            .data_table_lookup(id, "INT")
            .and_then(|raw_value| raw_value.parse::<u32>().ok())?;
        let primary = self
            .data_table_lookup(id, "Primary")
            .and_then(|raw_value| PrimaryAttribute::parse(&raw_value))?;
        let int_mana_bonus = self.gameplay_constants.int_mana_bonus();
        let mana = intelligence * int_mana_bonus;
        let mana_regen = self
            .data_table_lookup(id, "regenMana")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let strength_per_level = self
            .data_table_lookup(id, "STRplus")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let agility_per_level = self
            .data_table_lookup(id, "AGIplus")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let intelligence_per_level = self
            .data_table_lookup(id, "INTplus")
            .and_then(|raw_value| raw_value.parse::<f32>().ok())
            .unwrap_or(0.0);
        let mana_pool = ManaPool::new(mana, mana_regen);
        let base = AttributeBase::new(strength, agility, intelligence);
        let growth = AttributeGrowth::new(
            strength_per_level,
            agility_per_level,
            intelligence_per_level,
        );
        let hero_attributes = HeroAttributes::new(mana_pool, base, growth, primary);
        Some(hero_attributes)
    }

    fn substitute_placeholders(&self, raw: &str) -> String {
        let mut output = String::with_capacity(raw.len());
        let mut remainder = raw;
        loop {
            let Some(open_offset) = remainder.find('<') else {
                output.push_str(remainder);
                return output;
            };
            output.push_str(&remainder[..open_offset]);
            let after_open = &remainder[open_offset + 1..];
            let Some(close_offset) = after_open.find('>') else {
                output.push('<');
                remainder = after_open;
                continue;
            };
            let placeholder_body = &after_open[..close_offset];
            let after_placeholder = &after_open[close_offset + 1..];
            let resolved = self.resolve_placeholder_body(placeholder_body);
            output.push_str(&resolved);
            remainder = after_placeholder;
        }
    }

    fn resolve_placeholder_body(&self, body: &str) -> String {
        let parts: Vec<&str> = body.splitn(3, ',').collect();
        if parts.len() < 2 {
            return format!("<{body}>");
        }
        let entity_id = parts[0].trim();
        let field_name = parts[1].trim();
        let format_hint = parts.get(2).map(|hint| hint.trim()).unwrap_or("");
        let Some(value_string) = self.data_table_lookup(entity_id, field_name) else {
            return String::from("?");
        };
        self.format_substituted_value(&value_string, format_hint)
    }

    fn format_substituted_value(&self, value: &str, format_hint: &str) -> String {
        let trimmed_value = value.trim();
        if format_hint.is_empty() {
            return Self::strip_trailing_zero_decimal(trimmed_value);
        }
        let Ok(numeric_value) = trimmed_value.parse::<f32>() else {
            return trimmed_value.to_string();
        };
        match format_hint {
            "%" | "100%" => Self::format_number(numeric_value * 100.0),
            "i" | "int" => {
                let rounded = numeric_value.round();
                let integer: i64 =
                    num_traits::cast::cast(rounded).expect("rounded value representable as i64");
                format!("{integer}")
            }
            "f1" => format!("{numeric_value:.1}"),
            "f2" => format!("{numeric_value:.2}"),
            _ => trimmed_value.to_string(),
        }
    }

    fn strip_trailing_zero_decimal(value: &str) -> String {
        let Ok(parsed) = value.parse::<f32>() else {
            return value.to_string();
        };
        Self::format_number(parsed)
    }

    fn format_number(value: f32) -> String {
        if (value - value.round()).abs() < 1e-4 {
            let rounded = value.round();
            let integer: i64 =
                num_traits::cast::cast(rounded).expect("rounded value representable as i64");
            format!("{integer}")
        } else {
            format!("{value:.2}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        }
    }

    fn leak(s: &str) -> &'static str {
        Box::leak(s.to_string().into_boxed_str())
    }

    fn leak_slice<T: Clone>(slice: &[T]) -> &'static [T] {
        Box::leak(slice.to_vec().into_boxed_slice())
    }

    fn leak_object_ids(string_ids: &[String]) -> &'static [WarcraftObjectId] {
        let object_ids: Vec<WarcraftObjectId> = string_ids
            .iter()
            .map(|raw_id| {
                let leaked_id = Self::leak(raw_id);
                WarcraftObjectId::new(leaked_id)
            })
            .collect();
        Box::leak(object_ids.into_boxed_slice())
    }

    fn get_ids(&self) -> ObjectMap {
        let unit_ids = self.get_unit_ids();
        let ability_ids = self.get_ability_ids();
        let upgrade_ids = self.get_upgrade_ids();
        let item_ids = self.get_item_ids();
        let command_ids = self.get_command_ids();
        self.merge_ids(vec![
            unit_ids,
            ability_ids,
            upgrade_ids,
            item_ids,
            command_ids,
        ])
    }

    fn get_command_ids(&self) -> ObjectMap {
        let mut map = ObjectMap::new();
        for (command_id, entry) in &self.command_defaults {
            let leaked_id = Self::leak(command_id);
            let object_id = WarcraftObjectId::new(leaked_id);
            let pretty_name = Self::leak(&Self::pretty_command_name(command_id));
            let names = Self::leak_slice(&[pretty_name]);
            // Worker build commands all carry the generic "basic structure"
            // art, but in game the build button is the race build ability, whose
            // icon is race specific. Prefer the build ability's icon so the
            // editor shows the same per-race button the game renders.
            let build_ability_icon = Self::build_ability_for_command(command_id)
                .and_then(|ability_id| self.get_ability_icon_by_id(ability_id))
                .map(|raw_icon| self.normalize_icon_path(&raw_icon));
            let icon_path =
                build_ability_icon.or_else(|| entry.art().map(Self::resolve_command_icon));
            let icons: &'static [&'static str] = match icon_path {
                Some(path) => Self::leak_slice(&[Self::leak(path.as_str())]),
                None => &[],
            };
            let tip_static = entry.tip().map(Self::leak);
            let ubertip_static = entry
                .ubertip()
                .map(|raw_text| self.substitute_placeholders(raw_text))
                .map(|substituted| Self::leak(&substituted));
            let command_meta =
                CommandMeta::with_text(entry.button_position(), tip_static, ubertip_static);
            let identity = WarcraftObjectIdentity::new(
                object_id,
                names,
                icons,
                WarcraftObjectKind::Command,
                None,
            );
            let warcraft_object = self.build_object_with_text(
                command_id,
                identity,
                WarcraftObjectMeta::Command(command_meta),
                None,
            );
            map.insert(object_id, warcraft_object);
        }
        map
    }

    /// Maps a worker build command to the race build ability whose icon the
    /// live game renders for that button. Returns `None` for non-build commands.
    fn build_ability_for_command(command_id: &str) -> Option<&'static str> {
        let lowered = command_id.to_ascii_lowercase();
        match lowered.as_str() {
            "cmdbuildhuman" => Some("AHbu"),
            "cmdbuildorc" => Some("AObu"),
            "cmdbuildundead" => Some("AUbu"),
            "cmdbuildnightelf" => Some("AEbu"),
            _ => None,
        }
    }

    fn resolve_command_icon(art: &str) -> String {
        let lowered = art.trim_start_matches("Command").to_ascii_lowercase();
        let icon_stem: &str = match lowered.as_str() {
            "rally" => "rallypoint",
            "newskill" => "skillz",
            "purchase" => "hire",
            other if other.starts_with("basicstruct") => "basicstruct",
            other => return format!("commandbuttons/btn{other}.blp"),
        };
        format!("commandbuttons/btn{icon_stem}.blp")
    }

    fn pretty_command_name(command_id: &str) -> String {
        let stripped = command_id.strip_prefix("Cmd").unwrap_or(command_id);
        if stripped.is_empty() {
            return command_id.to_string();
        }
        let mut output = String::with_capacity(stripped.len() + 2);
        for (index, character) in stripped.chars().enumerate() {
            if index > 0 && character.is_ascii_uppercase() {
                output.push(' ');
            }
            output.push(character);
        }
        output
    }

    fn get_unit_ids(&self) -> ObjectMap {
        let human_ids = self.get_unit_ids_from_race(&Race::Human);
        let nightelf_ids = self.get_unit_ids_from_race(&Race::Nightelf);
        let orc_ids = self.get_unit_ids_from_race(&Race::Orc);
        let undead_ids = self.get_unit_ids_from_race(&Race::Undead);
        let neutral_ids = self.get_unit_ids_from_race(&Race::Neutral);
        self.merge_ids(vec![
            human_ids,
            nightelf_ids,
            orc_ids,
            undead_ids,
            neutral_ids,
        ])
    }

    /// Finds the source-form suppression that applies to `target_unit_id`, if
    /// any (Rule 2). Two SLK shapes reach a transformed unit:
    ///   - "Call to Arms" abilities name the base unit in `DataA1` and the
    ///     transformed unit in `DataB1` (peasant → militia).
    ///   - toggle-morph abilities (Bear Form, Raven Form) name the morphed unit
    ///     in `UnitID1` and carry no `DataB1`. `UnitID1` is overloaded — summon
    ///     abilities name their summoned unit there too — so a morph is only
    ///     recognized when the target unit carries the ability in its own list
    ///     (the toggle it uses to revert). A summoned unit never lists the
    ///     ability that summons it.
    fn transform_suppression_for(&self, target_unit_id: &str) -> Option<TransformSuppression> {
        for (ability_id, meta) in &self.ability_metadata {
            if let Some(to_unit) = meta.transform_to_unit()
                && to_unit.eq_ignore_ascii_case(target_unit_id)
                && let Some(from_unit) = meta.transform_from_unit()
            {
                let suppression = TransformSuppression {
                    transform_ability_id: ability_id.clone(),
                    base_unit_id: from_unit.to_string(),
                };
                return Some(suppression);
            }
            if let Some(morph_target) = meta.morph_target_unit()
                && morph_target.eq_ignore_ascii_case(target_unit_id)
                && self.unit_lists_ability(target_unit_id, ability_id)
                && let Some(base_unit_id) = self.base_form_for_morph(ability_id, target_unit_id)
            {
                let suppression = TransformSuppression {
                    transform_ability_id: ability_id.clone(),
                    base_unit_id,
                };
                return Some(suppression);
            }
        }
        None
    }

    /// The base form of a toggle morph: the unit other than the morphed target
    /// whose ability list also carries the morph ability.
    fn base_form_for_morph(&self, morph_ability_id: &str, target_unit_id: &str) -> Option<String> {
        self.unit_abilities.iter().find_map(|(unit_id, entry)| {
            if unit_id.eq_ignore_ascii_case(target_unit_id) {
                return None;
            }
            let lists_morph = entry
                .abilities()
                .iter()
                .any(|ability_id| ability_id.eq_ignore_ascii_case(morph_ability_id));
            if lists_morph {
                Some(unit_id.clone())
            } else {
                None
            }
        })
    }

    fn unit_lists_ability(&self, unit_id: &str, ability_id: &str) -> bool {
        let Some(entry) = self.unit_abilities.get(unit_id) else {
            return false;
        };
        entry
            .abilities()
            .iter()
            .any(|listed_id| listed_id.eq_ignore_ascii_case(ability_id))
    }

    fn get_unit_ids_from_race(&self, race: &Race) -> ObjectMap {
        let mut map = ObjectMap::new();

        let race_units = match self.units.get(race) {
            Some(v) => v,
            None => return map,
        };

        for (kind, units) in race_units {
            for (id, unit) in units {
                let leaked_id = Self::leak(id);
                let object_id = WarcraftObjectId::new(leaked_id);

                let icon = self.get_unit_icon_by_id(id).unwrap_or_default();
                let icon = self.normalize_icon_path(&icon);

                let Some(name) = self.resolve_unit_name(*race, id) else {
                    continue;
                };

                let leaked_name = Self::leak(name);
                let leaked_icon = Self::leak(icon.as_str());
                let names = Self::leak_slice(&[leaked_name]);
                let icons = Self::leak_slice(&[leaked_icon]);
                let build_time = unit.build_time();
                let level = unit.level();
                let gold_cost = unit.gold_cost();
                let abilities_for_unit: &'static [WarcraftObjectId] = {
                    let mut combined: Vec<String> = match self.unit_abilities.get(id) {
                        Some(entry) => entry.abilities().to_vec(),
                        None => Vec::new(),
                    };
                    for supplementary_id in supplementary_abilities_for(id) {
                        if !combined
                            .iter()
                            .any(|existing| existing.eq_ignore_ascii_case(supplementary_id))
                        {
                            combined.push((*supplementary_id).to_string());
                        }
                    }
                    // Shops: units with the purchase-item ability (Apit) also receive the
                    // select-user ability (Anei), added implicitly by the game engine.
                    if combined.iter().any(|a| a.eq_ignore_ascii_case("Apit"))
                        && !combined.iter().any(|a| a.eq_ignore_ascii_case("Anei"))
                    {
                        combined.push("Anei".to_string());
                    }
                    // Rule 1: suppress research-gated abilities from pre-upgrade units.
                    // A unit with a Replace-Unit (Acha) ability is a "pre-upgrade" form;
                    // any ability in its list that carries a `Requires=` in the func files
                    // only becomes active after the research completes (which replaces the
                    // unit), so hide it here.
                    let is_pre_upgrade_form = combined.iter().any(|ability_id| {
                        self.ability_metadata
                            .get(ability_id.as_str())
                            .and_then(|m| m.code())
                            == Some("Acha")
                    });
                    if is_pre_upgrade_form {
                        combined.retain(|ability_id| {
                            self.ability_defaults
                                .get(ability_id.as_str())
                                .and_then(|entry| entry.requires())
                                .is_none()
                        });
                    }
                    // Rule 2: suppress source-form abilities from transform-target units.
                    // E.g. militia (hmil) is the DataB1 target of the Amil ability whose
                    // DataA1 is the peasant (hpea). Militia's SLK list inherits Harvest
                    // and Repair from the peasant, but those abilities belong to the
                    // base form and should not appear on the transformed unit. Toggle
                    // morphs (Bear Form, Raven Form) encode the morphed unit in UnitID1
                    // instead, so the Druid's bear/crow form inherits the caster form's
                    // Rejuvenation/Cyclone the same way — see transform_suppression_for.
                    let suppression = self.transform_suppression_for(id);
                    if let Some(suppression) = suppression
                        && let Some(from_entry) =
                            self.unit_abilities.get(suppression.base_unit_id())
                    {
                        let from_abilities: std::collections::HashSet<String> = from_entry
                            .abilities()
                            .iter()
                            .map(|ability_id| ability_id.to_ascii_lowercase())
                            .collect();
                        let transform_ability_id = suppression.transform_ability_id();
                        combined.retain(|ability_id| {
                            // Always keep the transform ability itself.
                            ability_id.eq_ignore_ascii_case(transform_ability_id)
                                || !from_abilities.contains(&ability_id.to_ascii_lowercase())
                        });
                    }
                    // Rule 3: when a unit has both a code variant (e.g. Aslo) and
                    // its autocast counterpart (e.g. ACsw whose .code() == "Aslo"),
                    // remove the code variant only when both occupy the same button
                    // position — the autocast version supersedes it at that slot.
                    // Self-references (ability.code == ability itself) are skipped.
                    let superseded: std::collections::HashSet<String> = combined
                        .iter()
                        .filter_map(|ability_id| {
                            let code_id = self
                                .ability_metadata
                                .get(ability_id.as_str())
                                .and_then(|meta| meta.code())?;
                            if code_id.eq_ignore_ascii_case(ability_id) {
                                return None;
                            }
                            let main_pos = self
                                .ability_defaults
                                .get(ability_id.as_str())
                                .and_then(|d| d.button_position());
                            let code_pos = self
                                .ability_defaults
                                .get(code_id)
                                .and_then(|d| d.button_position());
                            if main_pos.is_some() && main_pos == code_pos {
                                Some(code_id.to_ascii_lowercase())
                            } else {
                                None
                            }
                        })
                        .collect();
                    if !superseded.is_empty() {
                        combined.retain(|ability_id| {
                            !superseded.contains(&ability_id.to_ascii_lowercase())
                        });
                    }
                    // Rule 4: among abilities sharing the same `.code` field, keep
                    // the LAST occurrence and drop earlier ones.  CASC alphabetical
                    // order processes `_balance/custom_v0` and `_balance/melee_v0`
                    // before `_balance/custom_v1` and `units/`, so the
                    // competitive-balance variants (e.g. ACd2, ACf2) appear later
                    // in the merged list and win.  Earlier alternative-mode entries
                    // (e.g. ACdm, ACfu from custom_v0/melee_v0) are superseded.
                    // Self-references (ability.code == ability itself) are skipped.
                    {
                        let mut last_seen_codes: std::collections::HashMap<String, String> =
                            std::collections::HashMap::new();
                        for ability_id in combined.iter().rev() {
                            let code_option = self
                                .ability_metadata
                                .get(ability_id.as_str())
                                .and_then(|meta| meta.code());
                            if let Some(code_id) = code_option {
                                if code_id.eq_ignore_ascii_case(ability_id) {
                                    continue;
                                }
                                let code_lower = code_id.to_ascii_lowercase();
                                last_seen_codes
                                    .entry(code_lower)
                                    .or_insert_with(|| ability_id.to_ascii_lowercase());
                            }
                        }
                        let superseded_by_last: std::collections::HashSet<String> = combined
                            .iter()
                            .filter_map(|ability_id| {
                                let code_option = self
                                    .ability_metadata
                                    .get(ability_id.as_str())
                                    .and_then(|meta| meta.code());
                                if let Some(code_id) = code_option {
                                    if code_id.eq_ignore_ascii_case(ability_id) {
                                        return None;
                                    }
                                    let code_lower = code_id.to_ascii_lowercase();
                                    let last = last_seen_codes.get(&code_lower)?;
                                    if !last.eq_ignore_ascii_case(ability_id) {
                                        return Some(ability_id.to_ascii_lowercase());
                                    }
                                }
                                None
                            })
                            .collect();
                        if !superseded_by_last.is_empty() {
                            combined.retain(|ability_id| {
                                !superseded_by_last.contains(&ability_id.to_ascii_lowercase())
                            });
                        }
                    }
                    // Rule 5: suppress balance-patch duplicate abilities — same slot,
                    // same name, "last wins".  Toggle+passive pairs were already split
                    // by split_toggle_passive_positions() (passive has no button_position
                    // after that step, so it is invisible here).
                    Self::suppress_same_slot_duplicates(&mut combined, self, *race);
                    Self::leak_object_ids(&combined)
                };
                let hero_abilities_for_unit: &'static [WarcraftObjectId] = {
                    let mut hero_combined: Vec<String> = self
                        .unit_abilities
                        .get(id)
                        .map(|e| e.hero_abilities().to_vec())
                        .unwrap_or_default();
                    // Heroes receive the item-form (item=1) variants of racial abilities
                    // in their research menu. These are absent from unitabilities.slk and
                    // must be derived from abilitydata.slk.
                    if matches!(kind, UnitKind::Hero) {
                        for item_alias in self.item_form_abilities_for_race(*race) {
                            if !hero_combined
                                .iter()
                                .any(|a| a.eq_ignore_ascii_case(&item_alias))
                            {
                                hero_combined.push(item_alias);
                            }
                        }
                    }
                    // Rule 5 (hero abilities): same same-slot deduplication as for
                    // regular abilities above.
                    Self::suppress_same_slot_duplicates(&mut hero_combined, self, *race);
                    Self::leak_object_ids(&hero_combined)
                };
                let ui_flags = self.unit_ui_flags.get(id);
                let is_campaign = ui_flags.map(|entry| entry.is_campaign()).unwrap_or(false);
                let is_in_editor = ui_flags.map(|entry| entry.is_in_editor()).unwrap_or(false);
                let is_hidden_in_editor = ui_flags
                    .map(|entry| entry.is_hidden_in_editor())
                    .unwrap_or(false);
                let is_special = ui_flags.map(|entry| entry.is_special()).unwrap_or(false);
                let unit_data_entry = self.unit_data.get(id);
                let builds_for_unit: &'static [WarcraftObjectId] = match unit_data_entry {
                    Some(entry) => Self::leak_object_ids(entry.builds()),
                    None => &[],
                };
                let trains_for_unit: &'static [WarcraftObjectId] = match unit_data_entry {
                    Some(entry) => Self::leak_object_ids(entry.trains()),
                    None => &[],
                };
                let researches_for_unit: &'static [WarcraftObjectId] = match unit_data_entry {
                    Some(entry) => {
                        let mut combined: Vec<String> = entry.researches().to_vec();
                        for upgrade_id in entry.upgrades() {
                            combined.push(upgrade_id.clone());
                        }
                        Self::leak_object_ids(&combined)
                    }
                    None => &[],
                };
                let sell_items_for_unit: &'static [WarcraftObjectId] = {
                    let combined: Vec<String> = match unit_data_entry {
                        Some(entry) => {
                            let mut items = entry.sell_items().to_vec();
                            for make_item in entry.make_items() {
                                if !items
                                    .iter()
                                    .any(|existing| existing.eq_ignore_ascii_case(make_item))
                                {
                                    items.push(make_item.clone());
                                }
                            }
                            items
                        }
                        None => Vec::new(),
                    };
                    Self::leak_object_ids(&combined)
                };
                let sell_units_for_unit: &'static [WarcraftObjectId] = match unit_data_entry {
                    Some(entry) => Self::leak_object_ids(entry.sell_units()),
                    None => &[],
                };
                let resolved_kind = match *kind {
                    UnitKind::Hero | UnitKind::Building => *kind,
                    _ => {
                        let is_worker = unit_data_entry
                            .map(|entry| entry.is_worker() || !entry.builds().is_empty())
                            .unwrap_or(false);
                        if is_worker {
                            UnitKind::Worker
                        } else {
                            UnitKind::Soldier
                        }
                    }
                };
                let production = UnitProduction::new(
                    researches_for_unit,
                    builds_for_unit,
                    trains_for_unit,
                    sell_items_for_unit,
                    sell_units_for_unit,
                );
                let flags =
                    UnitFlags::new(is_campaign, is_in_editor, is_hidden_in_editor, is_special);
                let combat = self.build_unit_combat(id);
                let mut unit_meta = UnitMeta::with_full_and_extras(
                    resolved_kind,
                    build_time,
                    abilities_for_unit,
                    hero_abilities_for_unit,
                    production,
                    flags,
                )
                .with_combat(combat)
                .with_level(level)
                .with_gold_cost(gold_cost);
                if let Some(hero_attributes) = self.build_hero_attributes(id) {
                    unit_meta = unit_meta.with_hero_attributes(hero_attributes);
                }
                let identity = WarcraftObjectIdentity::new(
                    object_id,
                    names,
                    icons,
                    WarcraftObjectKind::Unit,
                    Some(*race),
                );
                let unit_ubertip = self.resolve_unit_ubertip(*race, id);
                let unit_meta_wrapped = WarcraftObjectMeta::Unit(unit_meta);
                let warcraft_object =
                    self.build_object_with_text(id, identity, unit_meta_wrapped, unit_ubertip);
                map.insert(object_id, warcraft_object);
            }
        }

        map
    }

    fn get_ability_ids(&self) -> ObjectMap {
        let mut map = ObjectMap::new();

        for abilities in self.heroes().values() {
            for hero_ability in abilities {
                let hero_id = hero_ability.id();
                let icon = self.get_ability_icon_by_id(hero_id).unwrap_or_default();
                let icon = self.normalize_icon_path(&icon);
                let hero_race = hero_ability.race();
                let Some(name) = self.resolve_ability_name(Some(hero_race), hero_id) else {
                    continue;
                };

                let leaked_id = Self::leak(hero_id);
                let ability_id = WarcraftObjectId::new(leaked_id);
                let leaked_name = Self::leak(name);
                let leaked_icon = Self::leak(icon.as_str());
                let names = Self::leak_slice(&[leaked_name]);
                let icons = Self::leak_slice(&[leaked_icon]);
                let max_level = hero_ability.max_level();
                let is_ultimate = hero_ability.is_ultimate();
                let cooldowns = hero_ability.cooldowns();
                let defaults = self.ability_defaults.get(hero_id);
                let default_position = defaults.and_then(|entry| entry.button_position());
                let default_research_position =
                    defaults.and_then(|entry| entry.research_button_position());
                let raw_ubertip = self
                    .resolve_ability_ubertip(Some(hero_race), hero_id)
                    .or_else(|| defaults.and_then(|entry| entry.ubertip()));
                let raw_research_ubertip = self
                    .resolve_ability_research_ubertip(Some(hero_race), hero_id)
                    .or_else(|| defaults.and_then(|entry| entry.research_ubertip()));
                let ubertip_static = raw_ubertip
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let research_ubertip_static = raw_research_ubertip
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let metadata = self.ability_metadata.get(hero_id);
                let code_static = metadata.and_then(|entry| entry.code()).map(Self::leak);
                let evasion_chances = metadata
                    .map(|entry| entry.evasion_chance_per_level())
                    .unwrap_or([0.0; 4]);
                let morph_target = metadata
                    .and_then(|entry| entry.morph_target_unit())
                    .map(Self::leak)
                    .map(WarcraftObjectId::new);
                let off_button_position = defaults.and_then(|entry| entry.off_button_position());
                let off_tip_static = defaults
                    .and_then(|entry| entry.off_tip())
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let off_ubertip_static = defaults
                    .and_then(|entry| entry.off_ubertip())
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let off_icon_from_defaults = defaults
                    .and_then(|entry| entry.off_icon())
                    .map(str::to_owned);
                let off_icon_from_skins = self.get_ability_off_icon_by_id(hero_id);
                let raw_off_icon = off_icon_from_defaults.or(off_icon_from_skins);
                let off_icon_static = raw_off_icon
                    .map(|raw_icon| self.normalize_icon_path(&raw_icon))
                    .map(|normalized| Self::leak(&normalized));
                let ability_meta = AbilityMeta::with_ubertips(
                    max_level,
                    is_ultimate,
                    cooldowns,
                    default_position,
                    default_research_position,
                    ubertip_static,
                    research_ubertip_static,
                )
                .with_code(code_static)
                .with_morph_target(morph_target)
                .with_evasion_chances(evasion_chances)
                .with_off_state(
                    off_button_position,
                    off_tip_static,
                    off_ubertip_static,
                    off_icon_static,
                );
                let identity = WarcraftObjectIdentity::new(
                    ability_id,
                    names,
                    icons,
                    WarcraftObjectKind::Ability,
                    None,
                );
                let warcraft_object = self.build_object_with_text(
                    hero_id,
                    identity,
                    WarcraftObjectMeta::Ability(ability_meta),
                    None,
                );
                map.insert(ability_id, warcraft_object);
            }
        }

        for (unit_id, abilities_entry) in &self.unit_abilities {
            let race_for_unit = race_from_unit_id(unit_id);
            let hero_ability_id_set: std::collections::HashSet<&String> =
                abilities_entry.hero_abilities().iter().collect();
            for ability_string_id in abilities_entry
                .abilities()
                .iter()
                .chain(abilities_entry.hero_abilities().iter())
            {
                if map.contains_key(ability_string_id.as_str()) {
                    continue;
                }
                let icon = self
                    .get_ability_icon_by_id(ability_string_id)
                    .unwrap_or_default();
                let icon = self.normalize_icon_path(&icon);
                let resolved_name_option =
                    self.resolve_ability_name(race_for_unit, ability_string_id);
                let Some(resolved_name) = resolved_name_option else {
                    continue;
                };

                let leaked_id = Self::leak(ability_string_id);
                let ability_id = WarcraftObjectId::new(leaked_id);
                let leaked_name = Self::leak(resolved_name);
                let leaked_icon = Self::leak(icon.as_str());
                let names = Self::leak_slice(&[leaked_name]);
                let icons: &'static [&'static str] = if leaked_icon.is_empty() {
                    &[]
                } else {
                    Self::leak_slice(&[leaked_icon])
                };
                let defaults = self.ability_defaults.get(ability_string_id);
                let default_position = defaults.and_then(|entry| entry.button_position());
                let default_research_position =
                    defaults.and_then(|entry| entry.research_button_position());
                let raw_ubertip = self
                    .resolve_ability_ubertip(race_for_unit, ability_string_id)
                    .or_else(|| defaults.and_then(|entry| entry.ubertip()));
                let raw_research_ubertip = self
                    .resolve_ability_research_ubertip(race_for_unit, ability_string_id)
                    .or_else(|| defaults.and_then(|entry| entry.research_ubertip()));
                let ubertip_static = raw_ubertip
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let research_ubertip_static = raw_research_ubertip
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                // Abilities listed in a unit's hero ability slots but absent from the
                // hero database (hero=0 in abilitydata.slk) need their level count and
                // ultimate flag derived from the raw SLK data so the hotkey editor can
                // distinguish them from passive racial items (item=1), which must not
                // appear in the command card.
                let is_hero_listed = hero_ability_id_set.contains(ability_string_id);
                let resolved_max_level: usize;
                let resolved_is_ultimate: bool;
                if is_hero_listed {
                    let slk_levels = self
                        .data_table_lookup(ability_string_id, "levels")
                        .and_then(|raw_value| raw_value.parse::<usize>().ok())
                        .unwrap_or(1);
                    let is_item_form_ability = self
                        .data_table_lookup(ability_string_id, "item")
                        .map(|raw_value| raw_value.trim() == "1")
                        .unwrap_or(false);
                    resolved_max_level = if is_item_form_ability {
                        1
                    } else {
                        slk_levels.max(1)
                    };
                    resolved_is_ultimate = !is_item_form_ability && slk_levels <= 1;
                } else {
                    resolved_max_level = 1;
                    resolved_is_ultimate = false;
                }
                let zero_cooldowns: [u32; 4] = [0, 0, 0, 0];
                let metadata = self.ability_metadata.get(ability_string_id);
                let code_static = metadata.and_then(|entry| entry.code()).map(Self::leak);
                let evasion_chances = metadata
                    .map(|entry| entry.evasion_chance_per_level())
                    .unwrap_or([0.0; 4]);
                let morph_target = metadata
                    .and_then(|entry| entry.morph_target_unit())
                    .map(Self::leak)
                    .map(WarcraftObjectId::new);
                let off_button_position = defaults.and_then(|entry| entry.off_button_position());
                let off_tip_static = defaults
                    .and_then(|entry| entry.off_tip())
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let off_ubertip_static = defaults
                    .and_then(|entry| entry.off_ubertip())
                    .map(|text| self.substitute_placeholders(text))
                    .map(|text| Self::leak(&text));
                let off_icon_from_defaults = defaults
                    .and_then(|entry| entry.off_icon())
                    .map(str::to_owned);
                let off_icon_from_skins = self.get_ability_off_icon_by_id(ability_string_id);
                let raw_off_icon = off_icon_from_defaults.or(off_icon_from_skins);
                let off_icon_static = raw_off_icon
                    .map(|raw_icon| self.normalize_icon_path(&raw_icon))
                    .map(|normalized| Self::leak(&normalized));
                let ability_meta = AbilityMeta::with_ubertips(
                    resolved_max_level,
                    resolved_is_ultimate,
                    zero_cooldowns,
                    default_position,
                    default_research_position,
                    ubertip_static,
                    research_ubertip_static,
                )
                .with_code(code_static)
                .with_morph_target(morph_target)
                .with_evasion_chances(evasion_chances)
                .with_off_state(
                    off_button_position,
                    off_tip_static,
                    off_ubertip_static,
                    off_icon_static,
                );
                let identity = WarcraftObjectIdentity::new(
                    ability_id,
                    names,
                    icons,
                    WarcraftObjectKind::Ability,
                    None,
                );
                let warcraft_object = self.build_object_with_text(
                    ability_string_id,
                    identity,
                    WarcraftObjectMeta::Ability(ability_meta),
                    None,
                );
                map.insert(ability_id, warcraft_object);
            }
        }

        // Third pass: implicit abilities added by the game engine that are absent from unitabilities.slk.
        // Apply the same inference rules as get_unit_ids_from_race using race/kind from self.units.
        let mut inferred: std::collections::BTreeMap<String, Option<Race>> =
            std::collections::BTreeMap::new();
        for (race, kinds) in &self.units {
            for (kind, units) in kinds {
                if matches!(kind, UnitKind::Hero) {
                    for item_alias in self.item_form_abilities_for_race(*race) {
                        inferred.entry(item_alias).or_insert(Some(*race));
                    }
                }
                for unit_id in units.keys() {
                    let raw: Vec<String> = self
                        .unit_abilities
                        .get(unit_id.as_str())
                        .map(|e| e.abilities().to_vec())
                        .unwrap_or_default();
                    if raw.iter().any(|a| a.eq_ignore_ascii_case("Apit")) {
                        inferred.entry("Anei".to_string()).or_insert(Some(*race));
                    }
                }
            }
        }
        // The four worker build abilities (AHbu/AObu/AUbu/AEbu) are real
        // command-card buttons the live game renders, and the game reads their
        // grid position from the ability. No unit's abilList references them and
        // they carry no name string, so the passes above skip them. Add them
        // explicitly for every race that fields a worker; their icon comes from
        // abilityskin.txt and their Buttonpos (0,2) from the ability defaults.
        struct BuildAbilityInfo {
            race: Race,
            ability_id: &'static str,
            fallback_name: &'static str,
        }
        const BUILD_ABILITIES: &[BuildAbilityInfo] = &[
            BuildAbilityInfo {
                race: Race::Human,
                ability_id: "AHbu",
                fallback_name: "Build Structure",
            },
            BuildAbilityInfo {
                race: Race::Orc,
                ability_id: "AObu",
                fallback_name: "Build Structure",
            },
            BuildAbilityInfo {
                race: Race::Nightelf,
                ability_id: "AEbu",
                fallback_name: "Create Building",
            },
            BuildAbilityInfo {
                race: Race::Undead,
                ability_id: "AUbu",
                fallback_name: "Summon Building",
            },
        ];
        for build_ability in BUILD_ABILITIES {
            let race_units = self.units.get(&build_ability.race);
            let has_worker = race_units.is_some_and(|kinds| {
                let worker_units = kinds.get(&UnitKind::Worker);
                worker_units.is_some_and(|units| !units.is_empty())
            });
            if !has_worker {
                continue;
            }
            let ability_key = build_ability.ability_id.to_string();
            inferred
                .entry(ability_key)
                .or_insert(Some(build_ability.race));
        }

        for (ability_id_str, race) in &inferred {
            if map.contains_key(ability_id_str.as_str()) {
                continue;
            }
            let icon = self
                .get_ability_icon_by_id(ability_id_str)
                .unwrap_or_default();
            let icon = self.normalize_icon_path(&icon);
            let resolved_name = match self.resolve_ability_name(*race, ability_id_str) {
                Some(name) => name,
                None => {
                    let build_ability = BUILD_ABILITIES
                        .iter()
                        .find(|info| info.ability_id == ability_id_str.as_str());
                    let Some(build_ability) = build_ability else {
                        continue;
                    };
                    build_ability.fallback_name
                }
            };
            let leaked_id = Self::leak(ability_id_str);
            let ability_id = WarcraftObjectId::new(leaked_id);
            let leaked_name = Self::leak(resolved_name);
            let leaked_icon = Self::leak(icon.as_str());
            let names = Self::leak_slice(&[leaked_name]);
            let icons: &'static [&'static str] = if leaked_icon.is_empty() {
                &[]
            } else {
                Self::leak_slice(&[leaked_icon])
            };
            let defaults = self.ability_defaults.get(ability_id_str.as_str());
            let default_position = defaults.and_then(|entry| entry.button_position());
            let raw_ubertip = self
                .resolve_ability_ubertip(*race, ability_id_str)
                .or_else(|| defaults.and_then(|entry| entry.ubertip()));
            let ubertip_static = raw_ubertip
                .map(|text| self.substitute_placeholders(text))
                .map(|text| Self::leak(&text));
            let metadata = self.ability_metadata.get(ability_id_str.as_str());
            let code_static = metadata.and_then(|entry| entry.code()).map(Self::leak);
            let evasion_chances = metadata
                .map(|entry| entry.evasion_chance_per_level())
                .unwrap_or([0.0; 4]);
            let morph_target = metadata
                .and_then(|entry| entry.morph_target_unit())
                .map(Self::leak)
                .map(WarcraftObjectId::new);
            let off_button_position = defaults.and_then(|entry| entry.off_button_position());
            let off_tip_static = defaults
                .and_then(|entry| entry.off_tip())
                .map(|text| self.substitute_placeholders(text))
                .map(|text| Self::leak(&text));
            let off_ubertip_static = defaults
                .and_then(|entry| entry.off_ubertip())
                .map(|text| self.substitute_placeholders(text))
                .map(|text| Self::leak(&text));
            let off_icon_from_defaults = defaults
                .and_then(|entry| entry.off_icon())
                .map(str::to_owned);
            let off_icon_from_skins = self.get_ability_off_icon_by_id(ability_id_str);
            let raw_off_icon = off_icon_from_defaults.or(off_icon_from_skins);
            let off_icon_static = raw_off_icon
                .map(|raw_icon| self.normalize_icon_path(&raw_icon))
                .map(|normalized| Self::leak(&normalized));
            let default_research_position = defaults
                .and_then(|entry| entry.research_button_position())
                .or_else(|| defaults.and_then(|entry| entry.button_position()));
            let research_ubertip_static = defaults
                .and_then(|entry| entry.research_ubertip())
                .map(|text| self.substitute_placeholders(text))
                .map(|text| Self::leak(&text));
            let ability_meta = AbilityMeta::with_ubertips(
                1,
                false,
                [0, 0, 0, 0],
                default_position,
                default_research_position,
                ubertip_static,
                research_ubertip_static,
            )
            .with_code(code_static)
            .with_morph_target(morph_target)
            .with_evasion_chances(evasion_chances)
            .with_off_state(
                off_button_position,
                off_tip_static,
                off_ubertip_static,
                off_icon_static,
            );
            let identity = WarcraftObjectIdentity::new(
                ability_id,
                names,
                icons,
                WarcraftObjectKind::Ability,
                None,
            );
            let warcraft_object = self.build_object_with_text(
                ability_id_str,
                identity,
                WarcraftObjectMeta::Ability(ability_meta),
                None,
            );
            map.insert(ability_id, warcraft_object);
        }

        map
    }

    fn get_upgrade_ids(&self) -> ObjectMap {
        let mut map = ObjectMap::new();

        for entry in self.upgrades.races() {
            for (id, art_def) in entry.art_database() {
                let Some(name_def) = entry.name_database().get(id) else {
                    continue;
                };

                let names: Vec<&'static str> = name_def
                    .get_names()
                    .iter()
                    .map(|name| Self::leak(name))
                    .collect();
                let icons: Vec<&'static str> = art_def
                    .get_icons()
                    .iter()
                    .map(|icon| Self::leak(&self.normalize_icon_path(icon)))
                    .collect();

                let max_level = icons.len().min(names.len());
                let leaked_id = Self::leak(id);
                let object_id = WarcraftObjectId::new(leaked_id);
                let names_slice = Self::leak_slice(&names);
                let icons_slice = Self::leak_slice(&icons);
                let upgrade_meta = UpgradeMeta::new(max_level);

                let identity = WarcraftObjectIdentity::new(
                    object_id,
                    names_slice,
                    icons_slice,
                    WarcraftObjectKind::Upgrade,
                    Some(entry.race()),
                );
                let warcraft_object = self.build_object_with_text(
                    id,
                    identity,
                    WarcraftObjectMeta::Upgrade(upgrade_meta),
                    None,
                );
                map.insert(object_id, warcraft_object);
            }
        }

        map
    }

    fn get_item_ids(&self) -> ObjectMap {
        let mut map = ObjectMap::new();

        for (item_class, items_per_class) in self.items() {
            for (id, item) in items_per_class {
                let leaked_id = Self::leak(id);
                let object_id = WarcraftObjectId::new(leaked_id);

                let icon = self.get_item_icon_by_id(id).unwrap_or_default();
                let icon = self.normalize_icon_path(&icon);

                let ability_ids: Vec<WarcraftObjectId> = item
                    .ability_list()
                    .iter()
                    .map(|ability| {
                        let leaked_ability = Self::leak(ability);
                        WarcraftObjectId::new(leaked_ability)
                    })
                    .collect();
                let abilities: &'static [WarcraftObjectId] = Self::leak_slice(&ability_ids);

                let cooldown_id = item.cooldown_id().map(|cooldown_id| {
                    let leaked_cooldown_id = Self::leak(cooldown_id);
                    WarcraftObjectId::new(leaked_cooldown_id)
                });

                let Some(name) = self.resolve_item_name(id) else {
                    continue;
                };

                let leaked_name = Self::leak(name);
                let leaked_icon = Self::leak(icon.as_str());
                let names = Self::leak_slice(&[leaked_name]);
                let icons = Self::leak_slice(&[leaked_icon]);
                let item_meta = ItemMeta::new(*item_class, abilities, cooldown_id);
                let identity = WarcraftObjectIdentity::new(
                    object_id,
                    names,
                    icons,
                    WarcraftObjectKind::Item,
                    None,
                );
                let warcraft_object = self.build_object_with_text(
                    id,
                    identity,
                    WarcraftObjectMeta::Item(item_meta),
                    None,
                );
                map.insert(object_id, warcraft_object);
            }
        }

        map
    }

    fn merge_ids(&self, maps: Vec<ObjectMap>) -> ObjectMap {
        let mut merged = ObjectMap::new();
        for map in maps {
            merged.extend(map);
        }
        merged
    }

    fn merge_heroes(&mut self, heroes: HeroDatabase) {
        for (hero_id, incoming_abilities) in heroes {
            self.heroes
                .entry(hero_id)
                .or_default()
                .extend(incoming_abilities);
        }
    }

    fn merge_units(&mut self, units: UnitDatabase) {
        for (race, kinds) in units {
            let race_bucket = self.units.entry(race).or_default();
            for (kind, units_of_kind) in kinds {
                let kind_bucket = race_bucket.entry(kind).or_default();
                for (unit_id, definition) in units_of_kind {
                    kind_bucket.entry(unit_id).or_insert(definition);
                }
            }
        }
    }

    fn merge_items(&mut self, items: ItemDatabase) {
        for (item_class, items_of_class) in items {
            let class_bucket = self.items.entry(item_class).or_default();
            for (item_id, definition) in items_of_class {
                class_bucket.entry(item_id).or_insert(definition);
            }
        }
    }

    pub fn skins(&self) -> &SkinDatabase {
        &self.skins
    }

    /// Preprocessing step for balance-patch toggle+passive pairs.
    ///
    /// Some balance patches introduce two ability IDs for the same spell where one
    /// is an autocast toggle (has `off_button_position`) and the other is a plain
    /// passive indicator (no off-state).  Both share the same `button_position` and
    /// display name.  They are not duplicates — they serve different sections of the
    /// UI: the toggle appears on the command card, the passive appears in the research
    /// (passive ability) panel.
    ///
    /// This method splits their positions before `suppress_same_slot_duplicates` runs:
    /// - Clears the toggle's `research_button_position` (command card only).
    /// - Clears the passive's `button_position` (research panel only).
    ///
    /// After this step the two abilities no longer conflict in `suppress_same_slot_duplicates`
    /// (the passive has no `button_position`, so it is invisible to Rule 5).
    fn split_toggle_passive_positions(&mut self) {
        struct AbilityButtonEntry {
            ability_id: String,
            button_position: GridCoordinate,
            has_off_state: bool,
        }

        let button_entries: Vec<AbilityButtonEntry> = self
            .ability_defaults
            .iter()
            .filter_map(|(ability_id, entry)| {
                let button_position = entry.button_position()?;
                let has_off_state = entry.off_button_position().is_some();
                let ability_entry = AbilityButtonEntry {
                    ability_id: ability_id.clone(),
                    button_position,
                    has_off_state,
                };
                Some(ability_entry)
            })
            .collect();

        let mut slot_to_ability_ids: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut ability_has_off_state: std::collections::HashMap<String, bool> =
            std::collections::HashMap::new();

        for button_entry in &button_entries {
            let Some(name) = self.resolve_ability_name(None, &button_entry.ability_id) else {
                continue;
            };
            let button_position = button_entry.button_position;
            let slot_key = format!("{button_position}|{name}");
            let ability_id = button_entry.ability_id.clone();
            let ability_ids = slot_to_ability_ids.entry(slot_key).or_default();
            ability_ids.push(ability_id);
            let ability_lower = button_entry.ability_id.to_ascii_lowercase();
            ability_has_off_state.insert(ability_lower, button_entry.has_off_state);
        }

        let mut clear_button_position: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        let mut clear_research_button_position: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for slot_abilities in slot_to_ability_ids.values() {
            if slot_abilities.len() != 2 {
                continue;
            }
            let toggle_count = slot_abilities
                .iter()
                .filter(|ability_id| {
                    let ability_lower = ability_id.to_ascii_lowercase();
                    ability_has_off_state
                        .get(&ability_lower)
                        .copied()
                        .unwrap_or(false)
                })
                .count();
            if toggle_count != 1 {
                continue;
            }
            for ability_id in slot_abilities {
                let ability_lower = ability_id.to_ascii_lowercase();
                let has_off_state = ability_has_off_state
                    .get(&ability_lower)
                    .copied()
                    .unwrap_or(false);
                if has_off_state {
                    clear_research_button_position.insert(ability_lower);
                } else {
                    clear_button_position.insert(ability_lower);
                }
            }
        }

        for (ability_id, entry) in &mut self.ability_defaults {
            let ability_lower = ability_id.to_ascii_lowercase();
            if clear_button_position.contains(&ability_lower) {
                entry.clear_button_position();
            }
            if clear_research_button_position.contains(&ability_lower) {
                entry.clear_research_button_position();
            }
        }
    }

    /// Rule 5 helper: remove abilities that are balance-patch duplicates occupying
    /// the same default button slot with the same display name.
    ///
    /// The CASC additive merge accumulates both the original ability ID (from the
    /// base war3.w3mod) and its replacement (from a balance overlay) on the same
    /// unit.  Two abilities are considered balance-patch duplicates when they share
    /// the same default button position AND the same resolved display name — regardless
    /// of whether either ability is self-referential.
    ///
    /// Toggle+passive pairs (exactly one has off-state) are handled upstream by
    /// `split_toggle_passive_positions` before this function runs: the passive has
    /// its `button_position` cleared, so it does not appear in `slot_to_abilities`
    /// and is never a candidate for suppression here.
    ///
    /// For all remaining same-slot same-name pairs the earlier occurrence is
    /// suppressed — the CASC overlay appears later in the merged list and is the
    /// newer version.
    fn suppress_same_slot_duplicates(
        abilities: &mut Vec<String>,
        aggregation: &WarcraftDataAggregation,
        race: Race,
    ) {
        let mut slot_to_abilities: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();

        for ability_id in abilities.iter() {
            let defaults = aggregation.ability_defaults.get(ability_id.as_str());
            let Some(position) = defaults.and_then(|entry| entry.button_position()) else {
                continue;
            };
            let Some(name) = aggregation.resolve_ability_name(Some(race), ability_id) else {
                continue;
            };
            let slot_key = format!("{position}|{name}");
            slot_to_abilities
                .entry(slot_key)
                .or_default()
                .push(ability_id.clone());
        }

        let mut patch_superseded: std::collections::HashSet<String> =
            std::collections::HashSet::new();

        for slot_abilities in slot_to_abilities.values() {
            if slot_abilities.len() < 2 {
                continue;
            }
            let last_ability = slot_abilities.last().unwrap();
            for ability_id in slot_abilities.iter() {
                if !ability_id.eq_ignore_ascii_case(last_ability) {
                    let ability_lower = ability_id.to_ascii_lowercase();
                    patch_superseded.insert(ability_lower);
                }
            }
        }

        if !patch_superseded.is_empty() {
            abilities.retain(|ability_id| {
                let ability_lower = ability_id.to_ascii_lowercase();
                !patch_superseded.contains(&ability_lower)
            });
        }
    }
}

impl From<Vec<ExtractResult>> for WarcraftDataAggregation {
    fn from(value: Vec<ExtractResult>) -> Self {
        let mut db = Self::default();

        for result in value {
            match result {
                ExtractResult::IO => (),
                ExtractResult::Heroes(heroes) => db.merge_heroes(heroes),
                ExtractResult::Units(units) => db.merge_units(units),
                ExtractResult::UnitAbilities(map) => {
                    for (unit_id, incoming) in map {
                        match db.unit_abilities.get_mut(&unit_id) {
                            Some(existing) => existing.merge_additive(&incoming),
                            None => {
                                db.unit_abilities.insert(unit_id, incoming);
                            }
                        }
                    }
                }
                ExtractResult::AbilityMetadata(map) => {
                    for (alias, incoming) in map {
                        db.ability_metadata.entry(alias).or_insert(incoming);
                    }
                }
                ExtractResult::UpgradeSwaps(swaps) => {
                    for incoming in swaps {
                        db.upgrade_swaps.insert(incoming);
                    }
                }
                ExtractResult::UnitData(map) => {
                    // Union build/train/research/upgrade/sell lists across
                    // overlays. Variants drop fields rather than override
                    // them — see `htow`'s `Researches=Rhpm` going missing
                    // from `_balance/custom_v0/humanunitfunc.txt`.
                    for (unit_id, incoming) in map {
                        match db.unit_data.get_mut(&unit_id) {
                            Some(existing) => existing.merge_additive(&incoming),
                            None => {
                                db.unit_data.insert(unit_id, incoming);
                            }
                        }
                    }
                }
                ExtractResult::UnitUiFlags(map) => {
                    for (unit_id, incoming) in map {
                        db.unit_ui_flags.entry(unit_id).or_insert(incoming);
                    }
                }
                ExtractResult::CommandDefaults(map) => {
                    for (command_id, incoming) in map {
                        let existing = db.command_defaults.entry(command_id).or_default();
                        if existing.button_position().is_none()
                            && let Some(position) = incoming.button_position()
                        {
                            existing.set_button_position(Some(position));
                        }
                        if existing.art().is_none()
                            && let Some(art) = incoming.art()
                        {
                            existing.set_art(Some(art.to_string()));
                        }
                        if existing.tip().is_none()
                            && let Some(tip) = incoming.tip()
                        {
                            existing.set_tip(Some(tip.to_string()));
                        }
                        if existing.ubertip().is_none()
                            && let Some(ubertip) = incoming.ubertip()
                        {
                            existing.set_ubertip(Some(ubertip.to_string()));
                        }
                    }
                }
                ExtractResult::AbilityDefaults(map) => {
                    // Same field-merge story as UnitData: a variant may
                    // drop `Researchbuttonpos` / `Ubertip` lines that the
                    // base or another overlay set.
                    for (ability_id, incoming) in map {
                        match db.ability_defaults.get_mut(&ability_id) {
                            Some(existing) => existing.merge_additive(&incoming),
                            None => {
                                db.ability_defaults.insert(ability_id, incoming);
                            }
                        }
                    }
                }
                ExtractResult::DataTables(map) => {
                    for (entity_id, fields) in map {
                        let entry = db.data_tables.entry(entity_id).or_default();
                        for (field_name, value) in fields {
                            entry.entry(field_name).or_insert(value);
                        }
                    }
                }
                ExtractResult::ObjectTexts(map) => {
                    for (entity_id, incoming) in map {
                        let existing = db.object_texts.entry(entity_id).or_default();
                        existing.merge(incoming);
                    }
                }
                ExtractResult::DefaultPositions(map) => {
                    for (entity_id, incoming) in map {
                        db.default_positions
                            .entry(entity_id)
                            .or_default()
                            .merge(incoming);
                    }
                }
                ExtractResult::SystemKeybinds(entries) => {
                    db.system_keybinds = entries;
                }
                ExtractResult::Items(items) => db.merge_items(items),
                ExtractResult::HumanUpgradesArt(art) => db.upgrades.human_art = art,
                ExtractResult::NightelfUpgradesArt(art) => db.upgrades.nightelf_art = art,
                ExtractResult::OrcUpgradesArt(art) => db.upgrades.orc_art = art,
                ExtractResult::UndeadUpgradesArt(art) => db.upgrades.undead_art = art,
                ExtractResult::HumanUpgradesName(name) => db.upgrades.human_name = name,
                ExtractResult::NightelfUpgradesName(name) => db.upgrades.nightelf_name = name,
                ExtractResult::OrcUpgradesName(name) => db.upgrades.orc_name = name,
                ExtractResult::UndeadUpgradesName(name) => db.upgrades.undead_name = name,
                ExtractResult::AbilitySkin(skin) => db.skins.ability = skin,
                ExtractResult::ItemSkin(skin) => db.skins.item = skin,
                ExtractResult::UnitSkin(skin) => db.skins.unit = skin,
                ExtractResult::HumanAbilityStrings(strings) => {
                    db.strings.human_ability_strings = strings
                }
                ExtractResult::HumanUnitStrings(strings) => db.strings.human_unit_strings = strings,
                ExtractResult::NightelfAbilityStrings(strings) => {
                    db.strings.nightelf_ability_strings = strings
                }
                ExtractResult::NightelfUnitStrings(strings) => {
                    db.strings.nightelf_unit_strings = strings
                }
                ExtractResult::OrcAbilityStrings(strings) => {
                    db.strings.orc_ability_strings = strings
                }
                ExtractResult::OrcUnitStrings(strings) => db.strings.orc_unit_strings = strings,
                ExtractResult::UndeadAbilityStrings(strings) => {
                    db.strings.undead_ability_strings = strings
                }
                ExtractResult::UndeadUnitStrings(strings) => {
                    db.strings.undead_unit_strings = strings
                }
                ExtractResult::NeutralAbilityStrings(strings) => {
                    db.strings.neutral_ability_strings = strings
                }
                ExtractResult::NeutralUnitStrings(strings) => {
                    db.strings.neutral_unit_strings = strings
                }
                ExtractResult::ItemAbilityStrings(strings) => {
                    db.strings.item_ability_strings = strings
                }
                ExtractResult::ItemUnitStrings(strings) => db.strings.item_unit_strings = strings,
                ExtractResult::CampaignUnitStrings(strings) => {
                    db.strings.campaign_unit_strings = strings
                }
                ExtractResult::CampaignAbilityStrings(strings) => {
                    db.strings.campaign_ability_strings = strings
                }
                ExtractResult::CommonAbilityStrings(strings) => {
                    db.strings.common_ability_strings = strings
                }
                ExtractResult::CommonUnitStrings(strings) => {
                    db.strings.common_unit_strings = strings
                }
                ExtractResult::GameplayConstants(constants) => db.gameplay_constants = *constants,
            }
        }

        db
    }
}

#[derive(Default, Debug, Clone)]
pub struct SkinDatabase {
    ability: AbilitySkins,
    unit: UnitSkins,
    item: ItemSkins,
}

struct Wc3FormatText;

impl Wc3FormatText {
    fn strip_codes_once(input: &str) -> String {
        let mut output = String::with_capacity(input.len());
        let mut characters = input.chars().peekable();
        while let Some(current_char) = characters.next() {
            if current_char != '|' {
                output.push(current_char);
                continue;
            }
            let Some(next_char) = characters.peek().copied() else {
                output.push(current_char);
                continue;
            };
            match next_char.to_ascii_lowercase() {
                'n' => {
                    characters.next();
                    output.push(' ');
                }
                'r' => {
                    characters.next();
                }
                'c' => {
                    characters.next();
                    let mut consumed_hex = 0;
                    while consumed_hex < 8 {
                        let Some(peeked_char) = characters.peek().copied() else {
                            break;
                        };
                        if peeked_char.is_ascii_hexdigit() {
                            characters.next();
                            consumed_hex += 1;
                        } else {
                            break;
                        }
                    }
                }
                '|' => {
                    characters.next();
                    output.push('|');
                }
                _ => {
                    output.push(current_char);
                }
            }
        }
        output
    }

    fn strip_codes(input: &str) -> String {
        let mut current = Self::strip_codes_once(input);
        loop {
            let next = Self::strip_codes_once(&current);
            if next == current {
                break;
            }
            current = next;
        }
        let collapsed: Vec<&str> = current.split_whitespace().collect();
        collapsed.join(" ").trim().to_string()
    }
}

fn strip_wc3_format_codes(input: &str) -> String {
    Wc3FormatText::strip_codes(input)
}

fn supplementary_abilities_for(unit_id: &str) -> &'static [&'static str] {
    match unit_id {
        "htow" => &["Amic"],
        "nane" => &["ACss"],
        _ => &[],
    }
}

impl SkinDatabase {
    pub fn ability(&self) -> &AbilitySkins {
        &self.ability
    }

    pub fn unit(&self) -> &UnitSkins {
        &self.unit
    }

    pub fn item(&self) -> &ItemSkins {
        &self.item
    }
}

#[derive(Default, Debug, Clone)]
pub struct UpgradeDatabase {
    human_art: HumanUpgradeArtDatabase,
    human_name: HumanUpgradeNameDatabase,
    nightelf_art: NightelfUpgradeArtDatabase,
    nightelf_name: NightelfUpgradeNameDatabase,
    orc_art: OrcUpgradeArtDatabase,
    orc_name: OrcUpgradeNameDatabase,
    undead_art: UndeadUpgradeArtDatabase,
    undead_name: UndeadUpgradeNameDatabase,
}

impl UpgradeDatabase {
    fn races(&self) -> impl Iterator<Item = RaceUpgradeEntry<'_>> {
        let human_entry = RaceUpgradeEntry::new(Race::Human, &self.human_art, &self.human_name);
        let nightelf_entry =
            RaceUpgradeEntry::new(Race::Nightelf, &self.nightelf_art, &self.nightelf_name);
        let orc_entry = RaceUpgradeEntry::new(Race::Orc, &self.orc_art, &self.orc_name);
        let undead_entry = RaceUpgradeEntry::new(Race::Undead, &self.undead_art, &self.undead_name);
        [human_entry, nightelf_entry, orc_entry, undead_entry].into_iter()
    }
}

#[derive(Default, Debug, Clone)]
pub struct StringsDatabase {
    human_ability_strings: HumanAbilityStringsDatabase,
    human_unit_strings: HumanUnitStringsDatabase,
    nightelf_ability_strings: NightelfAbilityStringsDatabase,
    nightelf_unit_strings: NightelfUnitStringsDatabase,
    orc_ability_strings: OrcAbilityStringsDatabase,
    orc_unit_strings: OrcUnitStringsDatabase,
    undead_ability_strings: UndeadAbilityStringsDatabase,
    undead_unit_strings: UndeadUnitStringsDatabase,
    neutral_ability_strings: NeutralAbilityStringsDatabase,
    neutral_unit_strings: NeutralUnitStringsDatabase,
    item_ability_strings: ItemAbilityStringsDatabase,
    item_unit_strings: ItemUnitStringsDatabase,
    campaign_unit_strings: CampaignUnitStringsDatabase,
    campaign_ability_strings: CampaignAbilityStringsDatabase,
    common_ability_strings: CommonAbilityStringsDatabase,
    common_unit_strings: CommonUnitStringsDatabase,
}
