use tw_macro::tw;
classes! {
    base: tw![
        "flex",
        "items-baseline",
        "justify-between",
        "gap-[0.75em]",
        "px-[0.6em]",
        "py-[0.15em]",
        "rounded-control",
        "leading-heading",
        "text-shadow-drop",
        "min-w-0",
        "bg-warcraft-text-secondary/8",
        "[--matchup-color:var(--color-warcraft-text-secondary)]",
    ],
    mobile: tw![
        "mobile:gap-[3.16cqi]",
        "mobile:px-[4.43cqi]",
        "mobile:py-[1.58cqi]",
    ],
}
