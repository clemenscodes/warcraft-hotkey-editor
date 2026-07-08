use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-stretch",
        "px-0",
        "mt-0",
        "pt-0",
        "flex-none",
        "min-h-0",
        "gap-5",
    ],
    mobile: tw![
        "mobile:grid",
        "mobile:grid-cols-[minmax(0,1fr)]",
        "mobile:gap-6",
        "mobile:items-start",
    ],
    tablet: tw![
        "tablet:grid",
        "tablet:grid-cols-[minmax(0,1fr)]",
        "tablet:gap-6",
        "tablet:items-start",
    ],
}

/// The right column holding the hotkey override: the "Hotkey override" heading over
/// the override card. On phones it becomes a sticky bottom sheet, widened and shifted
/// out of the card's padding. Its own inlined element in the detail row.
pub(super) const PANEL: ClassList = ClassList::new(
    "flex flex-col min-w-0 self-start mobile:self-stretch mobile:items-stretch mobile:sticky mobile:bottom-0 mobile:z-100 mobile:-left-[0.85rem] mobile:w-[calc(100%+1.7rem)] mobile:pt-0 tablet:w-full tablet:self-stretch tablet:pt-0",
);
