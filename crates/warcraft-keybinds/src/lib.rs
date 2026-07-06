pub use warcraft_api::{SystemKeybindClass, SystemKeybindModifier, WarcraftObjectId};

pub mod cascade;
pub mod collision;
pub mod command;
pub mod custom_keys;
pub mod display;
pub mod editor_history;
pub mod grid;
pub mod identity;
pub mod model;
pub mod statistics;
pub mod system;
pub mod text;
pub mod unit;

#[cfg(test)]
mod ddd_conformance;

pub use cascade::conflict_graph::{CollidingPair, ConflictGraph, ConflictNode};
pub use cascade::planner::{CascadePlan, MoveReason, PlannedMove, UnresolvedMover};
pub use cascade::queue::{AssignmentQueue, AssignmentScope, GroupKind, PositionAssignmentGroup};

pub use collision::cross_unit::{
    AffectedUnitEntry, CrossUnitCollisionReport, CrossUnitPositionGroup, SharedAbilityEntry,
};

pub use collision::summary::CollisionSummary;
pub use collision::unit_report::{UnitCollisionEntry, UnitCollisionReport};
pub use command::move_request::MoveRequest;
pub use custom_keys::{CustomKeys, DEFAULT_CUSTOM_KEYS, HotkeyConflict, ImportOutcome};
pub use display::ability_cell::{AbilityCell, AbilityIconPath};

pub use display::grid_behavior::{
    AlternateFormBehavior, CommandBehavior, GridBehavior, ResearchBehavior,
};

pub use display::inspector_detail::InspectorDetail;
pub use display::rendered_grid::{CommandGridRenderInput, RenderedTile};
pub use display::templates::{BundledTemplate, ResolvedTemplate};

pub use grid::layout::{
    COMMAND_GRID_COLUMNS, COMMAND_GRID_ROWS, COMMAND_GRID_TILE_COUNT, GridLayout,
};

pub use editor_history::{EditorHistory, EditorSnapshot};

pub use identity::ability_id::AbilityId;
pub use identity::hotkey_target::HotkeyTarget;
pub use identity::hotkey_token::{HotkeyToken, HotkeyTokenIsNotLetter, HotkeyTokenParseError};

pub use identity::keycode::{
    Digit, FunctionKey, KeyCode, KeyCodeOutOfRange, Letter, MouseButton, NotALetter, NumpadKey,
    Punctuation,
};

pub use identity::slot::{CommandCard, GridSlotId};

pub use model::{
    AbilityBinding, AbilityBindingBuilder, AbilityModifier, BindingEntry, ColumnIndex,
    CommandBinding, CommandBindingBuilder, CommandEntry, CustomKeysBuilder, GridCoordinate, Hotkey,
    RowIndex, SystemBinding, WarcraftKeybinding,
};

pub use statistics::{
    Armor, AttackRange, AttackSpeed, AttackStatistics, AttributeStatistic, DamagePerSecond,
    DamageRange, EffectiveHitPoints, Evasion, Gain, HeroStatistics, HitPoints, HitPointsRegen,
    Mana, ManaRegen, Matchup, MatchupStrength, UnitStatistics,
};

pub use system::binding_map::{EffectiveBinding, ResolvedSystemBinding, SystemBindingMap};

pub use unit::grids::{
    CollisionSlots, GridRole, HotkeyCollisionAtCell, HotkeyCollisionCard,
    HotkeyCollisionCardIterator, NamedCommandGrid, PositionCollisionCard,
    PositionCollisionCardIterator, UnitGrids,
};

pub use unit::keyed::{UnitAbilityGroup, UnitAbilitySlot, UnitKeyedCustomKeys};
pub use unit::listing::{
    UnitCategoryEntry, UnitCategoryListing, UnitCategoryRequest, UnitListing, UnitListingEntry,
    UnitListingRequest,
};
pub use unit::slot_containers::UnitSlotContainers;
pub use unit::slots::UnitCommandSlots;
pub use warcraft_database::{BuildingTraits, CommandCatalog};
