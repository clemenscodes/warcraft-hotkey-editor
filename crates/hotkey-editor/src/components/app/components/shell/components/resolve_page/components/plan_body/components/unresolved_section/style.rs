use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "flex",
        "flex-col",
        "gap-2",
    ],
}

/// The auto-fill grid of unresolved cards. `.move-list` + `data-category` are coupled
/// to the e2e suite, written literally (shared value with the active move list).
pub(super) const MOVE_LIST: ClassList = ClassList::new(
    "move-list grid grid-cols-[repeat(auto-fill,minmax(min(760px,100%),1fr))] gap-4 content-start",
);
