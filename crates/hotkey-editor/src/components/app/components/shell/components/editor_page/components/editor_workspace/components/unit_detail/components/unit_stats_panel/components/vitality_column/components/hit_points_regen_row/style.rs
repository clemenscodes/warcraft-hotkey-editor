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
        "mt-[-0.2rem]",
        "pl-5",
    ],
    mobile: tw!["mobile:text-2xl", "mobile:leading-heading"],
}

/// The indented regeneration label: a dimmer, smaller gold than the headline rows.
pub(super) const LABEL: ClassList =
    ClassList::new("flex-[0_1_auto] min-w-0 text-inherit text-warcraft-gold/70 text-xl");

/// The regeneration gain: green, pushed to the row's end, dimmed when there is no
/// regeneration.
pub(super) const GAIN: ClassList = ClassList::new(
    "flex-[0_0_auto] ml-auto text-right text-warcraft-success text-xl font-normal [font-variant-numeric:tabular-nums] text-shadow-drop data-[zero=true]:text-warcraft-text-faint data-[zero=true]:font-normal mobile:text-xl",
);
