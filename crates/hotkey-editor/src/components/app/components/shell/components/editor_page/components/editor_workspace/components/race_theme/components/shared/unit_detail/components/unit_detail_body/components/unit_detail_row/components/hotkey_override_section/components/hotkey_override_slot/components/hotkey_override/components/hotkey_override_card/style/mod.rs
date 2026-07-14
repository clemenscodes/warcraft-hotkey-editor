use tw_macro::tw;
// The hotkey-override card: the gold-edged column that holds the header and the
// ability sections. Content-sized on every band; on touch bands it keeps the same
// look at tighter density.

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-stretch",
        "flex-none",
        "gap-5",
        "py-8",
        "px-9",
        "overflow-hidden",
        "border",
        "border-warcraft-gold",
        "rounded-panel",
        "bg-panel-gold-resting",
        "shadow-glow-soft",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:max-w-full",
        "mobile:min-w-0",
        "mobile:box-border",
        "mobile:flex-nowrap",
        "mobile:justify-start",
        "mobile:gap-y-1.5",
        "mobile:gap-x-2.5",
        "mobile:py-2.5",
        "mobile:px-3",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:max-w-full",
        "tablet:min-w-0",
        "tablet:box-border",
        "tablet:flex-nowrap",
        "tablet:justify-start",
        "tablet:gap-y-1.5",
        "tablet:gap-x-2.5",
        "tablet:py-2.5",
        "tablet:px-3",
    ],
}
