//! How a command grid resolves and mutates its tiles. Each behavior is a distinct
//! zero-sized type the renderer is generic over, so the choice is made at the type
//! level. The named contexts (the unit's research menu, an uprooted Ancient's menu)
//! are the concrete implementors.

/// The per-grid behavior toggles the domain resolution reads.
pub trait GridBehavior: Clone + PartialEq + Default + 'static {
    /// Read and write the secondary (`Researchbuttonpos`) position namespace and
    /// its hotkey field, and scope conflict checks to it.
    fn research_positions(&self) -> bool;

    /// Co-move a colocated off-state when its on-state is dragged.
    fn co_move_offstate(&self) -> bool;

    /// Treat an empty tile reserved by another ability's off-state as a blocked
    /// drop target.
    fn flag_offstate_collisions(&self) -> bool;

    /// Show the passive badge for passive abilities.
    fn show_passive_badge(&self) -> bool;

    /// Record selections from this grid as coming from the unit's alternate form.
    fn marks_alternate_form(&self) -> bool;
}

/// The ordinary command card (also build menus and off-state position pickers).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct CommandBehavior;

impl GridBehavior for CommandBehavior {
    fn research_positions(&self) -> bool {
        false
    }

    fn co_move_offstate(&self) -> bool {
        true
    }

    fn flag_offstate_collisions(&self) -> bool {
        true
    }

    fn show_passive_badge(&self) -> bool {
        true
    }

    fn marks_alternate_form(&self) -> bool {
        false
    }
}

/// A research menu: positions and hotkeys live in the secondary namespace, and
/// off-state collision and passive decorations do not apply.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct ResearchBehavior;

impl GridBehavior for ResearchBehavior {
    fn research_positions(&self) -> bool {
        true
    }

    fn co_move_offstate(&self) -> bool {
        true
    }

    fn flag_offstate_collisions(&self) -> bool {
        false
    }

    fn show_passive_badge(&self) -> bool {
        false
    }

    fn marks_alternate_form(&self) -> bool {
        false
    }
}

/// An alternate-form menu (an uprooted Ancient): no off-state co-move, and
/// selections record the alternate form.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct AlternateFormBehavior;

impl GridBehavior for AlternateFormBehavior {
    fn research_positions(&self) -> bool {
        false
    }

    fn co_move_offstate(&self) -> bool {
        false
    }

    fn flag_offstate_collisions(&self) -> bool {
        true
    }

    fn show_passive_badge(&self) -> bool {
        true
    }

    fn marks_alternate_form(&self) -> bool {
        true
    }
}
