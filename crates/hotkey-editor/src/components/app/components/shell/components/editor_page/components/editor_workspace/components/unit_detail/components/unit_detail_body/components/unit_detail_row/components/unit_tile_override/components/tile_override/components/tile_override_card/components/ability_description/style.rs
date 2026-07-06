use tw_macro::tw;
// The primary ubertip / tip text block for an ability or upgrade. A scrollable
// gold-edged panel on the sidebar; natural-height and smaller text on small screens.
// Each line is a `<p>` (pre-wrapped) so authored spacing survives.

classes! {
    base: tw![
        "flex-1",
        "min-h-0",
        "overflow-y-auto",
        "flex",
        "flex-col",
        "gap-[0.4rem]",
        "px-4",
        "py-[0.85rem]",
        "bg-warcraft-bg-base/35",
        "border-l-2",
        "border-warcraft-gold",
        "rounded-[0.25rem]",
        "text-warcraft-text-secondary",
        "text-[1.55rem]",
        "leading-[1.55]",
        "[&>p]:m-0",
        "[&>p]:whitespace-pre-wrap",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:overflow-visible",
        "mobile:max-h-none",
        "mobile:text-[13px]",
        "mobile:leading-[1.35]",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:overflow-visible",
        "tablet:max-h-none",
        "tablet:text-[13px]",
        "tablet:leading-[1.35]",
    ],
}
