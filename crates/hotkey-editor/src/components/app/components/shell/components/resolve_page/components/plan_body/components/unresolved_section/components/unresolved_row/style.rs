use tw_macro::{ClassList, tw};
classes! {
    base: tw!["grid"],
}

/// The card surface (the stuck variant): bordered, tinted, orc-accented.
pub(super) const PANEL: ClassList = ClassList::new(
    "flex flex-col py-6 bg-warcraft-bg-mid/45 border rounded-panel gap-5 px-6 box-border border-race-orc/50",
);
/// The abilities row: the stuck ability's column.
pub(super) const FIGHT_ROW: ClassList =
    ClassList::new("flex items-start justify-center gap-6 w-full");
/// The stuck ability column: its name plate over its ability icon.
pub(super) const FIGHT_COLUMN: ClassList =
    ClassList::new("flex flex-[1_1_0] max-w-[50%] flex-col items-center gap-3 min-w-0");
/// The grid block flagging the cell the ability is stuck on.
pub(super) const MOVE_TRANSITION: ClassList =
    ClassList::new("relative flex items-center justify-center gap-6 w-full");
