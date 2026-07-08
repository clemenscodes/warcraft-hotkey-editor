use tw_macro::ClassList;

/// The auto-fill grid of move cards. The `.move-list` class and its `data-category`
/// attribute are coupled to the e2e suite, so they are written literally here (shared
/// value with the unresolved section's list).
pub(super) const MOVE_LIST: ClassList = ClassList::new(
    "move-list grid grid-cols-[repeat(auto-fill,minmax(min(760px,100%),1fr))] gap-4 content-start",
);
