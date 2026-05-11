pub mod keybind;
pub mod keycode;
pub mod meta;
pub mod object;
pub mod player;
pub mod primitives;
pub mod race_labels;
pub mod version;

pub use keybind::{ContextSet, SystemKeybind, SystemKeybindClass, SystemKeybindModifier};
pub use keycode::KeyCode;
pub use meta::{
    AbilityMeta, AgilityBonuses, AttackType, AttributeBase, AttributeGrowth, CommandMeta,
    DamageEffectiveness, DamageMatrix, DefenseType, GameplayConstants, HeroAttributes,
    IntelligenceBonuses, ItemMeta, ManaPool, PrimaryAttribute, RegenType, StrengthBonuses,
    UnitAttack, UnitCombat, UnitFlags, UnitMeta, UnitProduction, UpgradeMeta,
};
pub use object::{
    ColumnIndex, GridCoordinate, ItemClass, ObjectMap, ParseGridCoordinateError, Race, RowIndex,
    UnitKind, WarcraftDatabase, WarcraftObject, WarcraftObjectId, WarcraftObjectKind,
    WarcraftObjectMeta, WarcraftObjectText,
};
pub use player::{
    AiDifficultyPreference, CampaignMatchType, CustomMatchType, MatchType, MeleeMatchType,
    PlayerColor, PlayerGameResult, PlayerRace, PlayerSlotState, PlayerType, RacePreference, Team,
    TeamPlayer, Teams,
};
pub use primitives::{
    Address, AgentReference, Boolean, Byte, ByteString, Bytes, Float, Identifier, Integer, OwnerId,
    Time,
};
pub use race_labels::{RaceLabels, SUPPORTED_RACES};
pub use version::{SUPPORTED_VERSION, SUPPORTED_VERSION_STRING, WarcraftVersion};
