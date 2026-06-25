use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::meta::{AbilityMeta, CommandMeta, ItemMeta, UnitMeta, UpgradeMeta};
use crate::primitives::Identifier;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Race {
    Human,
    Nightelf,
    Orc,
    Undead,
    Neutral,
}

impl TryFrom<&str> for Race {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "human" => Ok(Self::Human),
            "orc" => Ok(Self::Orc),
            "nightelf" => Ok(Self::Nightelf),
            "undead" => Ok(Self::Undead),
            "neutral" => Ok(Self::Neutral),
            _ => Err(()),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnitKind {
    #[default]
    Soldier,
    Worker,
    Hero,
    Building,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum WarcraftObjectKind {
    #[default]
    Unit,
    Ability,
    Upgrade,
    Item,
    Command,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ColumnIndex {
    #[default]
    Zero,
    One,
    Two,
    Three,
}

impl From<ColumnIndex> for u8 {
    fn from(index: ColumnIndex) -> Self {
        match index {
            ColumnIndex::Zero => 0,
            ColumnIndex::One => 1,
            ColumnIndex::Two => 2,
            ColumnIndex::Three => 3,
        }
    }
}

impl TryFrom<u8> for ColumnIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

impl From<ColumnIndex> for usize {
    fn from(index: ColumnIndex) -> Self {
        let byte = u8::from(index);
        usize::from(byte)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RowIndex {
    #[default]
    Zero,
    One,
    Two,
}

impl From<RowIndex> for u8 {
    fn from(index: RowIndex) -> Self {
        match index {
            RowIndex::Zero => 0,
            RowIndex::One => 1,
            RowIndex::Two => 2,
        }
    }
}

impl TryFrom<u8> for RowIndex {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            _ => Err(()),
        }
    }
}

impl From<RowIndex> for usize {
    fn from(index: RowIndex) -> Self {
        let byte = u8::from(index);
        usize::from(byte)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridCoordinate {
    column: ColumnIndex,
    row: RowIndex,
}

impl GridCoordinate {
    pub const fn new(column: ColumnIndex, row: RowIndex) -> Self {
        Self { column, row }
    }

    pub fn column(self) -> ColumnIndex {
        self.column
    }

    pub fn row(self) -> RowIndex {
        self.row
    }
}

impl Default for GridCoordinate {
    fn default() -> Self {
        Self {
            column: ColumnIndex::Zero,
            row: RowIndex::Zero,
        }
    }
}

impl TryFrom<&str> for GridCoordinate {
    type Error = ();

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let mut parts = text.splitn(2, ',');
        let column = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        let row = parts
            .next()
            .ok_or(())?
            .trim()
            .parse::<u8>()
            .map_err(|_| ())?;
        let column = ColumnIndex::try_from(column)?;
        let row = RowIndex::try_from(row)?;
        Ok(GridCoordinate { column, row })
    }
}

impl fmt::Display for GridCoordinate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let column = u8::from(self.column);
        let row = u8::from(self.row);
        write!(formatter, "{column},{row}")
    }
}

#[derive(Debug)]
pub struct ParseGridCoordinateError;

impl fmt::Display for ParseGridCoordinateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("invalid grid coordinate")
    }
}

impl std::error::Error for ParseGridCoordinateError {}

impl FromStr for GridCoordinate {
    type Err = ParseGridCoordinateError;

    fn from_str(text: &str) -> Result<Self, ParseGridCoordinateError> {
        Self::try_from(text).map_err(|()| ParseGridCoordinateError)
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WarcraftObjectId {
    pub(crate) value: &'static str,
}

impl WarcraftObjectId {
    pub const fn new(value: &'static str) -> Self {
        Self { value }
    }

    pub fn value(&self) -> &'static str {
        self.value
    }
}

impl From<&'static str> for WarcraftObjectId {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

/// An upgrade-swap: an upgrade whose effect replaces one trained unit with
/// another (e.g. Berserker upgrading Headhunters `ohun` into Berserkers
/// `otbk`). Extracted from `units/upgradedata.slk` rows whose effect slot is
/// the `rtma` "replace unit" mechanic.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UnitUpgradeSwap {
    from_unit_id: WarcraftObjectId,
    to_unit_id: WarcraftObjectId,
}

