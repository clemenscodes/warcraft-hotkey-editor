use tw_macro::tw;
classes! {
    base: tw![
        "group",
        "flex",
        "items-baseline",
        "justify-between",
        "gap-[0.5rem]",
        "px-[0.55rem]",
        "py-[0.35rem]",
        "rounded-control",
        "text-[clamp(1.3rem,0.85rem+0.4vw,1.85rem)]/[1.25]",
        "text-shadow-drop",
        "min-w-0",
        "data-[matchup=strong]:bg-warcraft-success/12",
        "data-[matchup=weak]:bg-warcraft-danger/12",
    ],
    mobile: tw![
        "mobile:text-[1.7rem]",
        "mobile:px-[0.6rem]",
        "mobile:py-[0.4rem]",
    ],
}
