use tw_macro::{ClassList, tw};
classes! {
    base: tw!["grid"],
}

/// The card surface (the plan-move variant): bordered, tinted, blue-accented.
pub(super) const PANEL: ClassList = ClassList::new(
    "flex flex-col py-6 bg-warcraft-bg-mid/45 border rounded-panel gap-5 px-6 box-border border-warcraft-blue-deep",
);
/// The fighting-abilities row: the mover column beside the optional rival column.
pub(super) const FIGHT_ROW: ClassList =
    ClassList::new("flex items-start justify-center gap-6 w-full");
/// One fighter column: a name element stacked over its ability icon.
pub(super) const FIGHT_COLUMN: ClassList =
    ClassList::new("flex flex-[1_1_0] max-w-[50%] flex-col items-center gap-3 min-w-0");
/// The from -> to transition block below the abilities.
pub(super) const MOVE_TRANSITION: ClassList =
    ClassList::new("relative flex items-center justify-center gap-6 w-full");
/// One side (from or to) of the transition block.
pub(super) const TRANSITION_COLUMN: ClassList =
    ClassList::new("flex flex-[1_1_0] max-w-[50%] justify-center min-w-0");