impl UnitUpgradeSwap {
    pub const fn new(from_unit_id: &'static str, to_unit_id: &'static str) -> Self {
        let from_unit_id = WarcraftObjectId::new(from_unit_id);
        let to_unit_id = WarcraftObjectId::new(to_unit_id);
        Self {
            from_unit_id,
            to_unit_id,
        }
    }

    pub fn from_unit_id(&self) -> WarcraftObjectId {
        self.from_unit_id
    }

    pub fn to_unit_id(&self) -> WarcraftObjectId {
        self.to_unit_id
    }
}

impl Borrow<str> for WarcraftObjectId {
    fn borrow(&self) -> &str {
        self.value
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemClass {
    Permanent = 0x0,
    Charged = 0x1,
    PowerUp = 0x2,
    Artifact = 0x3,
    #[default]
    Purchasable = 0x4,
    Campaign = 0x5,
    Miscellaneous = 0x6,
    Unknown = 0x7,
    Any = 0x8,
}

impl From<crate::primitives::Integer> for ItemClass {
    fn from(value: crate::primitives::Integer) -> Self {
        use ItemClass::*;
        match value.get_integer() {
            0x0 => Permanent,
            0x1 => Charged,
            0x2 => Artifact,
            0x3 => PowerUp,
            0x4 => Purchasable,
            0x5 => Campaign,
            0x6 => Miscellaneous,
            0x7 => Unknown,
            0x8 => Any,
            _ => Self::default(),
        }
    }
}

impl TryFrom<&str> for ItemClass {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "Artifact" => Ok(ItemClass::Artifact),
            "Permanent" => Ok(ItemClass::Permanent),
            "Charged" => Ok(ItemClass::Charged),
            "PowerUp" => Ok(ItemClass::PowerUp),
            "Campaign" => Ok(ItemClass::Campaign),
            "Miscellaneous" => Ok(ItemClass::Miscellaneous),
            "Purchasable" => Ok(ItemClass::Purchasable),
            _ => Err(()),
        }
    }
}

impl Serialize for ItemClass {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> Deserialize<'de> for ItemClass {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        use ItemClass::*;
        Ok(match value {
            0x0 => Permanent,
            0x1 => Charged,
            0x2 => PowerUp,
            0x3 => Artifact,
            0x4 => Purchasable,
            0x5 => Campaign,
            0x6 => Miscellaneous,
            0x7 => Unknown,
            0x8 => Any,
            _ => Self::default(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WarcraftObjectText {
    pub(crate) tip_levels: &'static [&'static str],
    pub(crate) ubertip_levels: &'static [&'static str],
    pub(crate) un_tip: Option<&'static str>,
    pub(crate) un_ubertip: Option<&'static str>,
}

impl WarcraftObjectText {
    pub const fn new(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip: None,
            un_ubertip: None,
        }
    }

    pub const fn with_alt(
        tip_levels: &'static [&'static str],
        ubertip_levels: &'static [&'static str],
        un_tip: Option<&'static str>,
        un_ubertip: Option<&'static str>,
    ) -> Self {
        Self {
            tip_levels,
            ubertip_levels,
            un_tip,
            un_ubertip,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }
}

pub type ObjectMap = BTreeMap<WarcraftObjectId, WarcraftObject>;

#[derive(Default, Debug, Clone)]
pub struct WarcraftDatabase {
    db: ObjectMap,
    lowercase_index: HashMap<String, WarcraftObjectId>,
}

impl<'a> IntoIterator for &'a WarcraftDatabase {
    type Item = (&'a WarcraftObjectId, &'a WarcraftObject);
    type IntoIter = std::collections::btree_map::Iter<'a, WarcraftObjectId, WarcraftObject>;

