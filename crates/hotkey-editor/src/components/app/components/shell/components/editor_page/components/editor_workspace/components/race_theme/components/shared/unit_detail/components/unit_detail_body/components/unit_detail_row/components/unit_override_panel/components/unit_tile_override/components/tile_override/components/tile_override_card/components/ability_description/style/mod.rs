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
        "gap-1.5",
        "px-4",
        "py-3.5",
        "bg-warcraft-bg-base/35",
        "border-l-2",
        "border-warcraft-gold",
        "rounded-control",
        "text-warcraft-text-secondary",
        "text-xl",
        "leading-prose",
        "[&>p]:whitespace-pre-wrap",
    ],
    mobile: tw![
        "mobile:flex-none",
        "mobile:overflow-visible",
        "mobile:max-h-none",
        "mobile:text-xs",
        "mobile:leading-heading",
    ],
    tablet: tw![
        "tablet:flex-none",
        "tablet:overflow-visible",
        "tablet:max-h-none",
        "tablet:text-xs",
        "tablet:leading-heading",
    ],
}
