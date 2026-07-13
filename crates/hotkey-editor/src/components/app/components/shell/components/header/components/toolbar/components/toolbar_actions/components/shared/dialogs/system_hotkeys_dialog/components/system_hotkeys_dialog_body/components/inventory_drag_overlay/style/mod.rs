use tw_macro::tw;
classes! {
    base: tw![
        "fixed",
        "pointer-events-none",
        "z-20",
        "flex",
        "items-center",
        "justify-center",
        "border-solid",
        "border-12",
        "select-none",
        "bg-panel-toast",
        "[border-image-source:var(--wc3-slot-frame)]",
        "[border-image-slice:12_fill]",
        "[border-image-repeat:stretch]",
        "filter-[drop-shadow(0_8px_24px_color-mix(in_oklab,var(--color-warcraft-shadow)_60%,transparent))_drop-shadow(0_0_16px_color-mix(in_oklab,var(--color-warcraft-gold)_60%,transparent))]",
    ],
    mobile: tw![
        "mobile:border-8",
    ],
    tablet: tw![
        "tablet:border-8",
    ],
}
