use tw_macro::tw;
// The override panel card: the gold-edged column that holds the header and the
// ability sections. Content-sized on desktop; a fixed-height scrollless block on the
// mobile panel. Class `.tile-override-card` is load-bearing (a scroll-into-view
// effect queries it).

classes! {
    base: tw![
        "flex",
        "flex-col",
        "items-stretch",
        "flex-[0_0_auto]",
        "gap-5",
        "p-[2rem_2.25rem]",
        "overflow-hidden",
        "border",
        "border-warcraft-gold",
        "rounded-panel",
        "bg-[linear-gradient(135deg,color-mix(in_oklab,var(--color-warcraft-gold-dark)_55%,transparent)_0%,color-mix(in_oklab,var(--color-warcraft-shadow)_55%,transparent)_100%)]",
        "shadow-[0_0_12px_color-mix(in_oklab,var(--color-warcraft-gold)_18%,transparent)]",
    ],
    mobile: tw![
        "mobile:w-full",
        "mobile:max-w-full",
        "mobile:min-w-0",
        "mobile:box-border",
        "mobile:flex-nowrap",
        "mobile:justify-start",
        "mobile:gap-[6px_10px]",
        "mobile:p-[10px_12px]",
        "mobile:h-[300px]",
    ],
    tablet: tw![
        "tablet:w-full",
        "tablet:max-w-full",
        "tablet:min-w-0",
        "tablet:box-border",
        "tablet:flex-nowrap",
        "tablet:justify-start",
        "tablet:gap-[6px_10px]",
        "tablet:p-[10px_12px]",
        "tablet:h-[300px]",
    ],
}
