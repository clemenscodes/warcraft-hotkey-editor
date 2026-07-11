use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "w-14",
        "h-14",
        "text-warcraft-gold",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
        "filter-[drop-shadow(0_0_10px_color-mix(in_oklab,var(--color-warcraft-gold)_45%,transparent))]",
    ],
}
