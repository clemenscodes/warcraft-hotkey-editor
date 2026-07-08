use tw_macro::{ClassList, tw};
classes! {
    base: tw![
        "group",
        "flex",
        "items-baseline",
        "gap-2",
        "text-xl",
        "leading-title",
        "text-shadow-drop",
        "min-w-0",
        "data-[primary=true]:[text-shadow:1px_1px_0_var(--color-warcraft-shadow),0_0_8px_color-mix(in_oklab,var(--color-warcraft-gold)_35%,transparent)]",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}

/// The attribute name label: gold, brightening to full gold when this is the hero's
/// primary attribute (a reaction to the row's own `data-primary`).
pub(super) const LABEL: ClassList = ClassList::new(
    "flex-[0_1_auto] min-w-0 text-inherit text-warcraft-gold/90 group-data-[primary=true]:text-warcraft-gold",
);
