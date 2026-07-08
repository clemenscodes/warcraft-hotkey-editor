use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}

/// The category label: the row's own gold treatment.
pub(super) const LABEL: ClassList =
    ClassList::new("flex-[0_1_auto] min-w-0 text-inherit text-warcraft-gold/90");
