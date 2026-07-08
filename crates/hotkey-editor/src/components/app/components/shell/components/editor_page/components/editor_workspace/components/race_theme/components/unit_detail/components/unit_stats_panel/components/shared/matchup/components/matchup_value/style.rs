use tw_macro::tw;
classes! {
    base: tw![
        "text-warcraft-text-secondary",
        "font-medium",
        "[font-variant-numeric:tabular-nums]",
        "flex-[0_0_auto]",
        "group-data-[matchup=strong]:text-warcraft-success",
        "group-data-[matchup=weak]:text-race-orc",
    ],
}