    fn into_iter(self) -> Self::IntoIter {
        self.db.iter()
    }
}

impl WarcraftDatabase {
    pub fn new(db: ObjectMap) -> Self {
        let lowercase_index = db
            .keys()
            .map(|key| (key.value().to_ascii_lowercase(), *key))
            .collect();
        Self {
            db,
            lowercase_index,
        }
    }

    pub fn get(&self, id: Identifier) -> Option<&WarcraftObject> {
        self.db.get(id.get_id().as_str())
    }

    pub fn db(&self) -> &ObjectMap {
        &self.db
    }

    pub fn by_id(&self, needle_id: &str) -> Option<&WarcraftObject> {
        let lowercase = needle_id.to_ascii_lowercase();
        let canonical_key = self.lowercase_index.get(&lowercase)?;
        self.db.get(canonical_key.value())
    }

    pub fn by_id_and_key(&self, needle_id: &str) -> Option<(WarcraftObjectId, &WarcraftObject)> {
        let lowercase = needle_id.to_ascii_lowercase();
        let canonical_key = self.lowercase_index.get(&lowercase)?;
        let warcraft_object = self.db.get(canonical_key.value())?;
        Some((*canonical_key, warcraft_object))
    }

    pub fn get_icons(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.icons)
    }

    pub fn get_names(&self, id: Identifier) -> Option<&'static [&'static str]> {
        self.get(id).map(|object| object.names)
    }

    pub fn get_ability_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_upgrade_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Upgrade(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn get_max_level(&self, id: Identifier) -> Option<usize> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.max_level()),
            WarcraftObjectMeta::Upgrade(ref meta) => Some(meta.max_level()),
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self, id: Identifier) -> Option<bool> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.is_ultimate()),
            _ => None,
        }
    }

    pub fn get_ability_cooldown_for_level(&self, id: Identifier, level: usize) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => meta.cooldown_for_level(level),
            _ => None,
        }
    }

    pub fn get_ability_base_cooldown(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.base_cooldown()),
            _ => None,
        }
    }

    pub fn get_ability_cooldowns(&self, id: Identifier) -> Option<[u32; 4]> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Ability(ref meta) => Some(meta.cooldowns()),
            _ => None,
        }
    }

    pub fn get_unit_build_time(&self, id: Identifier) -> Option<u32> {
        match self.get(id)?.meta {
            WarcraftObjectMeta::Unit(ref meta) => Some(meta.build_time()),
            _ => None,
        }
    }

    pub fn ability_names(&self) -> impl Iterator<Item = (&'static str, &'static [&'static str])> {
        self.db.iter().filter_map(|(id, object)| {
            if object.kind == WarcraftObjectKind::Ability {
                Some((id.value, object.names))
            } else {
                None
            }
        })
    }

    pub fn all_ability_names(&'static self) -> impl Iterator<Item = &'static str> {
        self.db.values().filter_map(|object| {
            if object.kind == WarcraftObjectKind::Ability {
                object.names.first().copied()
            } else {
                None
            }
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WarcraftObjectId, &WarcraftObject)> {
        self.db.iter()
    }
}

const ICON_PATH_BLACKLIST: &[&str] = &["commandbuttons/btnselectheroon.blp"];

#[derive(Default, Debug, Clone)]
pub struct WarcraftObject {
    id: WarcraftObjectId,
    names: &'static [&'static str],
    icons: &'static [&'static str],
    kind: WarcraftObjectKind,
    race: Option<Race>,
    meta: WarcraftObjectMeta,
    tip_levels: &'static [&'static str],
    ubertip_levels: &'static [&'static str],
    un_tip: Option<&'static str>,
    un_ubertip: Option<&'static str>,
    default_button_position: Option<GridCoordinate>,
    default_research_button_position: Option<GridCoordinate>,
}

