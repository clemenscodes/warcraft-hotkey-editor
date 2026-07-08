use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "flex",
        "flex-row",
        "items-stretch",
        "gap-3.5",
        "min-w-0",
        "relative",
        "[grid-area:defense]",
    ],
}

/// The rows stacked beside the column's icon.
pub(super) const ROWS: ClassList = ClassList::new("flex flex-col gap-2 min-w-0 flex-[1_1_auto]");
