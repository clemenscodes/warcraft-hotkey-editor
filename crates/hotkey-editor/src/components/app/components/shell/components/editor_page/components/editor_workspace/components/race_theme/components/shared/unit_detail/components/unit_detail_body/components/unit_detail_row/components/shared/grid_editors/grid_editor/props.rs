use crate::services::editor_state::{DragFollower, DraggingSlot, DropTargetTile};
use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use warcraft_api::WarcraftObjectId;
use warcraft_keybinds::CustomKeys;
use warcraft_keybinds::{GridBehavior, GridLayout, GridSlotId};

/// What the caller wants edited: a unit's slot set, the source-of-truth signal,
/// the shared selection/drag state, and behavior flags. This carries no behavior
/// type — callers pick the variant component (command/research/uprooted), which
/// binds the behavior. The data describes the grid to edit, not how it cascades.
#[derive(Props, Clone, PartialEq)]
pub struct GridEditorConfig {
    pub heading: &'static str,
    pub slot_ids: Rc<[GridSlotId]>,
    pub loaded_keys: Signal<Option<CustomKeys>>,
    pub selected_slot: Signal<Option<GridSlotId>>,
    pub selected_from_research: Signal<bool>,
    pub selected_from_uprooted: Signal<bool>,
    pub tier_overrides: Signal<HashMap<WarcraftObjectId, usize>>,
    pub dragging_slot: Signal<Option<DraggingSlot>>,
    pub drop_target_tile: Signal<Option<DropTargetTile>>,
    pub drag_follower: Signal<Option<DragFollower>>,
    pub grid_layout: Signal<GridLayout>,
    pub update_hotkeys_on_move: Signal<bool>,
    pub hotkey_assign_request: Signal<bool>,
    /// When true, drops onto cells already occupied by another slot are
    /// rejected outright instead of swapping. The off-state position
    /// picker uses this so dragging the toggle's off half can't displace
    /// another ability's on-state on the unit's command card.
    #[props(default = false)]
    pub prevent_swap_on_drop: bool,
    /// When non-empty, only slots whose `as_str()` matches one of these
    /// ids start a drag — other slots render in their cells but are
    /// display-only. Used by the off-state picker to keep the player from
    /// accidentally rearranging the unit's primary command card while
    /// editing one toggle's off position.
    #[props(default)]
    pub restrict_draggable_to: Vec<GridSlotId>,
    /// Unit ID of the host — used to block dragging of morph abilities on
    /// alternate-form units (e.g. Burrowed Crypt Fiend). The default (empty
    /// sentinel) id disables the check (off-state picker, build menus without
    /// a unit).
    #[props(default)]
    pub host_unit_id: WarcraftObjectId,
}

/// The `GridEditor` component's props: a [`GridEditorConfig`] bound to a concrete
/// [`GridBehavior`]. The behavior is a zero-sized marker; the three variant
/// wrappers each instantiate it with their own type. It exists so the generic
/// `GridEditor<B>` carries `B`, letting the `From` impls cascade and mutate tiles.
#[derive(Props, Clone, PartialEq)]
pub struct GridEditorProps<B: GridBehavior> {
    pub(crate) behavior: B,
    pub(crate) config: GridEditorConfig,
}

impl<B: GridBehavior> From<&GridEditorConfig> for GridEditorProps<B> {
    fn from(config: &GridEditorConfig) -> Self {
        let behavior = B::default();
        let config = config.clone();
        Self { behavior, config }
    }
}
