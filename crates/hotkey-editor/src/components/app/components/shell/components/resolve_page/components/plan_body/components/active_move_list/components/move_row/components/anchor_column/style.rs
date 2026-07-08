use tw_macro::ClassList;

/// One fighter column: a name element stacked over its ability icon (shared value with
/// the mover column in the move row).
pub(super) const FIGHT_COLUMN: ClassList =
    ClassList::new("flex flex-[1_1_0] max-w-[50%] flex-col items-center gap-3 min-w-0");
