use tw_macro::tw;
classes! {
    base: tw![
        "inline-flex",
        "items-center",
        "justify-center",
        "w-[1.15em]",
        "h-[1.15em]",
        "text-rose-400/90",
        "drop-shadow-[0_0_0.3em_color-mix(in_oklab,var(--color-race-orc)_35%,transparent)]",
        "[&_svg]:block",
        "[&_svg]:w-full",
        "[&_svg]:h-full",
    ],
}