impl WarcraftObject {
    pub fn new(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: &[],
            ubertip_levels: &[],
            un_tip: None,
            un_ubertip: None,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_text(
        id: WarcraftObjectId,
        names: &'static [&'static str],
        icons: &'static [&'static str],
        kind: WarcraftObjectKind,
        race: Option<Race>,
        meta: WarcraftObjectMeta,
        text: WarcraftObjectText,
    ) -> Self {
        Self {
            id,
            names,
            icons,
            kind,
            race,
            meta,
            tip_levels: text.tip_levels,
            ubertip_levels: text.ubertip_levels,
            un_tip: text.un_tip,
            un_ubertip: text.un_ubertip,
            default_button_position: None,
            default_research_button_position: None,
        }
    }

    pub fn with_default_position(mut self, position: Option<GridCoordinate>) -> Self {
        self.default_button_position = position;
        self
    }

    pub fn with_default_research_position(mut self, position: Option<GridCoordinate>) -> Self {
        self.default_research_button_position = position;
        self
    }

    pub fn id(&self) -> WarcraftObjectId {
        self.id
    }

    pub fn names(&self) -> &'static [&'static str] {
        self.names
    }

    pub fn icons(&self) -> &'static [&'static str] {
        self.icons
    }

    pub fn kind(&self) -> WarcraftObjectKind {
        self.kind
    }

    pub fn race(&self) -> Option<Race> {
        self.race
    }

    pub fn meta(&self) -> &WarcraftObjectMeta {
        &self.meta
    }

    pub fn tip(&self) -> Option<&'static str> {
        if let Some(first) = self.tip_levels.first() {
            return Some(*first);
        }
        if let WarcraftObjectMeta::Command(command_meta) = &self.meta {
            return command_meta.tip();
        }
        None
    }

    pub fn ubertip(&self) -> Option<&'static str> {
        if let Some(first) = self.ubertip_levels.first() {
            return Some(*first);
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.ubertip(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.ubertip(),
            _ => None,
        }
    }

    pub fn tip_levels(&self) -> &'static [&'static str] {
        self.tip_levels
    }

    pub fn ubertip_levels(&self) -> &'static [&'static str] {
        self.ubertip_levels
    }

    pub fn research_ubertip(&self) -> Option<&'static str> {
        if let WarcraftObjectMeta::Ability(ability_meta) = &self.meta {
            return ability_meta.research_ubertip();
        }
        None
    }

    pub fn un_tip(&self) -> Option<&'static str> {
        self.un_tip
    }

    pub fn un_ubertip(&self) -> Option<&'static str> {
        self.un_ubertip
    }

    pub fn default_button_position(&self) -> Option<GridCoordinate> {
        if self.default_button_position.is_some() {
            return self.default_button_position;
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.default_button_position(),
            WarcraftObjectMeta::Command(command_meta) => command_meta.default_button_position(),
            _ => None,
        }
    }

    pub fn default_research_button_position(&self) -> Option<GridCoordinate> {
        if self.default_research_button_position.is_some() {
            return self.default_research_button_position;
        }
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => {
                ability_meta.default_research_button_position()
            }
            _ => None,
        }
    }

    pub fn is_ultimate_ability(&self) -> bool {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.is_ultimate(),
            _ => false,
        }
    }

    pub fn cooldowns(&self) -> Option<[u32; 4]> {
        match self.meta() {
            WarcraftObjectMeta::Ability(ability_meta) => Some(ability_meta.cooldowns()),
            _ => None,
        }
    }

    pub fn has_displayable_icon(&self) -> bool {
        self.icons().iter().any(|icon_path| {
            if icon_path.trim().is_empty() {
                return false;
            }
            let normalized = icon_path.trim().to_ascii_lowercase();
            !ICON_PATH_BLACKLIST.contains(&normalized.as_str())
        })
    }

    pub fn is_passive_ability(&self) -> bool {
        self.icons()
            .first()
            .map(|icon_path| {
                icon_path
                    .to_ascii_lowercase()
                    .starts_with("passivebuttons/")
            })
            .unwrap_or(false)
    }

    pub fn ability_code(&self) -> Option<&'static str> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.code(),
            _ => None,
        }
    }

    pub fn ability_morph_target_id(&self) -> Option<&'static str> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => {
                ability_meta.morph_target_unit().map(|id| id.value())
            }
            _ => None,
        }
    }

    /// The number of research tiers this object has, but only when it is an
    /// upgrade. Multi-level upgrades store one hotkey token per tier
    /// (`Hotkey=F,F,F`), so the editor needs this to size that list. Leveled
    /// abilities (hero spells, auras) are deliberately excluded: their
    /// command-card button is shared across levels and binds a single hotkey, so
    /// they must not be replicated. Non-upgrades return `None`.
    pub fn upgrade_max_level(&self) -> Option<usize> {
        match &self.meta {
            WarcraftObjectMeta::Upgrade(upgrade_meta) => Some(upgrade_meta.max_level()),
            _ => None,
        }
    }

    pub fn ability_off_icon(&self) -> Option<&'static str> {
        match &self.meta {
            WarcraftObjectMeta::Ability(ability_meta) => ability_meta.off_icon(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum WarcraftObjectMeta {
    Unit(UnitMeta),
    Ability(AbilityMeta),
    Upgrade(UpgradeMeta),
    Item(ItemMeta),
    Command(CommandMeta),
}

impl Default for WarcraftObjectMeta {
    fn default() -> Self {
        Self::Unit(UnitMeta::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_coordinate_stores_column_and_row() {
        let coordinate = GridCoordinate::new(ColumnIndex::Three, RowIndex::One);
        assert_eq!(coordinate.column(), ColumnIndex::Three);
        assert_eq!(coordinate.row(), RowIndex::One);
    }

    #[test]
    fn warcraft_object_id_value_round_trips() {
        let id = WarcraftObjectId::new("hpea");
        assert_eq!(id.value(), "hpea");
    }

    #[test]
    fn warcraft_object_id_ordering_is_lexicographic() {
        let alpha = WarcraftObjectId::new("Aaaa");
        let beta = WarcraftObjectId::new("Zzzz");
        assert!(alpha < beta);
    }

    #[test]
    fn warcraft_object_id_borrow_yields_str() {
        use std::borrow::Borrow;
        let id = WarcraftObjectId::new("hfoo");
        let borrowed: &str = id.borrow();
        assert_eq!(borrowed, "hfoo");
    }

    #[test]
    fn item_class_default_is_purchasable() {
        assert_eq!(ItemClass::default(), ItemClass::Purchasable);
    }

    #[test]
    fn item_class_try_from_known_strings() {
        assert_eq!(ItemClass::try_from("Artifact"), Ok(ItemClass::Artifact));
        assert_eq!(ItemClass::try_from("Permanent"), Ok(ItemClass::Permanent));
        assert_eq!(ItemClass::try_from("Charged"), Ok(ItemClass::Charged));
    }

    #[test]
    fn item_class_try_from_unknown_string_is_error() {
        assert!(ItemClass::try_from("NotAClass").is_err());
    }

    #[test]
    fn warcraft_object_meta_default_is_unit_variant() {
        matches!(WarcraftObjectMeta::default(), WarcraftObjectMeta::Unit(_));
    }

    #[test]
    fn warcraft_object_text_accessors_return_slices() {
        let text = WarcraftObjectText::new(&["tip one", "tip two"], &["ubertip"]);
        assert_eq!(text.tip_levels(), &["tip one", "tip two"]);
        assert_eq!(text.ubertip_levels(), &["ubertip"]);
        assert!(text.un_tip().is_none());
    }

    #[test]
    fn warcraft_object_text_with_alt_stores_optional_fields() {
        let text = WarcraftObjectText::with_alt(&[], &[], Some("un tip"), Some("un uber"));
        assert_eq!(text.un_tip(), Some("un tip"));
        assert_eq!(text.un_ubertip(), Some("un uber"));
    }
}
