use tw_macro::ClassList;

/// The blue-edged off-state / upgraded-form block (shared value with the sibling
/// section).
pub(super) const CONTAINER: ClassList = ClassList::new(
    "flex flex-col gap-1 py-3 pr-0 pl-4 bg-warcraft-bg-base/55 border-l-2 border-race-human rounded-l-control text-warcraft-text-secondary text-lg leading-prose",
);
/// The block's top row: label column beside its controls.
pub(super) const HEADER: ClassList =
    ClassList::new("grid grid-cols-[minmax(0,1fr)_auto_auto] items-center gap-x-3.5");
/// The label column of the header row.
pub(super) const HEADER_TEXT: ClassList = ClassList::new("min-w-0");
